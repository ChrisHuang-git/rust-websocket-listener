
use futures_util::{SinkExt, StreamExt};
use serde_json::Error;

//use std::sync::{Arc, Mutex};

use tokio::time::{sleep, Duration};
//use tokio::io::{AsyncRead, AsyncWrite};
use tokio_tungstenite::connect_async;
use tungstenite::Message;

use crate::example_message;
use crate::database::database::Db;
pub struct VehicleStream {
    url: String,
    database: Db,
}
impl VehicleStream {
    pub fn new(url: String, database: Db) -> Self {
        Self {
            url,
            database
        }
    }
    pub async fn get_messages(self) -> Result<(), Error> {
        // Connect to websocket URL
        let mut retry_counter: u32 = 0; 
        let ws_stream = loop {
            match connect_async(&self.url).await {
                Ok(stream,) => {
                    println!("WebSocket handshake has been successfully completed");
                    break stream.0; // returns stream part of tuple
                },
                Err(e) => { // retry in 2, 4, 8, 16, ... seconds
                    let sleep_time: u64 = u64::pow(2, retry_counter);
                    println!("Error {e}");
                    println!("Connection to url ({}) has failed. Retrying in {} seconds", &self.url, sleep_time);
                    retry_counter += 1;
                    sleep(Duration::from_secs(sleep_time)).await;
                    // TODO add failure after # of retries
                }
            }
        };
        // Split WebSocket Stream into sender and receiver
        let (mut _sender, mut receiver) = ws_stream.split();
        
        // Handle incoming messages
        while let Some(msg) = receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    // Todo flesh out message parsing and use save_message function instead
                    let example_message: example_message::ExampleMessage = serde_json::from_str(&text)?;

                    self.database.clone().add_example_message(example_message);
                    //Self::update_database(example_message, self.shared_database.clone());

                    // self.database.push(example_message);

                    // Reverse received string and send it back
                    // let reversed: String = text.chars().rev().collect::<String>();
                    // if let Err(e) = sender.send(Message::Text(reversed)).await {
                    //     println!("Error sending message: {}", e);
                    // }
                    // println!("In the message Text senario");
                }
                Ok(Message::Close(_)) => break,
                Ok(_) => {
                    println!("Got something else?");
                }
                Err(e) => {
                    println!("Error processing message: {}", e);
                    break;
                }
            }
        }
        return Ok(());
        
    }
}


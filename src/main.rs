//use futures_util::{future, pin_mut, StreamExt, SinkExt};
//use log::error;
use vehicle_stream::VehicleStream;
use std::env;
//use std::sync::{Arc, Mutex};

use tokio::time::{sleep, Duration};
//use tokio::sync::Mutex;
//use tokio::net::{TcpListener, TcpStream};
//use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

// #[path = "example_message/example_message.rs"] mod example_message;
// #[path = "vehicle_stream/vehicle_stream.rs"] mod vehicle_stream;

mod example_message;
mod vehicle_stream;
mod database;

use database::database::Db;

#[tokio::main]
async fn main() {
    let num_vehicles = 4;

    // let total_message_storage: Arc<Mutex<Vec<example_message::ExampleMessage>>> = Arc::new(Mutex::new(Vec::new()));
    let mut all_db_vec: Vec<Db> = Vec::new();
    if 1 <= num_vehicles {
        for n in 1..=num_vehicles {
            let n_string = n.to_string();
            let ip = format!("ws://127.0.0.{n_string}:8080");
            let url: String = env::args().nth(n).unwrap_or_else(|| ip);

            let database = Db::new();
            let vehicle = VehicleStream::new(url, database.clone());
            all_db_vec.push(database);
            tokio::spawn(async move {
                vehicle.get_messages().await
            });
        }
    }
    loop {
        sleep(Duration::from_millis(100)).await;
        
        let mut total_msg = 0;
        for db in all_db_vec.iter() {
            total_msg += db.clone().get_size();
        }
        println!("total received messages: {}", total_msg );
    }
}

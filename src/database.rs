pub mod database {
    use std::{str, sync::{Arc, Mutex, MutexGuard}};
    use crate::example_message::{self, ExampleMessage};

    #[derive(Clone)]
    pub struct Db {
        shared_database: Arc<Mutex<Vec<example_message::ExampleMessage>>>,
    }
    impl Db {
        pub fn new() -> Self {
            Self {
                shared_database: Arc::new(Mutex::new(Vec::new())),
            }
        }
        pub fn add_example_message(self, example_message: example_message::ExampleMessage) {
            let mut db = match self.shared_database.lock() {
                Ok(t) => {
                    t
                }
                Err(mut e) => {
                    **e.get_mut() = Vec::new();
                    self.shared_database.clear_poison();
                    e.into_inner()
                }
            };
            db.push(example_message);
        }
        pub fn get_size(self) -> usize {
            let mut db = match self.shared_database.lock() {
                Ok(t) => {
                    t
                }
                Err(mut e) => {
                    **e.get_mut() = Vec::new();
                    self.shared_database.clear_poison();
                    e.into_inner()
                }
            };
            return db.len();
        }
        pub fn retain_all_after_time(self, time:i64) {
            let mut db = match self.shared_database.lock() {
                Ok(t) => {
                    t
                }
                Err(mut e) => {
                    **e.get_mut() = Vec::new();
                    self.shared_database.clear_poison();
                    e.into_inner()
                }
            };
            db.retain(|&x| x.is_after_time(time));

        }
        // fn apply_to_db(self, ) -> MutexGuard<'static, Vec<ExampleMessage>> {
        //     let mut db = match db_arc.lock() {
        //         Ok(t) => {
        //             t
        //         }
        //         Err(mut e) => {
        //             **e.get_mut() = Vec::new();
        //             db_arc.clear_poison();
        //             e.into_inner()
        //         }
        //     };
        //     return db;
        // }
    }
}

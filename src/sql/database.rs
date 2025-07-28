use sqlx;
use futures::executor::block_on;
use std::thread;
use crate::sql::Table;


pub struct Database {
    pool: sqlx::Pool<sqlx::Any>,
    tables: Vec<Table>
}


impl Database {
    pub fn connect(url: String) -> Database {
        // Connect to the database on a new thread
        let pool = thread::spawn(move || {
             block_on(sqlx::Pool::<sqlx::Any>::connect(&url))
                 .expect("Unable to connect to database")
        })
            .join()
            .expect("Thread crashed while connecting to database");
        
        Database {
            pool,
            tables: Vec::new()
        }
    }
    
    pub fn query() { todo!() }
    pub fn insert() { todo!() }
    pub fn migrate() { todo!() }
}
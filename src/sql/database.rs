use sqlx;
use futures::executor::block_on;
use std::thread;
use sqlx::{Error, Executor};
use sqlx::mysql::MySqlQueryResult;
use sqlx::postgres::PgQueryResult;
use sqlx::sqlite::SqliteQueryResult;
use crate::sql::Table;
use sqlx::FromRow;


#[cfg(any(
    all(feature = "database_mysql", feature = "database_postgres"),
    all(feature = "database_mysql", feature = "database_sqlite"),
    all(feature = "database_postgres", feature = "database_sqlite")
))]
compile_error!("Only one database driver may be enabled!");


#[cfg(feature = "database_mysql")]
pub struct Database
{
    pool: sqlx::Pool<sqlx::MySql>,
    tables: Vec<Table>
}


#[cfg(feature = "database_mysql")]
impl Database {
    pub fn connect(url: &'static str) -> Database {
        // Connect to the database on a new thread
        let pool = thread::spawn(move || {
            sqlx::any::install_default_drivers();
            block_on(
                sqlx::Pool::<sqlx::MySql>::connect(url)
            )
                .expect("Unable to connect to database")
        })
            .join()
            .expect("Thread crashed while connecting to database");

        Database { pool, tables: Vec::new() }
    }
}

#[cfg(feature = "database_postgres")]
pub struct Database
{
    pool: sqlx::Pool<sqlx::Postgres>,
    tables: Vec<Table>
}


#[cfg(feature = "database_postgres")]
impl Database {
    pub fn connect(url: &'static str) -> Database {
        // Connect to the database on a new thread
        let pool = thread::spawn(move || {
            sqlx::any::install_default_drivers();
            block_on(
                sqlx::Pool::<sqlx::Postgres>::connect(url)
            )
                .expect("Unable to connect to database")
        })
            .join()
            .expect("Thread crashed while connecting to database");

        Database { pool, tables: Vec::new() }
    }
}

#[cfg(feature = "database_sqlite")]
pub struct Database
{
    pool: sqlx::Pool<sqlx::Sqlite>,
    tables: Vec<Table>
}


#[cfg(feature = "database_sqlite")]
impl Database {
    pub fn connect(url: &'static str) -> Database {
        // Connect to the database on a new thread
        let pool = thread::spawn(move || {
            sqlx::any::install_default_drivers();
            block_on(
                sqlx::Pool::<sqlx::Sqlite>::connect(url)
            )
                .expect("Unable to connect to database")
        })
            .join()
            .expect("Thread crashed while connecting to database");

        Database { pool, tables: Vec::new() }
    }
}

#[cfg(feature = "database_mysql")]
type QueryResult = MySqlQueryResult;

#[cfg(feature = "database_postgres")]
type QueryResult = PgQueryResult;

#[cfg(feature = "database_sqlite")]
type QueryResult = SqliteQueryResult;


#[cfg(any(
    feature = "database_mysql",
    feature = "database_postgres",
    feature = "database_sqlite"
))]
impl Database {
    pub fn query() { todo!() }
    pub fn insert() { todo!() }
    
    pub fn execute(&mut self, query: String) -> Result<QueryResult, Error> {
        let pool = self.pool.clone();
        thread::spawn(move || {
           block_on(
               pool.execute(query.as_str())
           )
        })
            .join()
            .expect("Thread crashed while executing query")
            
    }
    
    pub fn clear_database(&self) {
        let pool = self.pool.clone();
        thread::spawn(move || {
            
        })
            .join()
            .expect("Unable to clear database");
    }
    
    pub fn migrate(&self) {
        self.clear_database();
        for table in &self.tables {
            let pool = self.pool.clone();
            
            let query = format!(
                "CREATE TABLE {table_name}({parameters})",
                table_name=table.name(),
                parameters={
                    let mut string = String::new();
                    for column in table.columns() {
                        string.push_str(&format!(
                            "{name} {sql_type} {null}\n",
                            name=&column.name,
                            sql_type=column.sql_type,
                            null=if column.null { "" } else { "not null" }
                        ))
                    }
                    string
                }
            );
            
            println!("Executing query \"{query}\"");
            
            block_on(
                pool.execute(query.as_str())
            ).expect(&format!("Unable to migrate table {}", table.name()));
        }
    }

    /// Add a table to the database
    pub fn add_table<T>(&mut self)
    where
        T: crate::sql::SQLTable
    {
        // Clone the pool
        let pool = self.pool.clone();

        // Spawn a new thread
        match thread::spawn(move || {
            // Get the query
            let binding = T::table().add_string();
            let query = binding.as_str();

            // Execute the query
            block_on(
                pool.execute(query)
            )
        })
            .join()
            .expect("Thread crashed unexpectedly while adding a table")
        {
            Ok(_) => println!("Table {} was successfully added", T::table().name),
            Err(e) => eprintln!("Error while adding table {}: {}", T::table().name, e)
        }
    }


    /// Get every row from the given table, and parse it into Rust structs
    /// ## Example
    /// ```rust
    /// use aerielle::*;
    ///
    /// #[table]
    /// struct User {
    ///     #[primary_key]
    ///     id: Integer,
    ///
    ///     #[sql_name = "username"]
    ///     name: String
    /// }
    ///
    /// fn main() {
    ///     let mut db: sql::Database = sql::Database::connect(url)
    ///     let users: Vec<User> = db.get_table::<User>();
    /// }
    /// ```
    pub fn get_table<T>(&mut self) -> Vec<T>
    where
        for <'r> T: crate::sql::SQLTable + sqlx::FromRow<'r, crate::SQLRow>
    {
        // Clone the pool for further use
        let pool = self.pool.clone();

        // Create a new thread
        let handle = thread::spawn(move || {
            // Query all the rows from the table
            let query = format!("SELECT * FROM {}", T::table().name);
            block_on(
                sqlx::query(&query).fetch_all(&pool)
            )
        })
            .join()
            .expect("Thread crashed while querying every row from the table");

        // Test if the query was successful
        match handle {
            // If the query was successful, parse the rows into Rust structs
            Ok(v) => {
                let mut vector = Vec::with_capacity(v.len());
                for row in &v {
                    match T::from_row(row) {
                        Ok(t) => vector.push(t),
                        Err(e) => eprintln!("Error while reading in row: {e}")
                    }
                }
                vector
            },

            // If the query was unsuccessful, print an error and return an empty vector
            Err(e) => {
                eprintln!("Unable to query rows from table {}: {e}", T::table().name);
                Vec::new()
            }
        }
    }

    pub fn filter_table<T>(&mut self) -> Result<(), Error>
    where
        T: crate::sql::SQLTable
    {
        todo!()
    }

    pub fn add_to_table<T>(&mut self, instance: T) -> Result<(), Error>
    where
        T: crate::sql::SQLTable
    {
        todo!()
    }
}
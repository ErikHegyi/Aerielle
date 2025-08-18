mod database;
mod table;
mod sql_types;
mod column;

pub use database::Database;
pub use table::{Table, SQLTable};
pub use column::Column;
pub use sql_types::SQLType;
pub use sql_types::SQLValue;

#[cfg(feature = "database_mysql")]
pub type SQLRow = sqlx::mysql::MySqlRow;

#[cfg(feature = "database_postgres")]
pub type SQLRow = sqlx::postgres::PgRow;

#[cfg(feature = "database_sqlite")]
pub type SQLRow = sqlx::sqlite::SqliteRow;

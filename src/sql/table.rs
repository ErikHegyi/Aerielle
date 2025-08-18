use sqlx::Executor;
use crate::sql::{Column, Database, Row};


#[derive(Debug)]
pub struct Table {
    pub name: String,
    pub primary_key: Column,
    pub columns: Vec<Column>
}


impl Table {
    pub fn new(name: String, primary_key: Column, columns: Vec<Column>) -> Self {
        Self { name, primary_key, columns }
    }
    pub fn query(&self, database: &mut Database) -> Vec<Row>
    {
        todo!()
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn columns(&self) -> Vec<Column> {
        let mut vec = Vec::with_capacity(self.columns.len() + 1);
        vec.push(self.primary_key.clone());
        for column in self.columns.clone() {
            vec.push(column)
        }
        vec
    }

    /// The table creation string for the current table instance
    /// ## Example
    /// The user created a table:
    /// ```rust
    /// #[table]
    /// struct UserTable {
    ///     #[primary_key]
    ///     id: Integer,
    ///
    ///     #[not_null]
    ///     username: Text
    /// }
    /// ```
    ///
    /// Aerielle will automatically translate this, and create a table addition string.
    /// For `MySQL`, this string would be:
    /// ```sql
    /// CREATE TABLE usertable(
    ///     id INTEGER PRIMARY KEY,
    ///     username VARCHAR NOT NULL
    /// )
    /// ```
    pub fn add_string(&self) -> String {
        format!(
            "CREATE TABLE {table_name}({columns})",
            table_name=self.name,
            columns=self.columns()
                .iter()
                .map(|c| c.sql_string())
                .collect::<Vec<String>>()
                .join(",")
        )
    }
}


pub trait SQLTable {
    fn table() -> Table;
}
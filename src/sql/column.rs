use crate::sql::sql_types::{SQLType, SQLValue};


#[derive(Debug)]
pub struct Column {
    // Attributes
    pub pk: bool,
    pub unique: bool,
    pub auto_increment: bool,
    pub default: SQLValue,
    pub null: bool,
    pub fk: bool,
    pub m2m: bool,

    pub name: String,
    pub sql_name: String,
    pub sql_type: SQLType
}
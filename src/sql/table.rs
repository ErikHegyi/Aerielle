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
    
    pub fn columns(&self) -> &Vec<Column> {
        &self.columns
    }
}

/*
#[macro_export]
macro_rules! table {
    (
        $name:ident ($pk: ident) {
            $($column:ident: $sql_type:ident $(($arg:expr))?),+
        }
    ) => {
        use crate::sql::{Column, SQLType, Table};
        {
            let mut columns = Vec::new();
            let mut pk: Column = Column {
                name: "id".to_string(),
                sql_type: SQLType::Integer,
                null: false
            };
            
            $(
                if stringify!($column) == stringify!($pk) {
                    pk = Column {
                        name: stringify!($column).to_string(),
                        sql_type: SQLType::$sql_type$( ($arg) )?,
                        null: false
                    };
                } else {
                    columns.push(
                        Column {
                            name: stringify!($column).to_string(),
                            sql_type: SQLType::$sql_type$( ($arg) )?,
                            null: false
                        }
                    )
                }
            )*
            Table::new(
                stringify!($name).to_string(),
                pk,
                columns
            )
        }
    };
}*/
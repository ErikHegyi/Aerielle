use crate::sql::sql_types::{SQLType, SQLValue};


#[derive(Debug, Clone)]
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


impl Column {
    #[cfg(feature="database_sqlite")]
    pub fn sql_string(&self) -> String {
        format!(
            "{sql_name} {sql_type}{unique}{primary_key}{auto_increment}{null}{default}",
            sql_name=self.sql_name,
            sql_type=self.sql_type,
            unique=if self.unique { " UNIQUE" } else { "" },
            primary_key=if self.pk { " PRIMARY KEY" } else { "" },
            auto_increment=if self.auto_increment { " AUTOINCREMENT" } else { "" },
            null=if !self.null { " NOT NULL" } else { "" },
            default=if !self.null { format!(" DEFAULT {}", self.default) } else { String::new() }
        )
    }

    #[cfg(feature="database_mysql")]
    pub fn sql_string(&self) -> String {
        format!(
            "{sql_name} {sql_type}{auto_increment}{unique}{primary_key}{null}{default}",
            sql_name=self.sql_name,
            sql_type=self.sql_type,
            auto_increment=if self.auto_increment { " AUTO_INCREMENT" } else { "" },
            unique=if self.unique { " UNIQUE" } else { "" },
            primary_key=if self.pk { " PRIMARY KEY" } else { "" },
            null=if !self.null { " NOT NULL" } else { "" },
            default=if !self.null { format!(" DEFAULT {}", self.default) } else { String::new() }
        )
    }

    #[cfg(feature="database_postgres")]
    pub fn sql_string(&self) -> String {
        let ty = if self.auto_increment && self.pk {
            match self.sql_type {
                SQLType::Integer => "SERIAL".to_string(),
                _ => format!("{}", self.sql_type)
            }
        } else {
            format!("{}", self.sql_type)
        };

        format!(
            "{sql_name} {sql_type}{unique}{primary_key}{null}{default}",
            sql_name=self.sql_name,
            sql_type=ty,
            unique=if self.unique { " UNIQUE" } else { "" },
            primary_key=if self.pk { " PRIMARY KEY" } else { "" },
            null=if !self.null { " NOT NULL" } else { "" },
            default=if !self.null { format!(" DEFAULT {}", self.default) } else { String::new() }
        )
    }
}
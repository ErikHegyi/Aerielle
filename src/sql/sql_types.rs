pub enum RawSQLType {
    Char(usize),
    Varchar(usize),
    Binary(usize),
    Varbinary(usize),
    TinyBlob,  // Max length: 255
    TinyText,  // Max length: 255
    Text(usize),  // Max length: 65 535
    Blob(usize),  // Max length: 65 535
    MediumText,
    MediumBlob,
    LongText,
    LongBlob,

    Bit(usize),  // Max value - 64
    TinyInt(usize),  // 0 - 255 / -128 - 127
    Boolean,
    SmallInt(usize),  // 0 - 65 535 / -32 768 - 32 767
    MediumInt(usize),
    Int(usize),
    Bigint,
    Float(usize),
    Decimal(usize),

    Date,
    Datetime,
    TimeStamp,
    // TODO: TimeStampTZ(),
    Time,
    Year
}


#[derive(Debug, Clone)]
pub enum SQLType {
    Text,
    Blob,
    Boolean,
    Bit,
    Integer,
    Float,
    TimeStamp,
    Date,
    Time
}

#[derive(Debug, Clone)]
pub enum SQLValue {
    Text(String),
    Blob(Vec<u8>),
    Boolean(bool),
    Bit(bool),
    Integer(i64),
    Float(f64),
    TimeStamp(u128),
    Date(u128),
    Time(u128),
    Null
}


impl std::fmt::Display for SQLType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match *self {
            Self::Text => "VARCHAR".to_string(),
            Self::Blob => "BLOB".to_string(),
            Self::Boolean => "BOOLEAN".to_string(),
            Self::Bit => "BIT".to_string(),
            Self::Integer => "INTEGER".to_string(),
            Self::Float => "FLOAT".to_string(),
            Self::Date => "DATE".to_string(),
            Self::TimeStamp => "TIMESTAMP".to_string(),
            Self::Time => "TIME".to_string(),
        })
    }
}


impl std::fmt::Display for SQLValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Text(v) => format!("\"{v}\""),
            Self::Blob(v) => format!("{v:?}"),
            Self::Boolean(v) => format!("\"{}\"", if *v { "TRUE" } else { "FALSE" }),
            Self::Bit(v) => format!("{}", if *v { "1" } else { "0" }),
            Self::Integer(v) => format!("{v}"),
            Self::Float(v) => format!("{v}"),
            Self::Date(v) => format!("{v}"),
            Self::TimeStamp(v) => format!("{v}"),
            Self::Time(v) => format!("{v}"),
            Self::Null => String::from("NULL")
        })
    }
}
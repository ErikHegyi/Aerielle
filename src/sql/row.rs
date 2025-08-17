pub enum RowValue {
    String(String),
    Bytes(Vec<u8>),
    Number(i128),
    Float(f64),
    Boolean(bool)
}


pub struct Row {
    pub values: Vec<RowValue>
}
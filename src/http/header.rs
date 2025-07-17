#[derive(Clone, Debug)]
pub struct Header {
    key: String,
    value: String
}


impl Header {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
    
    pub fn key(&self) -> &String {
        &self.key
    }
    
    pub fn value(&self) -> &String {
        &self.value
    }
}


impl From<String> for Header {
    fn from(value: String) -> Self {
        let index: usize = match value.find(": ") {
            Some(i) => i,
            None => panic!("Unable to parse header: {value}")
        };
        let (key, value) = value.split_at(index);
        
        Header {
            key: key.to_string(),
            value: value.to_string()
        }
    }
}


#[macro_export]
macro_rules! header {
    ($key: tt : $value: tt) => {
        Header::new($key.to_string(), $value.to_string())
    };
}
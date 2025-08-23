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
        match value.is_empty() {
            // If the string is empty, return an empty header
            true => Self {
                key: String::new(),
                value: String::new()
            },

            // If the string is not empty, parse it
            false => {
                // Split the header into a key and a value
                let index: usize = match value.find(": ") {
                    Some(i) => i,
                    None => panic!("Unable to parse header: \"{value}\"")
                };
                let (key, value) = value.trim().split_at(index);

                Self {
                    key: key.to_string(),
                    value: value[2..].to_string()
                }
            }
        }
    }
}


#[macro_export]
macro_rules! header {
    ($key: tt : $value: tt) => {
        Header::new($key.to_string(), $value.to_string())
    };
}
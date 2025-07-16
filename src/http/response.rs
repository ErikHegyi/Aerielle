use crate::http::{Header, Status};

pub struct Response {
    pub status: Status,
    pub headers: Vec<Header>,
    pub body: String
}


impl Response {
    pub fn new(status: Status, body: String) -> Self {
        Self {
            status,
            headers: vec![Header::new(
                "Content-Length".to_string(),
                body.len().to_string()
            )],
            body
        }
    }
    
    pub fn add_header(&mut self, header: Header) {
        self.headers.push(header);
    }
}
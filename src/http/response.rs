use std::{
    fs,
    path::PathBuf,
    io::ErrorKind
};
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
    
    pub fn server_error() -> Self {
        Self::new(
            Status::InternalServerError,
            String::new()
        )
    }
    
    pub fn not_found() -> Self {
        Self::new(
            Status::InternalServerError,
            String::new()
        )
    }
    
    pub fn read_in(path: PathBuf) -> Self {
        // Get the type of static file returned
        let content_type: Header = match path.extension() {
            Some(ext) => match ext.to_str() {
                Some(ext) => match ext {
                    "css" => Header::new("Content-Type".to_string(), "text/css".to_string()),
                    "js" | "ts" => Header::new("Content-Type".to_string(), "application/javascript".to_string()),
                    _ => Header::new("Content-Type".to_string(), "text/html".to_string())
                },
                None => Header::new("Content-Type".to_string(), "text/html".to_string())
            },
            None => Header::new("Content-Type".to_string(), "text/html".to_string())
        };
        
        // Read in the file
        let content: String = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => return Self::new(Status::NotFound, String::new()),
                _ => panic!("Reading in file failed: {e}")
            }
        };
        
        // Return
       Self {
            status: Status::OK,
            headers: vec![
                content_type,
                Header::new("Content-Length".to_string(), content.len().to_string())
            ],
            body: content
        }
    }
}
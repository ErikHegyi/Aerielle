use std::{
    io::Result,
    net::{TcpListener, TcpStream},
    path::PathBuf
};
use crate::http::{
    Request,
    Response
};
use regex::Regex;


pub struct WebServer {
    /* SERVER DATA */
    ip: String,
    port: u16,
    
    /* STATIC */
    static_url: Option<String>,
    static_dir: Option<PathBuf>,
    
    /* MAP URLs TO FUNCTIONS */
    url_map: Vec<Box<(Regex, dyn Fn(Request) -> Response)>>
}


impl WebServer {
    pub fn new() -> Self {
        WebServer::default()
    }

    /* SET ATTRIBUTES */
    pub fn set_ip(&mut self, ip: impl ToString) {
        self.ip = ip.to_string()
    }
    
    pub fn set_port(&mut self, port: u16) {
        self.port = port
    }
    
    pub fn set_static_url(&mut self, url: impl ToString) {
        self.static_url = Some(url.to_string())
    }
    
    pub fn set_static_dir(&mut self, dir: PathBuf) {
        self.static_dir = Some(dir)
    }
    
    pub fn disable_static(&mut self) { 
        self.static_url = None;
        self.static_dir = None;
    }
    
    /* HANDLE REQUESTS */
    fn handle(&self, request: &Request) -> Response {
        todo!()
    }
    
    /* START SERVER */
    pub fn start(&self) -> Result<()> {
        // Start the TCP listener
        let url: String = format!("{ip}:{port}", ip=self.ip, port=self.port);
        let listener = TcpListener::bind(url)?;
        
        // Listen to incoming requests
        for stream in listener.incoming() {
            println!("Incoming stream...");
            
            let stream = stream?;
            
            // Interpret the request
            let request: Request = Request::from(stream);
            
            // Handle the request
            let response: Response = self.handle(&request);
            
            // Write the response
            request.respond(response)?;
        }
        
        Ok(())
    }
}


impl Default for WebServer {
    fn default() -> Self {
        WebServer {
            ip: "localhost".to_string(),
            port: 8000,
            static_url: Some("/static".to_string()),
            static_dir: None,
            url_map: Vec::new()
        }
    }
}
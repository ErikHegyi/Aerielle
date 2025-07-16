use std::{
    io::Result,
    net::TcpListener,
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
    url_map: Vec<(Regex, Box<dyn Fn(&Request) -> Response>)>
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

    pub fn add_path(&mut self, pattern: &str, function: impl Fn(&Request) -> Response + 'static) {
        let re: Regex = Regex::new(&format!("^{pattern}$")).unwrap();
        self.url_map.push(
            (re, Box::new(function))
        )
    }
    
    /* ACCESS PROPERTIES */
    pub fn static_enabled(&self) -> bool {
        self.static_dir != None
    }

    /* HANDLE REQUESTS */
    fn serve_static(path: &str) -> Response {
        todo!()
    }


    fn handle(&self, request: &Request) -> Response {
        // Get the URL of the request
        let url: &str = request.url.as_str();

        if let Some(static_url) = &self.static_url {
            if url.starts_with(static_url) {
                // Test if static files should be served
                if self.static_enabled() {
                    return Self::serve_static(url);
                } else {
                    panic!("Static files are disabled, but a static file is expected: {url}")
                }
            }
        }

        // Match the URL to the given patterns
        for (pattern, function) in self.url_map.iter() {
            if pattern.is_match(url) {
                return function(request);
            }
        }

        panic!("Unable to match url ({url}) to any of the patterns.")
    }

    /* START SERVER */
    pub fn start(&self) -> Result<()> {
        // Start the TCP listener
        let url: String = format!("{ip}:{port}", ip=self.ip, port=self.port);
        let listener = TcpListener::bind(url)?;

        // Listen to incoming requests
        for stream in listener.incoming() {
            let stream = stream?;

            // Interpret the request
            let mut request: Request = Request::from(stream);
            println!("{request}");

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
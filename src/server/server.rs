use std::{
    io::Result,
    net::TcpListener,
    path::PathBuf
};
use crate::http::{
    Request,
    Response,
    Status
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
    url_map: Vec<(Regex, Box<dyn Fn(&Request) -> Response>)>,
    
    /* ERROR FUNCTIONS */
    server_error: Box<dyn Fn(&Request) -> Response>,
    not_found_error: Box<dyn Fn(&Request) -> Response>
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
    
    pub fn set_server_error(&mut self, function: impl Fn(&Request) -> Response + 'static) {
        self.server_error = Box::new(function);
    }
    
    pub fn set_not_found_error(&mut self, function: impl Fn(&Request) -> Response + 'static) {
        self.not_found_error = Box::new(function);
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
                let response: Response = function(request);
                return if response.status == Status::InternalServerError {
                    (self.server_error)(request)
                } else if response.status == Status::NotFound {
                    (self.not_found_error)(request)
                } else {
                    function(request)
                }
            }
        }
        
        // If the pattern was not found, return a 404 error
        (self.not_found_error)(request)
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
    
    /* BUILT-IN RESPONSES */
    pub fn server_error(_: &Request) -> Response {
        Response::new(
            Status::InternalServerError,
            String::from("<h1>500 Server Error</h1>")
        )
    }
    
    pub fn not_found(_: &Request) -> Response {
        Response::new(
            Status::NotFound,
            String::from("<h1>404 Not Found</h1>")
        )
    }
}


impl Default for WebServer {
    fn default() -> Self {
        WebServer {
            ip: "localhost".to_string(),
            port: 8000,
            static_url: Some("/static".to_string()),
            static_dir: None,
            url_map: Vec::new(),
            server_error: Box::new(WebServer::server_error),
            not_found_error: Box::new(WebServer::not_found)
        }
    }
}
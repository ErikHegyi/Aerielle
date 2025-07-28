use std::{
    io::{Result, ErrorKind},
    fs::read_dir,
    net::{TcpListener, UdpSocket},
    path::PathBuf,
    env::current_dir,
    fs::read_to_string
};
use crate::{
    http::{Request, Response, Status},
    html::render
};
use regex::Regex;
use minijinja as jinja;
use crate::sql::Database;

pub struct WebServer {
    /* SERVER DATA */
    ip: String,
    port: u16,

    /* STATIC */
    static_url: Option<String>,
    static_dir: Option<PathBuf>,
    templates: PathBuf,

    /* MAP URLs TO FUNCTIONS */
    url_map: Vec<(Regex, Box<dyn Fn(&Self, &Request) -> Response>)>,
    
    /* ERROR FUNCTIONS */
    server_error: Box<dyn Fn(&Request) -> Response>,
    not_found_error: Box<dyn Fn(&Request) -> Response>,
    
    /* TEMPLATE RENDERING */
    environment: jinja::Environment<'static>,
    
    /* DATABASE */
    database: Option<Database>
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
    
    pub fn set_templates_folder(&mut self, templates_folder: PathBuf) {
        self.templates = templates_folder;
    }

    pub fn add_path(&mut self, pattern: &str, function: impl Fn(&Self, &Request) -> Response + 'static) {
        let re: Regex = Regex::new(&format!("^{pattern}$")).unwrap();
        self.url_map.push(
            (re, Box::new(function))
        )
    }
    
    pub fn read_in_templates(&mut self) {
        let files = match read_dir(self.templates.as_path()) {
            Ok(f) => f,
            Err(e) => panic!("Unable to read in templates folder: {e}")
        };
        
        for file in files {
            let file = file.unwrap();
            let name = file.file_name().display().to_string();
            self.add_template(name);
        }
    }
    
    pub fn add_template(&mut self, name: String) {
        let body = match read_to_string(self.templates.join(&name)) {
            Ok(s) => s,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => panic!("Unable to find file \"{name}\" in templates folder \"{folder}\"", folder=self.templates.display()),
                _ => panic!("Something went wrong while reading in \"{name}\": {e}")
            }
        };
        match self.environment
            .add_template_owned(name.clone(), body) {
            Ok(_) => (),
            Err(e) => panic!("Unable to add template {name}: {e}")
        }
    }
    
    pub fn set_server_error(&mut self, function: impl Fn(&Request) -> Response + 'static) {
        self.server_error = Box::new(function);
    }
    
    pub fn set_not_found_error(&mut self, function: impl Fn(&Request) -> Response + 'static) {
        self.not_found_error = Box::new(function);
    }
    
    pub fn connect_to_database(&mut self, url: &str) {
        self.database = Some(Database::connect(url.to_string()))
    }
    
    /* ACCESS PROPERTIES */
    pub fn static_enabled(&self) -> bool {
        self.static_dir != None && self.static_url != None
    }
    
    pub fn get_environment(&self) -> &jinja::Environment<'static> {
        &self.environment
    }
    
    pub fn get_database(&self) -> &Database {
        if let Some(db) = &self.database { db }
        else { panic!("Tried to access database, but no database was set.") }
    }

    /* HANDLE REQUESTS */
    fn serve_static(&self, request: &Request, url: &str) -> Response {
        if let Some(static_url) = &self.static_url {
            // Trim the string
            let mut trimmed: String = url
                .trim_start_matches(static_url)
                .replace('/', "\\");
            
            if trimmed.starts_with('/') {
                trimmed = trimmed[1..].to_string();
            }
            
            // Join the path
            if let Some(static_path) = &self.static_dir {
                let path: PathBuf = static_path.join(&trimmed);
                
                // Read in the file
                let response: Response = Response::read_in(path);
                return if response.status == Status::InternalServerError {
                    (self.server_error)(request)
                } else if response.status == Status::NotFound {
                    (self.not_found_error)(request)
                } else {
                    response
                }
            }
        }
        panic!("Static files are disabled, but a static file is expected: {url}");
    }
    
    pub fn render(&self, template: &str, context: jinja::Value) -> Response {
        render(self, template, context)
    }

    fn handle(&self, request: &Request) -> Response {
        // Get the URL of the request
        let url: &str = request.url.as_str();

        if let Some(static_url) = &self.static_url {
            if url.starts_with(static_url) {
                // Test if static files should be served
                if self.static_enabled() {
                    return self.serve_static(request, url);
                } else {
                    panic!("Static files are disabled, but a static file is expected: {url}")
                }
            }
        }

        // Match the URL to the given patterns
        for (pattern, function) in self.url_map.iter() {
            if pattern.is_match(url) {
                let response: Response = function(&self, request);
                return if response.status == Status::InternalServerError {
                    (self.server_error)(request)
                } else if response.status == Status::NotFound {
                    (self.not_found_error)(request)
                } else {
                    function(&self, request)
                }
            }
        }
        
        // If the pattern was not found, return a 404 error
        (self.not_found_error)(request)
    }

    /* START SERVER */
    pub fn start(&mut self) -> Result<()> {
        // Read in templates
        self.read_in_templates();
        
        // Start the TCP listener
        let url: String = format!("{ip}:{port}", ip=self.ip, port=self.port);
        let listener = TcpListener::bind(url)?;
        
        println!("Listening for requests on {ip}:{port}...", ip=self.ip, port=self.port);
        
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
    
    /* UTILITY FUNCTIONS */
    pub fn local_ip_address() -> Result<String> {
        let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0")?;
        socket.connect("8.8.8.8:80")?;
        Ok(
            socket.local_addr()?
                .to_string()
                .split(':')
                .nth(0)
                .unwrap()
                .to_string()
        )
    }
}


impl Default for WebServer {
    fn default() -> Self {
        WebServer {
            ip: "localhost".to_string(),
            port: 8000,
            static_url: Some("/static".to_string()),
            static_dir: Some(PathBuf::from("static")),
            url_map: Vec::new(),
            server_error: Box::new(WebServer::server_error),
            not_found_error: Box::new(WebServer::not_found),
            templates: current_dir().unwrap().parent().unwrap().join("templates"),
            environment: jinja::Environment::new(),
            database: None
        }
    }
}
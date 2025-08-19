use std::{
    io::{Result, ErrorKind},
    fs::read_dir,
    net::{TcpListener, UdpSocket},
    path::PathBuf,
    env::current_dir,
    fs::read_to_string,
    result::Result as StdResult
};
use std::fmt::Debug;
use std::path::Path;
use crate::{
    http::{Request, Response, Status},
    html::render
};
use regex::Regex;
use minijinja as jinja;

#[cfg(feature = "_db_must")]
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
    #[cfg(feature = "_db_must")]
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
        let templates = Self::list_items_in_dir(self.templates.as_path());
        let templates_dir_name = Self::dir_name(self.templates.as_path());

        for template in templates {
            let prefix = format!("{}/", templates_dir_name);
            let name_without_prefix = match template.strip_prefix(prefix.as_str()) {
                Some(string) => string.to_string(),
                None => panic!("Unable to remove the templates folder prefix from the total path")
            };

            let path = self.templates.join(&name_without_prefix.replace('/', "\\"));

            self.add_template_named(
                path,
                name_without_prefix
            )
        }
    }

    /// # Add template
    /// Adds a template to the WebServer's Jinja environment.
    /// Useful in case the user wishes to add templates from outside the `templates` folder.
    /// ## Parameters
    /// - `path: P` -> The path to the HTML file
    /// ## Return
    /// This method does not return anything.
    /// ## Panicking
    /// The method panics, if:
    /// - the contents of the file can not be read
    /// - the name of the file can not be read
    /// - the name of the file can not be converted from `&OsStr` to `&str`
    /// - the template contains a syntax error, and `minijinja` can not read it
    /// ## Example
    /// ```rust
    /// use std::path::PathBuf;
    /// use aerielle::*;
    ///
    /// fn main() {
    ///     // Define the path
    ///     let path = "C:\\User\\Path\\To\\The\\Template.html";
    ///
    ///     // Create the server
    ///     let mut server = WebServer::new();
    ///
    ///     // Add the path to the server
    ///     server.add_template(path);  // The template has now been added under the name "Template.html"
    ///
    ///     // Start responding to requests
    ///     server.start().unwrap();
    /// }
    /// ```
    pub fn add_template<P>(&mut self, path: P)
    where
        PathBuf: From<P>
    {
        let path = PathBuf::from(path);
        let body = Self::read_file(&path);

        let name = match path.file_name() {
            Some(file_name) => match file_name.to_str() {
                Some(string) => string.to_string(),
                None => panic!("Unable to convert file name of {path:?} from &OsStr to &str")
            },
            None => panic!("Unable to read the file name of {path:?}")
        };

        match self.environment
            .add_template_owned(name, body) {
            Ok(_) => (),
            Err(e) => panic!("Unable to add template {path:?}: {e}")
        }
    }

    /// # Add a template with a custom name
    /// Adds a template to the WebServer's Jinja environment with a name.
    /// Useful in case the user wishes to add templates from outside the `templates` folder,
    /// and also wishes to name it.
    /// ## Parameters
    /// - `path: P` -> The path to the HTML file
    /// - `name: String` -> The name of the template
    /// ## Return
    /// This method does not return anything.
    /// ## Panicking
    /// The method panics, if:
    /// - the contents of the file can not be read
    /// - the name of the file can not be read
    /// - the name of the file can not be converted from `&OsStr` to `&str`
    /// - the template contains a syntax error, and `minijinja` can not read it
    /// ## Example
    /// ```rust
    /// use std::path::PathBuf;
    /// use aerielle::*;
    ///
    /// fn main() {
    ///     // Define the path
    ///     let path = "C:\\User\\Path\\To\\The\\Template.html";
    ///
    ///     // Define the name
    ///     let name = String::from("MyTemplate.html");
    ///
    ///     // Create the server
    ///     let mut server = WebServer::new();
    ///
    ///     // Add the path to the server
    ///     server.add_template_named(path, name);  // The template has now been added under the name "MyTemplate.html"
    ///
    ///     // Start responding to requests
    ///     server.start().unwrap();
    /// }
    /// ```
    pub fn add_template_named<P>(&mut self, path: P, name: String)
    where
        PathBuf: From<P>
    {
        let path = PathBuf::from(path);

        let body = Self::read_file(&path);

        match self.environment
            .add_template_owned(name, body) {
            Ok(_) => (),
            Err(e) => panic!("Unable to add template {path:?}: {e}")
        }
    }
    
    pub fn get_template(&self, name: &str) -> StdResult<jinja::Template, jinja::Error> {
        self.get_environment().get_template(name)
    }
    
    pub fn set_server_error(&mut self, function: impl Fn(&Request) -> Response + 'static) {
        self.server_error = Box::new(function);
    }
    
    pub fn set_not_found_error(&mut self, function: impl Fn(&Request) -> Response + 'static) {
        self.not_found_error = Box::new(function);
    }

    #[cfg(feature = "_db_must")]
    pub fn connect_to_database(&mut self, database: Database) {
        self.database = Some(database)
    }
    
    /* ACCESS PROPERTIES */
    pub fn static_enabled(&self) -> bool {
        self.static_dir != None && self.static_url != None
    }
    
    pub fn get_environment(&self) -> &jinja::Environment<'static> {
        &self.environment
    }

    #[cfg(feature = "_db_must")]
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

    /// # Read in a file
    /// Read in the body of a file
    /// ## Parameters
    /// - `file: P` - The path to the file
    /// ## Returns
    /// This method returns a string, which contains the contents of the file.
    /// ## Panicking
    /// This method panics if:
    /// - The file can not be found
    /// - An unknown error causes Rust's std library to not be able to read the file
    fn read_file<P>(file: P) -> String
    where
        P: AsRef<Path> + Debug
    {
        match read_to_string(&file) {
            Ok(body) => body,
            Err(e) => match e.kind() {
                ErrorKind::NotFound => panic!("Unable to find file {file:?}."),
                _ => panic!("Something went wrong while reading in {file:?}: {e}")
            }
        }
    }

    fn list_items_in_dir<P>(dir: P) -> Vec<String>
    where
        P: AsRef<Path> + Debug
    {
        // Save the files into a vector
        let mut files = Vec::new();

        // Save the name of the directory
        let dir_name = Self::dir_name(&dir);

        let file_list = match read_dir(&dir) {
            Ok(result) => result,
            Err(e) => panic!("Unable to read in directory {dir:?} because of error {e}")
        };

        for file in file_list {
            let file = file.unwrap();

            let file_type = match file.file_type() {
                Ok(ty) => ty,
                Err(e) => {
                    eprintln!("Unable to read in file type of {path:?} because of error {e}", path = file.path());
                    continue
                }
            };

            let file_name = match file.file_name().to_str() {
                Some(string) => string.to_string(),
                None => {
                    eprintln!("Unable to find name of {path:?}, continuing...", path=file.path());
                    continue
                }
            };

            if file_type.is_dir() {
                let items_in_dir = Self::list_items_in_dir(file.path());
                for item in items_in_dir {
                    files.push(format!("{dir_name}/{item}"));
                }
            }
            else if file_type.is_file() {
                files.push(
                    format!("{dir_name}/{file_name}")
                );
            }
            else {
                continue
            }
        }

        files
    }

    fn dir_name<P>(dir: P) -> String
    where
        P: AsRef<Path> + Debug
    {
        if !dir.as_ref().is_dir() {
            panic!("Can not get the directory name of something that is not a directory")
        }
        match dir.as_ref().file_name() {
            Some(dir_name) => match dir_name.to_str() {
                Some(string) => string.to_string(),
                None => panic!("Unable to read in directory name of {dir:?}")
            },
            None => panic!("Unable to read in directory name of {dir:?}")
        }
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
            #[cfg(feature = "_db_must")]
            database: None
        }
    }
}
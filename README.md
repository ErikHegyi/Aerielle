# Aerielle
Aerielle is a high-level web framework written entirely in Rust.  
It was partly inspired by [Django](https://www.djangoproject.com/).

## Installation
To install Aerielle, just add it to your dependencies in your Cargo.toml file:
```toml
[dependencies]
aerielle = "0.1.0"
```
or install it from the command line:
```
cargo install aerielle
```

## Usage
Aerielle was developed with simplicity in mind.  
It makes starting a web server incredibly easy:
```rust
use aerielle::WebServer;


fn main() {
    // Initialize the server
    let mut server = WebServer::new();  // Starts a server on localhost:8000
    
    // Set up paths
    server.add_path("/", index);
    
    // Start the server (blocks the thread)
    server.start().unwrap();
}
```

That's it! Create the server, add the paths and the functions it should run, and start the server.  
The function definitions should look like this:
```rust
use aerielle::http::{Request, Response, Status, Header};


fn function(request: &Request) -> Response {
    /* Your logic */
    
    // Create the response
    let mut response = Response::new(Status::OK, body);
    
    // Add headers if you want
    let header = Header::new(
        "key".to_string(),
        "value".to_string()
    );
    response.add_header(header);
    
    // Return the response
    response
}
```

## A simple, Hello World webpage
```rust
use aerielle::{
    WebServer,
    http::{Request, Response, Status, Header}
};


fn index(request: &Request) -> Response {
    Response::new(
        Status::OK,
        String::from(
            "<h1>Hello World</h1>"
        )
    )
}


fn main() {
    let mut server = WebServer::new();
    server.add_path("/", index);
    server.start().unwrap();
}
```
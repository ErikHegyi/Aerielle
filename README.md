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
    server.add_path("/", index);  // 'localhost:8000/' will now call the index function
    
    // Start the server (blocks the thread)
    server.start().unwrap();
}
```

That's it! Create the server, add the paths and the functions it should run, and start the server.  
The function definitions should look like this:
```rust
use aerielle::{
    WebServer,
    http::{Request, Response, Status, Header},
    html::context
};


fn function(server: &WebServer, request: &Request) -> Response {
    /* Your logic */
    
    // Render an HTML page
    server.render(
        "template.html",  // The name of your HTML file inside your 'templates' folder
        context!{
            "key": "value"  // Here you define what should be passed to your HTML page in key-value pairs
        }
    )
}
```

## A simple, Hello World webpage
Create a `templates` folder inside your project directory.
Your project structure should look something like this:
```
project
- src
- - main.rs
- Cargo.toml
- Cargo.lock
- templates
```
Define your `index.html` page and place it inside your `templates` folder:
```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>{{ title }}</title>
</head>
<body>
    <p>{{ text }}</p>
</body>
</html>
```
And then create your Rust server:
```rust
use aerielle::{
    WebServer,
    http::{Request, Response, Status, Header},
    html::context
};


fn index(server: &WebServer, request: &Request) -> Response {
    server.render(
        "index.html",  // The name of your HTML file inside the templates folder
        // Define what you want to pass to your HTML page
        context!{
            "title": "Aerielle",
            "text": "Aerielle is wonderful!"
        }
    )
}


fn main() {
    let mut server = WebServer::new();
    server.add_path("/", index);
    server.start().unwrap();
}
```
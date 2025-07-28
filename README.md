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

# Server methods
The `WebServer` struct has many methods, allowing full customization of the project structure.  
These methods should be used **before** starting the server.  
Here is a list of these methods:
### Set IP
```rust
pub fn set_ip(&mut self, ip: impl ToString)
```
Set the IP manually.  
Default value is `localhost`
### Set port
```rust
pub fn set_port(&mut self, port: u16)
```
Set the port, on which the server should listen to requests.  
Default value is `8000`
### Set static URL
```rust
pub fn set_static_url(&mut self, url: impl ToString)
```
Set the **static url** of the server.  
The default value is `/static`.  
This means that if the server receives a request on the URL:
`ip:port/static/folder/file.ext`, it will automatically look inside the given **static folder**, and look for `folder\file.ext`, and serve it.
### Set static directory
```rust
pub fn set_static_dir(&mut self, dir: PathBuf)
```
Tell the server where your static files (`.css`, `.js`, ...) files are located.  
The server will automatically look inside this folder when a static file is requested on the static url.  
The default value is `static` inside the root folder.
### Disable static files
```rust
pub fn disable_static(&mut self)
```
Disable automatic static file serving.  
Useful if you want to implement it yourself.  
To re-enable it, set the static URL and the static directory.
### Set templates folder
```rust
pub fn set_templates_folder(&mut self, templates_folder: PathBuf)
```
Set the folder, where the server will automatically look for HTML files.  
Default is `templates` inside the root folder.
### Add a URL path
```rust
pub fn add_path(&mut self, pattern: &str, function: impl Fn(&Self, &Request) -> Response + 'static)
```
Add a path, and a handler function to the server.
For example:
```rust
fn load_english(server: &WebServer, request: &Request) -> Response {
    server.render("index.html", english_context())
}

fn set_up_paths(server: &mut WebServer) {
    server.add_path("/en", load_english)  // ip:port/en/ will now run the load_english function
}
```

#### Parameters
- `pattern` -> a regex pattern, against which the URL will be matched
- `function` -> the handler function
### Add a template
```rust
pub fn add_template(&mut self, name: String, path: PathBuf)
```
Add a template from outside the `templates` folder.

### Set server error
```rust
pub fn set_server_error(&mut self, function: impl Fn(&Request) -> Response + 'static)
```
Set the function, which should be run when a `500 - Server Error` is encountered.
### Set not found error
```rust
pub fn set_not_found_error(&mut self, function: impl Fn(&Request) -> Response + 'static)
```
Set the function, which should be run when a `404 - Not Found` is encountered.

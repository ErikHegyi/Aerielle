use std::{
    fs::read_to_string,
    hash::Hash,
    path::Path
};
use std::path::PathBuf;
use crate::{
    header,
    html::Context,
    http::{Header, Response, Status},
    server::WebServer
};


fn execute(
    server: &WebServer,
    command: &str,
    context: &mut Context
) -> String {
    if command.contains("if") { todo!() }
    else if command.contains("for") { todo!() }

    // Serve static files - format: static folder/file.extension
    else if command.contains("static") {
        // Get the static folder
        let folder: PathBuf = match server.static_folder() {
            Some(folder) => folder,
            None => panic!("Static files are disabled, but a static file was requested in an HTML template: \"{command}\"")
        };

        // Parse the path
        // Convert "static folder/file.extension" to "static/folder/file.extension"
        let path: &str = command
            .trim_start_matches("static")
            .trim_start_matches("/")
            .trim_end_matches("/")
            .trim();

        format!("/{static_folder}/{path}/", static_folder = folder.display())
    }

    // Variable
    else {
        match context.get(&command.to_string()) {
            Some(c) => c.clone(),
            None => {
                eprintln!("{command} not found in context");
                String::new()
            }
        }
    }

}

pub fn render(
    server: &WebServer,
    template: impl AsRef<Path>,
    mut context: Context
) -> Response {
    // Read in the file
    let file: String = read_to_string(template).unwrap();
    let mut html: String = String::new();

    // Parse the file
    let mut current: String = String::new();
    let mut command: bool = false;
    for i in 0..file.len() {
        let character: char = file.chars().nth(i).unwrap();
        if i >= 2 {
            if command {
                if file.chars().nth(i + 1) == Some('}') && file.chars().nth(i + 2) == Some('}') {
                    html += execute(server, current.trim(), &mut context).as_str();
                    println!("{html}");
                    current.clear();
                    command = false;
                } else {
                    current.push(character);
                }
            } else {
                if file.chars().nth(i - 2) == Some('{') && file.chars().nth(i - 1) == Some('{') {
                    command = true;
                } else {
                    html.push(character);
                }
            }
        } else {
            html.push(character);
        }
    }

    html = html.replace('{', "").replace('}', "");
    html = html.replace(r"\{", "{").replace(r"\}", "}");  // Allow for { and }
    
    // Create the response
    let headers: Vec<Header> = vec![
        header!("Content-Type": "text/html"),
        header!("Content-Length": (html.len()))
    ];
    
    let mut response: Response = Response::new(
        Status::OK,
        html
    );
    for header in headers {
        response.add_header(header);
    }
    
    // Return
    response
}
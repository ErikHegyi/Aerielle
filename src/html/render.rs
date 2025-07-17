use std::fs::read_to_string;
use std::hash::Hash;
use std::path::Path;
use crate::header;
use crate::html::Context;
use crate::http::{Header, Response, Status};


fn execute(
    command: &str,
    context: &Context
) -> String {
    if command.contains("if") { todo!() }
    else if command.contains("for") { todo!() }
    else if command.contains("static") { todo!() }
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
    template: impl AsRef<Path>,
    context: Context
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
                    html += execute(current.trim(), &context).as_str();
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


macro_rules! render {
    ($template: expr) => {
        {
            use crate::html::render;
            render($template, context!())
        }
    };
    ($template: expr, $context: expr) => {
        {
            use crate::html::render;
            render($template, $context)
        }
    }
}
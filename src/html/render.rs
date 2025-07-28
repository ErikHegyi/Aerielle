use minijinja::Value;
use crate::{
    header,
    http::{Response, Status, Header},
    server::WebServer
};


pub fn render(
    server: &WebServer,
    name: &str,
    context: Value
) -> Response {
    match server
        .get_template(name)
        .expect(format!("Unable to fetch template: {name}").as_str())
        .render(context)
    {
        // Rendering was successful
        Ok(body) => {
            let mut response = Response::new(Status::OK, body);
            response.add_header(
                header!("Content-Type": "text/html")
            );
            response
        },
        // Rendering was unsuccessful
        Err(e) => {
            eprintln!("Error while rendering template: {name} ({e})");
            Response::server_error()
        }
    }
}
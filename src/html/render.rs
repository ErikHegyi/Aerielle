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
        .get_environment()
        .get_template(name)
        .expect(format!("Unable to fetch template: {name}").as_str())
        .render(context) {
        Ok(body) => {
            let mut response = Response::new(Status::OK, body);
            response.add_header(
                header!("Content-Type": "text/html")
            );
            response
        },
        Err(e) => {
            eprintln!("Error while rendering template: {name} ({e})");
            Response::server_error()
        }
    }
}
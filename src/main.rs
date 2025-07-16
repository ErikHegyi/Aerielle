mod server;
mod sql;
mod http;
mod html;


fn serve(request: &http::Request) -> http::Response {
    http::Response::new(
        http::Status::OK, String::from("<h1>Hello, world!</h1>")
    )
}


fn main() {
    let mut s = server::WebServer::new();
    s.add_path("/", serve);
    s.start().unwrap();
}
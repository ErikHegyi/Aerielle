mod server;
mod sql;
mod http;
mod html;


fn serve(s: &server::WebServer, request: &http::Request) -> http::Response {
    s.render("index.html", context!{
        "title": "this is my kingdom come",
        "text": "kms"
    })
}


fn main() {
    let mut s = server::WebServer::new();
    s.add_path("/", serve);
    s.start().unwrap();
}
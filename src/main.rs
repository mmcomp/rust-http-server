use server::HttpRequest;
#[allow(dead_code)]
use server::HttpServer;

pub mod server;
pub mod server_tests;

fn test_handler(req: &HttpRequest) -> String {
    if req.params.get("name").is_some() {
        return req.params.get("name").unwrap().to_string();
    }
    "TEST".to_owned()
}

fn other_handler(_: &HttpRequest) -> String {
    "OTHER".to_owned()
}

fn default_handler(_: &HttpRequest) -> String {
    "DEFAULT".to_owned()
}

fn main() {
    let mut server = HttpServer::new("127.0.0.1:7878".to_owned(), default_handler);
    server.register_route("GET".to_owned(), "/other".to_owned(), other_handler);
    server.register_route("".to_owned(), "/test".to_owned(), test_handler);
    server.start();
}

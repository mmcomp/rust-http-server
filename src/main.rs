use server::HttpRequest;
#[allow(dead_code)]
use server::HttpServer;

pub mod server;

fn majid_handler(req: &HttpRequest) -> String {
    if req.params.get("name").is_some() {
        return req.params.get("name").unwrap().to_string();
    }
    "MAJID".to_owned()
}

fn alo_handler(_: &HttpRequest) -> String {
    "SALAM".to_owned()
}

fn default_handler(_: &HttpRequest) -> String {
    "DEFAULT".to_owned()
}

fn main() {
    let mut server = HttpServer::new("127.0.0.1:7878".to_owned());
    server.register_route("".to_owned(), "/".to_owned(), default_handler);
    server.register_route("GET".to_owned(), "/alo".to_owned(), alo_handler);
    server.register_route("".to_owned(), "/majid".to_owned(), majid_handler);
    server.start()
}

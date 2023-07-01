#[cfg(test)]
mod server_tests {
    use crate::server::{HttpRequest, HttpServer};
    use std::collections::HashMap;

    fn handler(_: &HttpRequest) -> String {
        "TEST".to_owned()
    }

    #[test]
    fn test_new() {
        let server = HttpServer::new("127.0.0.1:7878".to_owned(), handler);
        let mut test_server = HttpServer {
            addr: "127.0.0.1:7878".to_owned(),
            routes: HashMap::new(),
        };
        test_server.routes.insert("default:".to_owned(), handler);
        assert_eq!(server, test_server);
    }

    #[test]
    fn test_register_route() {
        let mut server = HttpServer::new("127.0.0.1:7878".to_owned(), handler);
        server.register_route("".to_owned(), "/".to_owned(), handler);
        assert_eq!(server.routes.len(), 2);
        assert_eq!(server.routes.get("default:").is_some(), true);
        assert_eq!(server.routes.get(":/").is_some(), true);
    }
}

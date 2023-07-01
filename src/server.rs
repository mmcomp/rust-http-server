use std::collections::HashMap;
#[allow(dead_code)]
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub query_string: String,
    pub base_url: String,
    pub headers: HashMap<String, String>,
    pub params: HashMap<String, String>,
    pub raw_body: String,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct HttpServer {
    pub addr: String,
    pub routes: HashMap<String, fn(&HttpRequest) -> String>,
}
impl HttpServer {
    fn get_method_path(&self, first_line_option: Option<&&str>) -> (String, String, String) {
        Option::expect(first_line_option, "Bad Request!");
        let first_line = match first_line_option {
            Some(value) => value,
            None => "",
        };
        let parts = first_line.split(" ");
        let mut method: &str = "";
        let mut index = 0;
        let mut path: &str = "";
        for part in parts {
            match index {
                0 => method = part,
                1 => path = part,
                _ => (),
            };
            index = index + 1;
        }

        let path_split: Vec<&str> = path.split("?").collect();
        let main_path = path_split[0];
        let mut query_string = "";
        if path_split.len() == 2 {
            query_string = path_split[1];
        }

        return (
            method.to_owned(),
            main_path.to_owned(),
            query_string.to_owned(),
        );
    }

    fn parse_query_string(&mut self, http_request: &mut HttpRequest) {
        if http_request.query_string == "" {
            return;
        }
        let query_strings: Vec<&str> = http_request.query_string.split("&").collect();
        for qs in query_strings {
            let key_value: Vec<&str> = qs.split("=").collect();
            http_request
                .params
                .insert(key_value[0].to_owned(), key_value[1].to_owned());
        }
    }

    fn get_headers(&mut self, http_raw_request: Vec<&str>, http_request: &mut HttpRequest) {
        let mut index = 0;

        for ln in http_raw_request.clone() {
            if index > 0 {
                if ln == "" {
                    break;
                }
                let col: Vec<&str> = ln.split(": ").collect();
                let key: String = match col.get(0) {
                    Some(val) => val.to_string(),
                    None => "".to_owned(),
                };
                let value: String = match col.get(1) {
                    Some(val) => val.to_string(),
                    None => "".to_owned(),
                };
                if key != "" && value != "" {
                    http_request.headers.insert(key, value);
                }
            }
            index += 1;
        }
        if http_raw_request[http_raw_request.len() - 2] == "" {
            http_request.raw_body = http_raw_request[http_raw_request.len() - 1].replace("\0", "").to_owned();
        }
        http_request.base_url = match http_request.headers.get("Host") {
            Some(host) => host.to_string(),
            None => "".to_owned(),
        }
    }

    fn get_rout(
        &mut self,
        http_raw_request: &Vec<&str>,
        http_request: &mut HttpRequest,
    ) -> &fn(&HttpRequest) -> String {
        let first_line_option = http_raw_request.get(0);
        let (method, path, query_string) = self.get_method_path(first_line_option);
        http_request.path = path;
        http_request.method = method;
        http_request.query_string = query_string;

        self.get_headers(http_raw_request.to_vec(), http_request);

        self.parse_query_string(http_request);

        let mut method_path = http_request.method.clone();
        method_path.push_str(":");
        method_path.push_str(&http_request.path.clone());
        if self.routes.get(&method_path).is_none() {
            method_path = ":".to_owned();
            method_path.push_str(&http_request.path.clone());
            if self.routes.get(&method_path).is_none() {
                return self.routes.get("default:").unwrap();
            }
        }
        self.routes.get(&method_path).unwrap()
    }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();
        use std::str;
        let x = str::from_utf8(&buffer).unwrap();

        let http_raw_request: Vec<&str> = x.split("\r\n").collect();

        let mut request = HttpRequest {
            method: "".to_owned(),
            path: "".to_owned(),
            query_string: "".to_owned(),
            base_url: "".to_owned(),
            headers: HashMap::new(),
            params: HashMap::new(),
            raw_body: "".to_owned(),
        };
        let handler = self.get_rout(&http_raw_request, &mut request);
        let handler_content = handler(&request);

        let status_line = "HTTP/1.1 200 OK";
        let contents = handler_content;
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    }

    pub fn start(&mut self) {
        let listener = TcpListener::bind(&self.addr).unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            self.handle_connection(stream);
        }
    }

    pub fn register_route(
        &mut self,
        method: String,
        route: String,
        handler: fn(&HttpRequest) -> String,
    ) {
        self.routes.insert(method + ":" + &route, handler);
    }

    pub fn new(addr: String, default_route: fn(&HttpRequest) -> String,) -> HttpServer {
        let mut server = HttpServer {
            addr,
            routes: HashMap::new(),
        };
        server.register_route("default".to_owned(), "".to_owned(), default_route);

        server
    }
}

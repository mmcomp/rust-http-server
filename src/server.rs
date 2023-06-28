use std::collections::HashMap;
#[allow(dead_code)]
use std::{
    fs,
    io::{prelude::*, BufReader},
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
}

pub struct HttpServer {
    addr: String,
    routes: HashMap<String, fn(&HttpRequest) -> String>,
}
impl HttpServer {
    fn get_method_path(&self, first_line_option: Option<&String>) -> (String, String, String) {
        Option::expect(first_line_option, "Bad Request!");
        let first_line = match first_line_option {
            Some(value) => value,
            None => "",
        };
        let parts = first_line.split(" ");
        let mut method: &str = "";
        let mut indx = 0;
        let mut path: &str = "";
        for part in parts {
            match indx {
                0 => method = part,
                1 => path = part,
                _ => (),
            };
            indx = indx + 1;
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

    fn get_headers(&mut self, http_raw_request: Vec<String>, http_request: &mut HttpRequest) {
        let mut indx = 0;

        for ln in http_raw_request {
            if indx > 0 {
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
            indx += 1;
        }
        http_request.base_url = match http_request.headers.get("Host") {
            Some(host) => host.to_string(),
            None => "".to_owned(),
        }
    }

    fn get_rout(
        &mut self,
        http_raw_request: &Vec<String>,
        http_request: &mut HttpRequest,
    ) -> &fn(&HttpRequest) -> String {
        let first_line_option = http_raw_request.get(0);
        let (method, path, query_string) = self.get_method_path(first_line_option);
        http_request.path = path;
        http_request.method = method;
        http_request.query_string = query_string;

        self.get_headers(http_raw_request.to_vec(), http_request);

        self.parse_query_string(http_request);

        if self.routes.get(&http_request.path).is_none() {
            return self.routes.get("/").unwrap();
        }
        self.routes.get(&http_request.path).unwrap()
    }

    fn handle_connection(&mut self, mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let http_raw_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let mut request = HttpRequest {
            method: "".to_owned(),
            path: "".to_owned(),
            query_string: "".to_owned(),
            base_url: "".to_owned(),
            headers: HashMap::new(),
            params: HashMap::new(),
        };
        let handler = self.get_rout(&http_raw_request, &mut request);
        let handler_content = handler(&request);
        println!(
            "Handler for route '{}' => '{}'",
            request.path, handler_content
        );

        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("hello.html")
            .unwrap()
            .replace("#CONTENT#", &handler_content);
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

    pub fn register_route(&mut self, route: String, handler: fn(&HttpRequest) -> String) {
        self.routes.insert(route, handler);
    }

    pub fn new(addr: String) -> HttpServer {
        HttpServer {
            addr,
            routes: HashMap::new(),
        }
    }
}

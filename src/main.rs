use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    let parse_resp = |contents: String, status_line: Option<&str>| -> String {
        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line.unwrap_or("HTTP/1.1 200 OK"),
            contents.len(),
            contents
        );
        return response;
    };

    if buffer.starts_with(get) {
        let contents = fs::read_to_string("./templates/index.html").unwrap();

        let resp_200 = parse_resp(contents, None);

        stream.write(resp_200.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let contents = fs::read_to_string("./templates/404.html").unwrap();

        let resp_404 = parse_resp(contents, Some("HTTP/1.1 404 NOT FOUND"));

        stream.write(resp_404.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

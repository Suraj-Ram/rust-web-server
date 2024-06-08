use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

const SUCCESS_LINE: &str = "HTTP/1.1 200 OK";
const NOT_FOUND_LINE: &str = "HTTP/1.1 404 NOT FOUND";

fn main() {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        (SUCCESS_LINE, "hello.html")
    } else {
        (NOT_FOUND_LINE, "404.html")
    };

    let html_contents = fs::read_to_string(filename).unwrap();
    let length = html_contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{html_contents}");
    stream.write_all(response.as_bytes()).unwrap();

}

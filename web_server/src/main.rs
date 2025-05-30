use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {http_request:#?}");

    let request_line = http_request
        .get(0)
        .map(|line| line.as_str())
        .unwrap_or("");
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        println!("Received a GET request for the root path");
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        println!("Received a non-GET request or a request for a different path");
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
        let contents = fs::read_to_string(filename).unwrap();
        let lenght = contents.len();
        let response = format!(
            "{status_line}\r\nContent-Length: {lenght}\r\n\r\n{contents}"
        );

        stream.write_all(response.as_bytes()).unwrap();
    
    
}
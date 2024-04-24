use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

// ofc you don't ".unwrap()" everywhere in production
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let s = stream.unwrap();
        handle_connection(s);
    }
}

// this method only logs the request: when using this and typing "127.0.0.1:7878" in the address bar,
// the browser will retry the request many times as it doesn't receive any response
fn handle_connection_no_reply(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", req);
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let req: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let get = "GET / HTTP/1.1";
    let (status_line, filename) = match req[0].starts_with(get) {
        true => ("HTTP/1.1 200 OK", "index.html"),
        false => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let content = fs::read_to_string(filename).unwrap();

    // the .html files must be stored at the root-level project, not under "src"
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, content.len(), content);
    stream.write_all(response.as_bytes()).unwrap();

    // // as we are only interested in the first part of the request, we could have read the request as follows:
    // let req = buf_reader.lines().next().unwrap().unwrap(); // req is now a string, not an array
    // let (status_line, filename) = match req.starts_with(get)

    // alternative way to get the tuple "(status_line, filename)":
    // let (status_line, filename) = if req[0] == "GET / HTTP/1.1" {
    //     ("HTTP/1.1 200 OK", "hello.html")
    // } else {
    //     ("HTTP/1.1 404 NOT FOUND", "404.html")
    // };
}

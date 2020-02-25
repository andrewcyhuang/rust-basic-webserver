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
// stream argument must be set as mutable here because of TCPStream instance
// magic, its internal state might change. Therefore, even though we are only
// reading the input, we need the mut keyword

fn handle_connection(mut stream: TcpStream) {
    // Super simplified buffer management - limited to reading 512 bytes of data
    let mut buffer = [0; 512]; // buffer == mutable array of 512 bytes
    
    // stream.read will read request + data & put into buffer
    stream.read(&mut buffer).unwrap();
    //~~~~> PS. unwrap() stops the program @ error - irl you'd use error handling here
    
    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    
    let contents = fs::read_to_string(filename).unwrap();
    let res = format!("{}{}", status_line, contents);

    stream.write(res.as_bytes()).unwrap();

    // flush waits & prevents connection from continuing until write() is complete
    stream.flush().unwrap();
    
}

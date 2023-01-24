/* 
A bare bones multi-threaded server that can handle multiple requests at once. 
Based on the implementation in the Rust Lang book at https://doc.rust-lang.org/book/ch20-02-multithreaded.html
*/

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use server::ThreadPool;

fn main() {
    //Create a TCP listener on port 7878 with the localhost IP address 127.0.0.1
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //Create a thread pool with 4 threads
    let pool = ThreadPool::new(4);

    //Iterate over the incoming connections
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        //Spawn a new thread for each connection
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let html_contents = fs::read_to_string(filename).unwrap();
    
    handle_request(stream, status_line, html_contents);
}

fn handle_request(mut stream: TcpStream, status_line: &str, html_contents: String) {
    
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        html_contents.len(),
        html_contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

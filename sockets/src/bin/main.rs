fn main() {
    println!("Hello, world!");
}
/*
A bare bones multi-threaded server that can handle multiple requests at once.
Inspired by the implementation in the Rust Lang book at https://doc.rust-lang.org/book/ch20-02-multithreaded.html
and the video series on the Rust Lang book from the Youtube channel
"Let's Get Rusty": https://www.youtube.com/playlist?list=PLai5B987bZ9CoVR-QEIN9foz4QCJ0H2Y8
*/

use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use sockets::ThreadPool;

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
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    //Create a buffer to hold the request
    let mut buffer = [0; 1024];
    //Read the request into the buffer
    stream.read(&mut buffer).unwrap();

    //Get the first item from the iterator.
    //Unwrap the Option and the Result to get the request line.
    let request_line = buffer.lines().next().unwrap().unwrap();

    //Return the HTML filename and status line based on the request line
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    //Get the HTML contents from the file
    let html_contents = fs::read_to_string(filename).unwrap();

    handle_request(stream, status_line, html_contents);
}

fn handle_request(mut stream: TcpStream, status_line: &str, html_contents: String) {
    //Create the response
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        html_contents.len(),
        html_contents
    );

    //Write the response to the stream
    stream.write(response.as_bytes()).unwrap();
    //Flush the stream to ensure the response is sent
    stream.flush().unwrap();
}

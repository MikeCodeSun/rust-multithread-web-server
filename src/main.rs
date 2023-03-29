use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, fs};

use multithread_web_server::ThreadPool;

fn main() {
    // tcp server listen to port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // create new thread pool
    let thread_pool = ThreadPool::new(4);
    // iterate each connection stream in listener
    for stream in listener.incoming().take(3) {
        let stream = stream.unwrap();
        thread_pool.excute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    // get req from stream connection
    let buf_stream = BufReader::new(&stream);
    // get first line from req
    let request_line = buf_stream.lines().next().unwrap().unwrap();
    // uri route
    let (status, file_path) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "./public/index.html"),
        "GET /about HTTP/1.1" => ("HTTP/1.1 200 OK", "./public/about.html"),
        _ => ("HTTP/1.1 404 NOT FOUND", "./public/404.html"),
    };
    // response
    let contents = fs::read_to_string(file_path).unwrap();
    let length = contents.len();
    let res = format!("{status}/r/nContent_length: {length}\r\n\r\n{contents}");
    // write response to stream.
    stream.write_all(&res.as_bytes()).unwrap();
}

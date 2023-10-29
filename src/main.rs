use std::{
    fs,
    net::{TcpListener, TcpStream},
    io::{prelude::*, BufReader}, thread, time::Duration
};
use rusty_web_server_demo::ThreadPool;

const POOL_CAPACITY: usize = 4;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(POOL_CAPACITY);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn read_content(file_path: &str) -> (String, String) {
    let contents = fs::read_to_string(file_path).unwrap();
    let content_length = format!("Content-Length: {}", contents.len());
    return (content_length, contents);
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let new_line = "\r\n";

    println!("RECEIVED REQUEST!");

    let (status_desc, file_path) = match request_line.as_str() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            /* Simulating Slow Response... */
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let (content_length, contents) = read_content(file_path);

    let response = format!("{status_desc}{new_line}{content_length}{new_line}{new_line}{contents}");
    stream.write_all(response.as_bytes()).unwrap();    
}

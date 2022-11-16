use web_server::ThreadPool;

use std::{
    io::{prelude::*,BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
    fs,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(3);

    for stream in listener.incoming().take(5) {
        let stream = stream.unwrap();

        pool.execute(move || {
            handle_connection(stream);
        });
    }

    println!("Shutting down main thread.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    // println!("Request: {:#?}", http_request);

    let (status_line, filename) =
        match &(http_request[0])[..] {
            "GET / HTTP/1.1"      => ("HTTP/1.1 200 OK", "hello.html"),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(3));
                ("HTTP/1.1 200 OK", "hello.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
    println!("{:?}",(status_line, filename));

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

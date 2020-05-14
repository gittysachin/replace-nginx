use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;

use hello::Client;

use async_std::io;
use async_std::prelude::*;

#[async_std::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let mut client = Client::new("localhost:6379").await?;
    let mut result;
    let mut response: String = "".to_string();
    for stream in listener.incoming() {
        let mut _stream = stream.unwrap();
        result = client.get("sachin".into()).await.unwrap();

        if result.to_string() == "-1" {
            // flag = 1;
            println!("Not Available");
            response = handle_connection(_stream);
            client.set("sachin".into(), (&response).into()).await.unwrap();
        } else {
                println!("Yes, Available");
                _stream.write(result.as_bytes()).unwrap();
                _stream.flush().unwrap();
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> String {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

    response.to_owned().to_string()
}
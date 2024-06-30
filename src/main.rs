use std::io::{Write};
use std::net::{TcpListener};

fn main() {
    println!("******************! Flash Cache Server !*******************");

    let listener: TcpListener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let response = "+PONG\r\n";
                stream.write(response.as_bytes()).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

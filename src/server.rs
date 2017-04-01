use chrono::offset::local::Local;

use std::io::{Read, Write, BufReader, BufRead};
use std::net::{TcpListener, Shutdown};
use std::time::Duration;

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(host: &str) -> Self {
        Server { listener: TcpListener::bind(host).unwrap() }
    }

    pub fn listen(&mut self) {
        println!("Waiting for connection from client...");

        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("[SERVER]: Recieve connection from client. {:?}", stream);
                    let mut buffered_stream = BufReader::new(stream.try_clone().unwrap());
                    let mut recieve_buffer = String::new();

                    loop {
                        match buffered_stream.read_line(&mut recieve_buffer) {
                            Ok(s) => {
                                if s == 2 {
                                    // Request header is end.
                                    break;
                                }
                                println!("read size is: {}", s);
                            },
                            Err(e) => {
                                println!("FAIL: {}", e);
                                break;
                            },
                        };
                    }
                    println!("{}", recieve_buffer); // TODO: リクエストからパスを引き出す

                    let send_buffer = [
                        format!("HTTP/1.1 200 OK"),
                        format!("Date:  {}", Local::now()),
                        format!("Server: Modoki/0.1"),
                        format!("Connection: close"),
                        format!("Content-type: text/html"),
                        format!(""),
                        format!(r#"
                            <!DOCTYPE HTML PUBLIC "-//IETF//DTD HTML 2.0//EN">
                            <html><head>
                            <title>OK</title>
                            </head><body>
                            <h1>It works!</h1>
                            </body></html>
                        "#),
                    ].join("\r\n");

                    stream.write_all(send_buffer.as_bytes()).unwrap();
                    println!("[SERVER]: Send message to client.");
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    }
}


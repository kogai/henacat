use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs::File;

#[derive(Debug)]
pub struct Server {
    listener: TcpListener,
}

impl Server {
    pub fn new(host: &str) -> Self {
        Server { listener: TcpListener::bind(host).unwrap() }
    }

    pub fn listen<C>(&self, on_connect: C)
        where C: Fn(&TcpStream) -> ()
    {
        println!("Waiting for connection from client...");
        for stream in self.listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    println!("Accept connection from client.");
                    let mut recieve_file = File::create("server_recieve.txt").unwrap();
                    let mut recieve_buffer = Vec::new();
                    stream.read_to_end(&mut recieve_buffer).unwrap();
                    recieve_file.write(&recieve_buffer).unwrap();

                    let mut send_file = File::open("server_send.txt").unwrap();
                    let mut send_buffer = Vec::new();
                    send_file.read_to_end(&mut send_buffer).unwrap();
                    stream.write(&send_buffer).unwrap();

                    on_connect(&stream);
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Client {}

impl Client {}


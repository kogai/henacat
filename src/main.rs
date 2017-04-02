extern crate chrono;

mod server;
mod method;
mod content_type;
mod status;
// mod client;

fn main() {
    // let s = "\r\n".as_bytes();
    // println!("{:?}", s);
    let mut server = server::Server::new("127.0.0.1:8000");
    server.listen();
}


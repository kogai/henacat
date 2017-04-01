extern crate chrono;

mod server;
mod client;

fn main() {
    let mut server = server::Server::new("127.0.0.1:8000");
    server.listen();
}


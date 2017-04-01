mod server;
mod client;

use std::thread;
use std::time::Duration;

fn main() {
    let mut server = server::Server::new("127.0.0.1:8000");
    server.listen();

    // let client = tcp::Client::new("127.0.0.1:8000");
    // client.send();
}


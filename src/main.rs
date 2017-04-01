use std::thread;
use std::time::Duration;

mod tcp;

fn main() {
    let mut server = tcp::Server::new("127.0.0.1:8000");
    thread::spawn(move || server.listen());

    let mut client = tcp::Client::new("127.0.0.1:8000");
    client.send();

    thread::sleep(Duration::from_millis(500));
}


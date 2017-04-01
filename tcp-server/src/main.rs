mod tcp;

fn main() {
    let mut server = tcp::Server::new("127.0.0.1:8000");
    server.listen();
}


mod tcp;

fn main() {
    let server = tcp::Server::new("127.0.0.1:8000");
    server.listen(|s| {
        println!("CONNECT: {:?}", s)
    });
}


use server::Server;

mod server;
mod controller;

fn main() {
    let server = Server::default();
    let address = server.get_serve_address();
    println!("{}", address);
    server.run();
}

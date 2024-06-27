use std::thread;

use server::Server;

mod server;
mod controller;
mod tray;

fn main() {
    let server = Server::default();
    let addresses = server.get_addresses();

    thread::spawn(move || {
        server.run();
    });

    tray::main(addresses)
}

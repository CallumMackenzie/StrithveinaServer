use std::net::UdpSocket;

use games::GamesManager;

mod socket;
mod world;
mod user;
mod games;

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:34254").unwrap();
	GamesManager::new(socket).run();
}

use std::net::UdpSocket;

use games::GamesManager;

#[cfg(test)]
mod tests;

mod socket;
mod world;
mod user;
mod games;

fn main() {
	println!("Starting Strithveina server ...");
    let socket = UdpSocket::bind("127.0.0.1:34254").unwrap();
	println!("Created socket!");
	if let Ok(mut gm) = GamesManager::new(socket){
		println!("Game manager initialized.");
		if let Err(_error) = gm.run() {
			println!("Server runtime error");
		}
	} else {
		println!("Server initialization error");
	}
}
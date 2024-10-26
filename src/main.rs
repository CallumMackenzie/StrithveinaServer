mod socket;

use std::{
    net::{SocketAddr, UdpSocket},
    sync::mpsc::{channel, Receiver},
};

use socket::DynamicUDPSocket;

struct GamesManager {
    running_games: u32,
    max_games: u32,
    socket: DynamicUDPSocket,
}

impl<'a> GamesManager {
    fn new(socket: UdpSocket) -> Self {
        GamesManager {
            running_games: 0,
            max_games: 5,
			socket: DynamicUDPSocket::new(socket)
        }
    }

    fn run(&mut self) -> ! {
        let (sender, receiver) = channel::<&'a str>();
        loop {
            // Poll for client requests
            // Check message queue for RunningGames new info
        }
    }
}

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:34254").unwrap();
    let gm = GamesManager::new(socket);
}

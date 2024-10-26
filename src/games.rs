use std::{net::UdpSocket, sync::mpsc::channel};

use crate::socket::{DynamicUDPSocket, GameSocket};


pub struct GamesManager<S: GameSocket> {
    running_games: u32,
    max_games: u32,
    socket: S,
}

impl<'a> GamesManager<DynamicUDPSocket> {
    pub fn new(socket: UdpSocket) -> Self {
        GamesManager {
            running_games: 0,
            max_games: 5,
			socket: DynamicUDPSocket::new(socket)
        }
    }

    pub fn run(&mut self) -> ! {
        let (sender, receiver) = channel::<&'a str>();
        loop {
            // Poll for client requests
            // Check message queue for RunningGames new info
        }
    }
}

use std::{net::UdpSocket, sync::mpsc::channel};

use crate::socket::{DynamicUDPSocket, GameSocket};

/// Manages initial client connections to create games to connect to as well as ensuring too many games are not created
pub struct GamesManager<S: GameSocket> {
    running_games: u32,
    max_games: u32,
    socket: S,
}

impl<'a> GamesManager<DynamicUDPSocket> {
	// Creates a new games manager for the given socket
    pub fn new(socket: UdpSocket) -> Self {
        GamesManager {
            running_games: 0,
            max_games: 5,
            socket: DynamicUDPSocket::new(socket),
        }
    }

	/// Starts the games manager, polling UDP
    pub fn run(&mut self) -> ! {
        let (sender, receiver) = channel::<&'a str>();
        loop {
            // Poll for client requests
            // Check message queue for RunningGames new info
        }
    }
}

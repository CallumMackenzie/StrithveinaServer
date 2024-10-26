use std::{io::Error, net::UdpSocket, sync::mpsc::channel};

use crate::socket::{DynamicUDPSocket, GameSocket};

/// Manages initial client connections to create games to connect to as well as ensuring too many games are not created
pub struct GamesManager<S: GameSocket> {
    running_games: u32,
    max_games: u32,
    socket: S,
}

impl<'a> GamesManager<DynamicUDPSocket> {
	// Creates a new games manager for the given socket
    pub fn new(socket: UdpSocket) -> Result<Self, ()>  {
		// I'll wait forever for you!
		if let Err(_) = socket.set_read_timeout(None) {
			return Err(());
		}
        Ok(GamesManager {
            running_games: 0,
            max_games: 5,
            socket: DynamicUDPSocket::new(socket),
        })
    }

	/// Starts the games manager, polling UDP
    pub fn run(&mut self) -> Result<(), Error> {
        let (sender, receiver) = channel::<&'a str>();
        loop {
            // Poll for client requests
			self.poll_game_connect_request()?;
            // Check message queue for RunningGames new info
        }
    }

	/// Checks if udp socket has a packet waiting that matches the format of a
	/// game connection request and if so, processes it
	fn poll_game_connect_request(&mut self) -> Result<(), Error> {
		// Peek for data waiting
		let (size, addr) = self.socket.peek_from()?;

		// TODO: Check request format

		// TODO: Parse request

		// All good
		Ok(())
	}
}

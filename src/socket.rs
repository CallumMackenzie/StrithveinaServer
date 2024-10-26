use std::net::{SocketAddr, UdpSocket};

/// Packet size
const DYNAMIC_SOCKET_BUFFER_BYTES: usize = 32 * 32;

/// A trait representing a game socket which data is sent/recieved from.
/// Trait created to allow for dependency injection during testing.
pub trait GameSocket {
	/// Creates a new GameSocket from the given UdpSocket
	fn new(socket: UdpSocket) -> Self;
	/// Recieves bytes and returns their size and the address they originated from.
	fn recv_from(&mut self) -> std::io::Result<(usize, SocketAddr)>;
}

/// Dynamic UDP socket using a UdpSocket recieving packets from a client.
pub struct DynamicUDPSocket {
    buffer: [u8; DYNAMIC_SOCKET_BUFFER_BYTES],
    socket: UdpSocket,
}

impl GameSocket for DynamicUDPSocket {
    fn new(socket: UdpSocket) -> Self {
		Self { buffer: [0; DYNAMIC_SOCKET_BUFFER_BYTES], socket }
	}
	
	fn recv_from(&mut self) -> std::io::Result<(usize, SocketAddr)> {
        let (amt, src) = self.socket.recv_from(&mut self.buffer)?;
        Ok((amt, src))
    }
}

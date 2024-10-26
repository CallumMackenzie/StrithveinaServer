use std::net::{SocketAddr, UdpSocket};


const DYNAMIC_SOCKET_BUFFER_BYTES: usize = 32 * 32;

pub struct DynamicUDPSocket {
    buffer: [u8; DYNAMIC_SOCKET_BUFFER_BYTES],
    socket: UdpSocket,
}

impl DynamicUDPSocket {
    pub fn new(socket: UdpSocket) -> Self {
		Self { buffer: [0; DYNAMIC_SOCKET_BUFFER_BYTES], socket }
	}
	
	pub fn recv_from(&mut self) -> std::io::Result<(usize, SocketAddr)> {
        let (amt, src) = self.socket.recv_from(&mut self.buffer)?;
        Ok((amt, src))
    }
}

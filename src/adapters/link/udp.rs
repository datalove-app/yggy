use smol::Async;
use std::net::{SocketAddr, UdpSocket};

#[derive(Debug)]
pub struct UDPSocket {
    addr: SocketAddr,
    socket: Async<UdpSocket>,
}

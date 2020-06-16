use crate::error::Error;
use smol::Async;
use std::net::{SocketAddr, UdpSocket};

#[derive(Debug)]
pub struct UDPSocket {
    addr: SocketAddr,
    socket: Option<Async<UdpSocket>>,
}

impl UDPSocket {
    pub fn bind(addr: SocketAddr) -> Result<Self, Error> {
        let raw_socket = Async::<UdpSocket>::bind(addr).map_err(|e| Error::Init(e.into()))?;
        let socket = Async::new(raw_socket)
            .map(Some)
            .map_err(|e| Error::init(e.into()))?;

        Ok(Self { addr, socket })
    }

    // fn split(mut self) -> Result
}

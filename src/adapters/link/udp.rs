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
        let socket = Async::<UdpSocket>::bind(addr)
            .map(Some)
            .map_err(|e| Error::Init(e.into()))?;

        Ok(Self { addr, socket })
    }

    pub fn connect(&self, addr: SocketAddr) -> Result<(), Error> {
        let socket = &self.socket.as_ref().ok_or_else(|| {
            Error::Init(anyhow::Error::msg("UDPSocket must already be initialized"))
        })?;
        socket.get_ref().connect(addr).map_err(Error::Conn)?;
        Ok(())
    }

    // fn split(mut self) -> Result
}

use super::interface::{LinkReader, LinkWriter};
use smol::Async;
use std::{
    hash,
    net::{SocketAddr, UdpSocket},
};
use yggy_core::dev::*;

#[derive(Debug)]
pub struct UDPSocket {
    addr: SocketAddr,
    socket: Async<UdpSocket>,
}

impl UDPSocket {
    #[inline]
    pub fn bind(addr: SocketAddr) -> Result<Self, Error> {
        let socket = Async::<UdpSocket>::bind(addr).map_err(ConnError::Interface)?;
        Ok(Self { addr, socket })
    }

    #[inline]
    pub fn connect(&self, addr: &SocketAddr) -> Result<(), Error> {
        Ok(self
            .socket
            .get_ref()
            .connect(addr)
            .map_err(ConnError::Interface)?)
    }

    #[inline]
    pub fn local_addr(&self) -> &SocketAddr {
        &self.addr
    }

    #[inline]
    pub fn remote_addr(&self) -> Result<SocketAddr, Error> {
        Ok(self
            .socket
            .get_ref()
            .peer_addr()
            .map_err(ConnError::Interface)?)
    }

    #[inline]
    pub fn split(self) -> (LinkReader, LinkWriter) {
        let (r, w) = io::AsyncReadExt::split(self);
        (LinkReader::UDP(r), LinkWriter::UDP(w))
    }
}

impl Eq for UDPSocket {}
impl PartialEq for UDPSocket {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}

impl hash::Hash for UDPSocket {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.addr.hash(state);
    }
}

impl AsyncRead for UDPSocket {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &mut [u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        // If we're connected to a remote addr,
        // if let Some(addr) = self.remote_addr().ok() {

        // } else {

        // }

        // let reader = self.socket;
        // futures::pin_mut!(reader);
        // reader.poll_read(cx, buf)

        unimplemented!()
    }
}

impl AsyncWrite for UDPSocket {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &[u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        // let writer = &mut self.writer;
        // futures::pin_mut!(writer);
        // writer.poll_write(cx, buf)
        unimplemented!()
    }

    #[inline]
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), io::Error>> {
        // let writer = &mut self.writer;
        // futures::pin_mut!(writer);
        // writer.poll_flush(cx)
        unimplemented!()
    }

    #[inline]
    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), io::Error>> {
        // let writer = &mut self.writer;
        // futures::pin_mut!(writer);
        // writer.poll_close(cx)
        unimplemented!()
    }
}

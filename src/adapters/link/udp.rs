use super::interface::{LinkReader, LinkWriter};
use futures::{io, prelude::*, task};
use smol::Async;
use std::{
    net::{SocketAddr, UdpSocket},
    pin::Pin,
};
use yggy_core::error::Error;

#[derive(Debug)]
pub struct UDPSocket {
    addr: SocketAddr,
    socket: Async<UdpSocket>,
}

impl UDPSocket {
    #[inline]
    pub fn bind(addr: SocketAddr) -> Result<Self, Error> {
        let socket = Async::<UdpSocket>::bind(addr).map_err(|e| Error::Init(e.into()))?;
        Ok(Self { addr, socket })
    }

    #[inline]
    pub fn connect(&self, addr: &SocketAddr) -> Result<(), Error> {
        self.socket.get_ref().connect(addr).map_err(Error::Conn)
    }

    #[inline]
    pub fn local_addr(&self) -> &SocketAddr {
        &self.addr
    }

    #[inline]
    pub fn remote_addr(&self) -> Result<SocketAddr, Error> {
        self.socket.get_ref().peer_addr().map_err(Error::Conn)
    }

    #[inline]
    pub fn split(self) -> (LinkReader, LinkWriter) {
        let (r, w) = io::AsyncReadExt::split(self);
        (LinkReader::UDP(r), LinkWriter::UDP(w))
    }
}

impl AsyncRead for UDPSocket {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &mut [u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        // let reader = &mut self.reader;
        // futures::pin_mut!(reader);
        // reader.poll_read(cx, buf)

        unimplemented!()
    }
}

impl AsyncWrite for UDPSocket {
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

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), io::Error>> {
        // let writer = &mut self.writer;
        // futures::pin_mut!(writer);
        // writer.poll_flush(cx)
        unimplemented!()
    }

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

use super::interface::{LinkReader, LinkWriter};
use smol::Async;
use std::{
    hash,
    net::{SocketAddr, TcpListener, TcpStream},
};
use yggy_core::{dev::*, types::PeerURI};

///
/// TODO look into https://github.com/wzhd/ustcp
#[derive(Debug)]
pub struct TCPListener {
    addr: SocketAddr,
    listener: Async<TcpListener>,
}

impl TCPListener {
    #[inline]
    pub fn bind(addr: SocketAddr) -> Result<Self, Error> {
        let listener = Async::<TcpListener>::bind(&addr).map_err(ConnError::Interface)?;
        Ok(Self { addr, listener })
    }

    #[inline]
    pub async fn accept(&self) -> Result<TCPStream, Error> {
        let (stream, addr) = self.listener.accept().await.map_err(ConnError::Interface)?;
        Ok(TCPStream { stream, addr })
    }

    #[inline]
    pub fn local_addr(&self) -> &SocketAddr {
        &self.addr
    }
}

impl Eq for TCPListener {}
impl PartialEq for TCPListener {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}

impl hash::Hash for TCPListener {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.addr.hash(state);
    }
}

#[derive(Debug)]
pub struct TCPStream {
    addr: SocketAddr,
    stream: Async<TcpStream>,
}

impl TCPStream {
    #[inline]
    pub async fn connect(addr: SocketAddr) -> Result<Self, Error> {
        let stream = Async::<TcpStream>::connect(&addr)
            .await
            .map_err(ConnError::Interface)?;
        Ok(Self { addr, stream })
    }

    #[inline]
    pub fn split(self) -> (LinkReader, LinkWriter) {
        let (r, w) = io::AsyncReadExt::split(self);
        (LinkReader::TCP(r), LinkWriter::TCP(w))
    }

    #[inline]
    pub fn remote_addr(&self) -> &SocketAddr {
        &self.addr
    }
}

impl AsyncRead for TCPStream {
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &mut [u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        // let reader = self.stream;
        // futures::pin_mut!(reader);
        // reader.poll_read(cx, buf)

        unimplemented!()
    }
}

impl AsyncWrite for TCPStream {
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

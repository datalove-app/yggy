use super::{
    tcp::{TCPListener, TCPStream},
    udp::UDPSocket,
    LinkInfo,
};
// use smol::Async;
use std::sync::Arc;
use yggy_core::{dev::*, types::PeerURI};

///
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum LinkInterface {
    TCP(TCPListener),
    UDP,
}

impl LinkInterface {
    ///
    pub fn new(info: Arc<LinkInfo>) -> Result<Self, Error> {
        match info.listen_uri {
            PeerURI::TCP(addr) => Ok(Self::TCP(TCPListener::bind(addr)?)),
            _ => unimplemented!(),
        }
    }

    /// Establishes the link interface, returning a `Stream` of
    /// `LinkReader` + `LinkWriter` pairs upon every new connection.
    pub fn listen(&self) -> impl Stream<Item = Result<(LinkReader, LinkWriter), Error>> + '_ {
        match self {
            Self::TCP(listener) => listener.incoming().map_ok(|stream| stream.split()),
            // PeerURI::TCP(addr) => stream::once(async move { TCPListener::bind(addr) })
            //     .map_ok(|listener: TCPListener| listener.incoming())
            //     .try_flatten()
            //     .map_ok(|stream| stream.split()),
            // PeerURI::UDP(addr) => UDPSocket::bind(addr)?.split(),
            // PeerURI::SOCKS
            // PeerURI::TOR
            _ => unimplemented!(),
        }
    }
}

///
#[derive(Debug)]
pub enum LinkReader {
    TCP(io::ReadHalf<TCPStream>),
    UDP(io::ReadHalf<UDPSocket>),
    // SOCKS(io::ReadHalf<TCPStream>),
    // #[cfg(feature = "tor")]
    // TOR(io::ReadHalf<TCPStream>),
}

impl AsyncRead for LinkReader {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &mut [u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        match self.get_mut() {
            // Self::UDP(reader) => {
            //     futures::pin_mut!(reader);
            //     reader.poll_read(cx, buf)
            // }
            _ => unimplemented!(),
        }
    }
}

///
#[derive(Debug)]
pub enum LinkWriter {
    TCP(io::WriteHalf<TCPStream>),
    UDP(io::WriteHalf<UDPSocket>),
    // SOCKS(io::WriteHalf<TCPStream>),
    // #[cfg(feature = "tor")]
    // TOR(io::WriteHalf<TCPStream>),
}

impl AsyncWrite for LinkWriter {
    #[inline]
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &[u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        match self.get_mut() {
            // Self::UDP(writer) => {
            //     futures::pin_mut!(writer);
            //     writer.poll_write(cx, buf)
            // }
            _ => unimplemented!(),
        }
    }

    #[inline]
    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), io::Error>> {
        match self.get_mut() {
            // Self::UDP(writer) => {
            //     futures::pin_mut!(writer);
            //     writer.poll_flush(cx)
            // }
            _ => unimplemented!(),
        }
    }

    #[inline]
    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), io::Error>> {
        match self.get_mut() {
            // Self::UDP(writer) => {
            //     futures::pin_mut!(writer);
            //     writer.poll_close(cx)
            // }
            _ => unimplemented!(),
        }
    }
}

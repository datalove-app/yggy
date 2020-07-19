use super::{
    tcp::{TCPListener, TCPStream},
    LinkInfo,
};
use std::{hash, sync::Arc};
use stream_cancel::{StreamExt as _, Trigger, Tripwire};
use yggy_core::{dev::*, types::PeerURI};

/// Handle to an open hardware interface, listening for incoming connections.
#[derive(Debug)]
pub struct LinkInterface {
    // name: String,
    addr: PeerURI,
    stop: Trigger,
}

impl LinkInterface {
    pub fn new(
        addr: PeerURI,
    ) -> Result<(Self, impl Stream<Item = (LinkInfo, LinkReader, LinkWriter)>), Error> {
        let listener = LinkListener::new(&addr)?;
        let (stop, stopped) = Tripwire::new();
        Ok((Self { addr, stop }, listener.take_until_if(stopped)))
    }
}

impl Eq for LinkInterface {}
impl PartialEq for LinkInterface {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
impl hash::Hash for LinkInterface {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.addr.hash(state);
    }
}

/// Listens for incoming `Link`s.
#[derive(Debug, Eq, Hash, PartialEq)]
enum LinkListener {
    TCP(TCPListener),
    SOCKS,
    #[cfg(feature = "tor")]
    TOR,
}

impl LinkListener {
    /// Creates a new `LinkListener`.
    pub fn new(listen_uri: &PeerURI) -> Result<Self, Error> {
        match listen_uri {
            PeerURI::TCP(addr) => Ok(Self::TCP(TCPListener::bind(*addr)?)),
            // PeerURI::SOCKS
            // PeerURI::TOR
            _ => unimplemented!(),
        }
    }
}

/// Produces a link upon each incoming connection.
impl Stream for LinkListener {
    type Item = (LinkInfo, LinkReader, LinkWriter);

    fn poll_next(self: Pin<&mut Self>, cx: &mut task::Context) -> task::Poll<Option<Self::Item>> {
        use task::Poll;

        match self.into_ref().get_ref() {
            Self::TCP(listener) => {
                let mut item = listener.accept().map_ok(|stream| {
                    let addr = stream.remote_addr().clone();
                    let info = LinkInfo::new(PeerURI::TCP(addr));
                    let (r, w) = stream.split();
                    (info, r, w)
                });
                futures::pin_mut!(item);
                match Future::poll(item, cx) {
                    Poll::Pending => Poll::Pending,
                    Poll::Ready(Ok(i)) => Poll::Ready(Some(i)),
                    Poll::Ready(Err(e)) => {
                        // TODO
                        Poll::Ready(None)
                    }
                }
            }
            _ => unimplemented!(),
        }
    }
}

/// Reads bytes from an established `Link`.
#[derive(Debug)]
pub enum LinkReader {
    TCP(io::ReadHalf<TCPStream>),
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
            Self::TCP(reader) => {
                futures::pin_mut!(reader);
                reader.poll_read(cx, buf)
            }
            _ => unimplemented!(),
        }
    }
}

/// Writes bytes to an established `Link`.
#[derive(Debug)]
pub enum LinkWriter {
    TCP(io::WriteHalf<TCPStream>),
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
            Self::TCP(writer) => {
                futures::pin_mut!(writer);
                writer.poll_write(cx, buf)
            }
            _ => unimplemented!(),
        }
    }

    #[inline]
    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), io::Error>> {
        match self.get_mut() {
            Self::TCP(writer) => {
                futures::pin_mut!(writer);
                writer.poll_flush(cx)
            }
            _ => unimplemented!(),
        }
    }

    #[inline]
    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), io::Error>> {
        match self.get_mut() {
            Self::TCP(writer) => {
                futures::pin_mut!(writer);
                writer.poll_close(cx)
            }
            _ => unimplemented!(),
        }
    }
}

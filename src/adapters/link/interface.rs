use super::{
    tcp::{TCPListener, TCPStream},
    LinkInfo,
};
use smol::Async;
use std::{hash, sync::Arc};
use stream_cancel::{StreamExt as _, Trigger, Tripwire};
use yggy_core::{dev::*, types::PeerURI};

/// Handle to an open hardware interface, listening for incoming connections.
#[derive(Debug)]
pub struct LinkHandle {
    // name: String,
    addr: PeerURI,
    stop: Trigger,
}

impl LinkHandle {
    pub fn new(
        addr: PeerURI,
    ) -> Result<(Self, impl Stream<Item = (PeerURI, LinkReader, LinkWriter)>), Error> {
        let (stop, stopped) = Tripwire::new();
        let listener = LinkListener::new(&addr)?;
        Ok((Self { addr, stop }, listener.take_until_if(stopped)))
    }
}

impl Eq for LinkHandle {}
impl PartialEq for LinkHandle {
    fn eq(&self, other: &Self) -> bool {
        self.addr == other.addr
    }
}
impl hash::Hash for LinkHandle {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.addr.hash(state);
    }
}

/// Reads bytes from an established `Link`.
#[derive(Debug)]
pub struct LinkReader {
    inner: LinkReaderInner,
}

impl LinkReader {
    // pub fn read
}

#[derive(Debug)]
pub(crate) enum LinkReaderInner {
    TCP(io::ReadHalf<TCPStream>),
    // SOCKS(io::ReadHalf<TCPStream>),
    // #[cfg(feature = "tor")]
    // TOR(io::ReadHalf<TCPStream>),
}

impl From<LinkReaderInner> for LinkReader {
    fn from(inner: LinkReaderInner) -> Self {
        Self { inner }
    }
}

impl AsyncRead for LinkReader {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &mut [u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        match &mut (self.get_mut().inner) {
            LinkReaderInner::TCP(reader) => {
                futures::pin_mut!(reader);
                reader.poll_read(cx, buf)
            }
            _ => unimplemented!(),
        }
    }
}

/// Writes bytes to an established `Link`.
#[derive(Debug)]
pub struct LinkWriter {
    inner: LinkWriterInner,
}

#[derive(Debug)]
pub(crate) enum LinkWriterInner {
    TCP(io::WriteHalf<TCPStream>),
    // SOCKS(io::WriteHalf<TCPStream>),
    // #[cfg(feature = "tor")]
    // TOR(io::WriteHalf<TCPStream>),
}

impl From<LinkWriterInner> for LinkWriter {
    fn from(inner: LinkWriterInner) -> Self {
        Self { inner }
    }
}

#[async_trait::async_trait]
impl Actor for LinkWriter {}

impl AsyncWrite for LinkWriter {
    #[inline]
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &[u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        match &mut (self.get_mut().inner) {
            LinkWriterInner::TCP(writer) => {
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
        match &mut (self.get_mut().inner) {
            LinkWriterInner::TCP(writer) => {
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
        match &mut (self.get_mut().inner) {
            LinkWriterInner::TCP(writer) => {
                futures::pin_mut!(writer);
                writer.poll_close(cx)
            }
            _ => unimplemented!(),
        }
    }
}

/// Listens for incoming `Link`s.
#[derive(Debug, Eq, Hash, PartialEq)]
pub enum LinkListener {
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

    ///
    pub async fn open(peer_uri: &PeerURI) -> Result<(LinkReader, LinkWriter), Error> {
        match peer_uri {
            PeerURI::TCP(addr) => Ok(TCPStream::connect(*addr).await?.split()),
            _ => unimplemented!(),
        }
    }
}

/// Produces a link upon each incoming connection.
impl Stream for LinkListener {
    type Item = (PeerURI, LinkReader, LinkWriter);
    fn poll_next(self: Pin<&mut Self>, cx: &mut task::Context) -> task::Poll<Option<Self::Item>> {
        use task::Poll;

        match self.into_ref().get_ref() {
            Self::TCP(listener) => {
                let mut item = listener.accept().map_ok(|stream| {
                    let addr = stream.remote_addr().clone();
                    let uri = PeerURI::TCP(addr);
                    let (r, w) = stream.split();
                    (uri, r, w)
                });
                futures::pin_mut!(item);
                match Future::poll(item, cx) {
                    Poll::Pending => Poll::Pending,
                    Poll::Ready(Ok(i)) => Poll::Ready(Some(i)),
                    Poll::Ready(Err(e)) => {
                        // TODO log
                        Poll::Ready(None)
                    }
                }
            }
            _ => unimplemented!(),
        }
    }
}

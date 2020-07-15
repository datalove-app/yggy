use super::{
    tcp::{TCPListener, TCPStream},
    LinkInfo,
};
use std::{hash, sync::Arc};
use stream_cancel::{StreamExt as _, Trigger, Tripwire};
use yggy_core::{dev::*, types::PeerURI};

#[derive(Debug)]
pub struct LinkInterface {
    info: LinkInfo,
    stop: Trigger,
}

impl LinkInterface {
    pub fn new(
        info: LinkInfo,
    ) -> Result<(Self, impl Stream<Item = (LinkInfo, LinkReader, LinkWriter)>), Error> {
        let listener = LinkListener::new(&info.addr)?;
        let (stop, stopped) = Tripwire::new();
        Ok((Self { info, stop }, listener.take_until_if(stopped)))
    }
}

impl Eq for LinkInterface {}
impl PartialEq<Self> for LinkInterface {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info
    }
}
impl hash::Hash for LinkInterface {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.info.hash(state);
    }
}

///
#[derive(Debug, Eq, Hash, PartialEq)]
enum LinkListener {
    TCP(TCPListener),
    SOCKS,
    #[cfg(feature = "tor")]
    TOR,
}

impl LinkListener {
    /// Creates a new interface.
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
                    let info = LinkInfo {
                        addr: PeerURI::TCP(stream.remote_addr().clone()),
                    };
                    let (r, w) = stream.split();
                    (info, r, w)
                });
                match item.now_or_never() {
                    Some(Ok(res)) => Poll::Ready(Some(res)),
                    Some(Err(e)) => {
                        // TODO
                        Poll::Ready(None)
                    }
                    _ => Poll::Pending,
                }
            }
            _ => unimplemented!(),
        }
    }
}

///
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

///
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

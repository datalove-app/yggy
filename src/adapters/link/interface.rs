use super::{udp::UDPSocket, Link, LinkAdapter, LinkInfo};
use smol::Async;
use yggy_core::{dev::*, types::PeerURI};

impl<C: Core> Link<C> {
    /// Starts a [`Link`] that reads and writes packets on the provided `PeerURI`.
    ///
    /// [`Link`]: ../mod/struct.Link.html
    pub async fn start_link(
        listen_uri: PeerURI,
        adapter: Addr<LinkAdapter<C>>,
    ) -> Result<Addr<Self>, Error> {
        let (reader, writer) = match listen_uri {
            // PeerURI::TCP
            PeerURI::UDP(addr) => UDPSocket::bind(addr)?.split(),
            // PeerURI::SOCKS
            // PeerURI::TOR
            _ => unimplemented!(),
        };

        let link = Link {
            info: LinkInfo { listen_uri },
            adapter,
            reader,
            writer,
        };

        Ok(Actor::start(link).await?)
    }
}

///
#[derive(Debug)]
pub enum LinkReader {
    // TCP(io::ReadHalf<TCPStream>),
    UDP(io::ReadHalf<UDPSocket>),
    // SOCKS(io::ReadHalf<TCPStream>),
    // TOR(io::ReadHalf<TCPStream>),
}

impl AsyncRead for LinkReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &mut [u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        match self.get_mut() {
            Self::UDP(reader) => {
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
    // TCP(io::WriteHalf<TCPStream>),
    UDP(io::WriteHalf<UDPSocket>),
    // SOCKS(io::WriteHalf<TCPStream>),
    // TOR(io::WriteHalf<TCPStream>),
}

impl AsyncWrite for LinkWriter {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &[u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        match self.get_mut() {
            Self::UDP(writer) => {
                futures::pin_mut!(writer);
                writer.poll_write(cx, buf)
            }
            _ => unimplemented!(),
        }
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), io::Error>> {
        match self.get_mut() {
            Self::UDP(writer) => {
                futures::pin_mut!(writer);
                writer.poll_flush(cx)
            }
            _ => unimplemented!(),
        }
    }

    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), io::Error>> {
        match self.get_mut() {
            Self::UDP(writer) => {
                futures::pin_mut!(writer);
                writer.poll_close(cx)
            }
            _ => unimplemented!(),
        }
    }
}

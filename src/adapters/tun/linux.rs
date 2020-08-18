use anyhow::anyhow;
use futures_locks::Mutex;
use smol::Async;
use std::fs::File;
use utuntap::tun::OpenOptions;
use yggy_core::{dev::*, interfaces::tun, types::MTU};

// const MAX_UDP_SIZE: usize = (1 << 16) - 1;

///
/// TODO rename
#[derive(Debug)]
pub struct Socket {
    name: String,
    // mtu: MTU,
    file: Option<Async<File>>,
    // src_buf: [u8; MAX_UDP_SIZE],
    // dst_buf: [u8; MAX_UDP_SIZE],
}

impl tun::TunInterface for Socket {
    type Reader = TunReader;
    type Writer = TunWriter;

    // TODO: get/set MTU?
    // TODO: support retries if name of socket is already taken?
    // TODO: support custom names?
    fn open() -> Result<Self, Error> {
        let (raw_file, name) = OpenOptions::new()
            .nonblock(true)
            .open()
            .map_err(ConnError::Interface)?;

        Ok(Self {
            name,
            // mtu: MTU::default(),
            file: Async::new(raw_file)
                .map(Some)
                .map_err(ConnError::Interface)?,
        })
    }

    #[inline]
    fn name(&self) -> &str {
        &self.name
    }

    // fn mtu(&self) -> MTU {
    //     unimplemented!()
    // }

    fn split(mut self) -> (Self::Reader, Self::Writer) {
        let (reader, writer) = (&mut self)
            .file
            .take()
            .map(|file| file.split())
            .expect("already initialized TUN socket");

        let socket_info = Mutex::new(self);
        let reader = TunReader {
            socket_info: socket_info.clone(),
            reader,
        };
        let writer = TunWriter {
            socket_info,
            writer,
        };

        (reader, writer)
    }
}

#[derive(Debug)]
pub struct TunReader {
    socket_info: Mutex<Socket>,
    reader: io::ReadHalf<Async<File>>,
}

#[async_trait::async_trait]
impl Actor for TunReader {
    // async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {}
}

impl AsyncRead for TunReader {
    ///
    /// ? see: iface.go:tunReader._read
    #[inline]
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &mut [u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        let reader = &mut self.reader;
        futures::pin_mut!(reader);
        reader.poll_read(cx, buf)
    }
}

#[derive(Debug)]
pub struct TunWriter {
    socket_info: Mutex<Socket>,
    writer: io::WriteHalf<Async<File>>,
}

#[async_trait::async_trait]
impl Actor for TunWriter {
    // async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {}
}

impl AsyncWrite for TunWriter {
    #[inline]
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &[u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        let writer = &mut self.writer;
        futures::pin_mut!(writer);
        writer.poll_write(cx, buf)
    }

    #[inline]
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), io::Error>> {
        let writer = &mut self.writer;
        futures::pin_mut!(writer);
        writer.poll_flush(cx)
    }

    #[inline]
    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), io::Error>> {
        let writer = &mut self.writer;
        futures::pin_mut!(writer);
        writer.poll_close(cx)
    }
}

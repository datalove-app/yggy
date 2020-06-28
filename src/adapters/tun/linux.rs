use anyhow::anyhow;
use futures::{io, prelude::*, task};
use smol::Async;
use std::{
    fs::File,
    pin::Pin,
    sync::{Arc, Mutex},
};
use utuntap::tun::OpenOptions;
use xactor::{Actor, Addr, Context};
use yggy_core::{error::Error, interfaces::tun, types::MTU};

// const MAX_UDP_SIZE: usize = (1 << 16) - 1;

///
///
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
            .map_err(|e| Error::Init(e.into()))?;

        Ok(Self {
            name,
            // mtu: MTU::default(),
            file: Async::new(raw_file)
                .map(Some)
                .map_err(|e| Error::Init(e.into()))?,
        })
    }

    fn name(&self) -> &str {
        &self.name
    }

    // fn mtu(&self) -> MTU {
    //     unimplemented!()
    // }

    fn split(mut self) -> Result<(Self::Reader, Self::Writer), Error> {
        let (reader, writer) = (&mut self)
            .file
            .take()
            .map(|file| file.split())
            .ok_or_else(|| Error::Init(anyhow!("already initialized TUN socket")))?;

        let socket_info = Arc::new(Mutex::from(self));
        Ok((
            TunReader {
                socket_info: socket_info.clone(),
                reader,
            },
            TunWriter {
                socket_info,
                writer,
            },
        ))
    }
}

#[derive(Debug)]
pub struct TunReader {
    socket_info: Arc<Mutex<Socket>>,
    reader: io::ReadHalf<Async<File>>,
}

#[async_trait::async_trait]
impl Actor for TunReader {
    // async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {}
}

impl AsyncRead for TunReader {
    ///
    /// ? see: iface.go:tunReader._read
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
    socket_info: Arc<Mutex<Socket>>,
    writer: io::WriteHalf<Async<File>>,
}

#[async_trait::async_trait]
impl Actor for TunWriter {
    // async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {}
}

impl AsyncWrite for TunWriter {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &[u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        let writer = &mut self.writer;
        futures::pin_mut!(writer);
        writer.poll_write(cx, buf)
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), io::Error>> {
        let writer = &mut self.writer;
        futures::pin_mut!(writer);
        writer.poll_flush(cx)
    }

    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), io::Error>> {
        let writer = &mut self.writer;
        futures::pin_mut!(writer);
        writer.poll_close(cx)
    }
}

use super::TunSocket;
use crate::{core_types::MTU, error::Error};
use futures::{
    io::{self, AsyncRead, AsyncReadExt, AsyncWrite},
    task,
};
use smol::Async;
use std::{
    fs::File,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
};
use utuntap::tun::OpenOptions;
use xactor::{Actor, Addr, Context as ActorContext};

// const MAX_UDP_SIZE: usize = (1 << 16) - 1;

///
///
#[derive(Debug)]
pub struct Socket {
    name: String,
    mtu: MTU,
    file: Option<Async<File>>,
    // src_buf: [u8; MAX_UDP_SIZE],
    // dst_buf: [u8; MAX_UDP_SIZE],
}

impl TunSocket for Socket {
    type Reader = TunReader;
    type Writer = TunWriter;

    // TODO: set MTU
    // TODO: support retries if name is taken?
    fn open(mtu: MTU) -> Result<Self, Error> {
        let (raw_file, name) = OpenOptions::new()
            .open()
            .map_err(|e| Error::Init(format!("{}", e)))?;
        let file = Async::new(raw_file)
            .map(Some)
            .map_err(|e| Error::Init(format!("{}", e)))?;

        Ok(Self { name, mtu, file })
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
            .ok_or_else(|| Error::Init("already initialized TUN socket".into()))?;

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

impl AsyncRead for TunReader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &mut [u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        unimplemented!()
    }
}

#[derive(Debug)]
pub struct TunWriter {
    socket_info: Arc<Mutex<Socket>>,
    writer: io::WriteHalf<Async<File>>,
}

#[async_trait::async_trait]
impl Actor for TunWriter {
    async fn started(&mut self, ctx: &ActorContext<Self>) {}
}

impl AsyncWrite for TunWriter {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &[u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        unimplemented!()
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), io::Error>> {
        unimplemented!()
    }

    fn poll_close(
        self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), io::Error>> {
        unimplemented!()
    }
}

use crate::{core_interfaces::tun::TunDevice as ITunDevice, core_types::MTU};
use futures::io::{self, AsyncRead, AsyncWrite};
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tun::Device;

const MAX_UDP_SIZE: usize = (1 << 16) - 1;

///
/// TODO support listening on multiple IPs
#[derive(Debug, Default)]
pub struct TunDevice {
    // iface: Device,
// src_buf: [u8; MAX_UDP_SIZE],
// dst_buf: [u8; MAX_UDP_SIZE],
}

impl AsyncRead for TunDevice {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &mut [u8],
    ) -> Poll<Result<usize, io::Error>> {
        unimplemented!()
    }
}

impl AsyncWrite for TunDevice {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        unimplemented!()
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
        unimplemented!()
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), io::Error>> {
        unimplemented!()
    }
}

impl ITunDevice for TunDevice {
    fn name(&self) -> &str {
        unimplemented!()
    }

    fn mtu(&self) -> MTU {
        unimplemented!()
    }
}

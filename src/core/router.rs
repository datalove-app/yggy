use crate::{
    core_interfaces::{peer, router, Core},
    core_types::wire,
};
use futures::{io, prelude::*};
use std::{pin::Pin, task};
use xactor::{Actor, Addr, Context, Handler, StreamHandler};

type IPeer<C> = <<C as Core>::PeerManager as peer::PeerManager<C>>::Peer;

///
#[derive(Debug)]
pub struct Router<C: Core> {
    ///
    core: Addr<C>,

    ///
    self_peer: Addr<IPeer<C>>,

    ///
    writer: RouterWriter<C>,
}

impl<C: Core> router::Router<C> for Router<C> {
    // type Interface = RouterInterface<C>;

    fn reconfigure(&mut self) {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<C: Core> Actor for Router<C> {
    async fn started(&mut self, ctx: &Context<Self>) {}
}

#[async_trait::async_trait]
impl<C: Core> StreamHandler<wire::Traffic> for Router<C> {
    async fn handle(&mut self, ctx: &Context<Self>, msg: wire::Traffic) {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<C: Core> StreamHandler<wire::ProtocolTraffic> for Router<C> {
    async fn handle(&mut self, ctx: &Context<Self>, msg: wire::ProtocolTraffic) {
        unimplemented!()
    }
}

///
/// TODO
#[derive(Debug)]
pub struct RouterInterface<C: Core> {
    router: Addr<Router<C>>,
}

#[async_trait::async_trait]
impl<C: Core> peer::PeerInterface for RouterInterface<C> {
    type Reader = Unreadable;
    type Writer = RouterWriter<C>;
}

///
/// TODO
#[derive(Debug)]
pub struct RouterWriter<C: Core> {
    intf: RouterInterface<C>,
}

impl<C: Core> AsyncWrite for RouterWriter<C> {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &[u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        unimplemented!()
    }

    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), io::Error>> {
        unimplemented!()
    }

    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), io::Error>> {
        unimplemented!()
    }
}

///
#[derive(Debug)]
pub struct Unreadable;

impl AsyncRead for Unreadable {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &mut [u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        unreachable!()
    }
}

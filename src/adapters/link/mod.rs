mod udp;

use crate::{
    core_interfaces::{link, peer, Core},
    core_types::{BoxPublicKey, PeerURI, SigningPublicKey},
};
use futures::{io, prelude::*};
use std::{collections::HashMap, hash, pin::Pin, task, time::Duration};
use xactor::{Actor, Addr, Context, Handler};

pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(6);

lazy_static! {
    ///
    pub static ref PING_INTERVAL: Duration = (DEFAULT_TIMEOUT * 2) / 3;
}

type IPeer<C> = <<C as Core>::PeerManager as peer::PeerManager<C>>::Peer;

///
#[derive(Debug)]
pub struct LinkInfo {
    pub_box_key: BoxPublicKey,
    pub_sign_key: SigningPublicKey,
    uri: PeerURI,
    // local: String,
    // remote: String,
}

// TODO
impl hash::Hash for LinkInfo {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.uri.hash(state);
    }
}

///
#[derive(Debug)]
pub struct LinkManager<C: Core> {
    ///
    core: Addr<C>,

    ///
    links: HashMap<LinkInfo, Addr<Link<C>>>,
}

#[async_trait::async_trait]
impl<C: Core> link::LinkManager<C> for LinkManager<C> {
    fn reconfigure(&mut self) {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<C: Core> Actor for LinkManager<C> {}

#[async_trait::async_trait]
impl<C: Core> Handler<link::messages::Listen> for LinkManager<C> {
    async fn handle(&mut self, ctx: &Context<Self>, msg: link::messages::Listen) {
        unimplemented!()
    }
}

///
#[derive(Debug)]
pub struct Link<C: Core> {
    ///
    link_manager: Addr<LinkManager<C>>,

    ///
    peer: Addr<IPeer<C>>,

    ///
    reader: LinkReader<C>,
    ///
    writer: LinkWriter<C>,
}

#[async_trait::async_trait]
impl<C: Core> link::Link<C, LinkManager<C>> for Link<C> {}

#[async_trait::async_trait]
impl<C: Core> peer::PeerInterface for Link<C> {
    type Reader = LinkReader<C>;
    type Writer = LinkWriter<C>;
}

#[async_trait::async_trait]
impl<C: Core> Actor for Link<C> {
    async fn started(&mut self, ctx: &Context<Self>) {}
}

#[async_trait::async_trait]
impl<C: Core> Handler<link::messages::Notification> for Link<C> {
    async fn handle(&mut self, ctx: &Context<Self>, msg: link::messages::Notification) {
        unimplemented!()
    }
}

///
#[derive(Debug)]
pub struct LinkReader<C: Core> {
    link: Addr<Link<C>>,
}

impl<C: Core> AsyncRead for LinkReader<C> {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
        buf: &mut [u8],
    ) -> task::Poll<Result<usize, io::Error>> {
        unimplemented!()
    }
}

///
#[derive(Debug)]
pub struct LinkWriter<C: Core> {
    link: Addr<Link<C>>,
}

impl<C: Core> AsyncWrite for LinkWriter<C> {
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

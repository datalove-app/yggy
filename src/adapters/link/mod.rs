mod udp;

use crate::{
    core_interfaces::{link, peer, Core},
    core_types::{BoxPublicKey, PeerURI, SigningPublicKey},
    error::Error,
};
use futures::{io, prelude::*, task};
use std::{collections::HashMap, hash, pin::Pin, time::Duration};
use xactor::{Actor, Addr, Context, Handler};

lazy_static! {
    ///
    pub static ref PING_INTERVAL: Duration = (DEFAULT_TIMEOUT * 2) / 3;
}

///
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(6);

type IPeer<C> = <<C as Core>::PeerManager as peer::PeerManager<C>>::Peer;
type Links<C> = HashMap<LinkInfo, Addr<Link<C>>>;

///
#[derive(Debug)]
pub struct LinkInfo {
    /// The URI and type of link.
    uri: PeerURI,

    /// The linked node's signing public key.
    signing_pub_key: SigningPublicKey,

    /// The linked node's encryption public key.
    box_pub_key: BoxPublicKey,
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
pub struct LinkAdapter<C: Core> {
    ///
    core: Addr<C>,

    ///
    links: Links<C>,
}

impl<C: Core> LinkAdapter<C> {
    #[inline]
    pub async fn start(core: Addr<C>) -> Result<Addr<Self>, Error> {
        let mut adapter = Self {
            core,
            links: HashMap::default(),
        };

        Ok(Actor::start(adapter).await?)
    }
}

#[async_trait::async_trait]
impl<C: Core> link::LinkAdapter<C> for LinkAdapter<C> {
    fn reconfigure(&mut self) {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<C: Core> Actor for LinkAdapter<C> {
    async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {
        let config = C::current_config(&mut self.core).await?;

        // TODO initialize links
        for listen_uri in config.listen_addrs.into_iter() {
            // bind to socket
        }

        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<C: Core> Handler<link::messages::Listen> for LinkAdapter<C> {
    async fn handle(&mut self, ctx: &Context<Self>, msg: link::messages::Listen) {
        unimplemented!()
    }
}

///
#[derive(Debug)]
pub struct Link<C: Core> {
    ///
    info: LinkInfo,

    ///
    adapter: Addr<LinkAdapter<C>>,

    ///
    peer: Addr<IPeer<C>>,

    ///
    reader: LinkReader<C>,

    ///
    writer: LinkWriter<C>,
}

#[async_trait::async_trait]
impl<C: Core> link::Link<C, LinkAdapter<C>> for Link<C> {}

#[async_trait::async_trait]
impl<C: Core> peer::PeerInterface for Link<C> {
    type Reader = LinkReader<C>;
    type Writer = LinkWriter<C>;
}

#[async_trait::async_trait]
impl<C: Core> Actor for Link<C> {
    async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {
        unimplemented!()
    }
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

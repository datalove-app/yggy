mod interface;
mod udp;

use self::interface::{LinkReader, LinkWriter};
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkInfo {
    /// The URI and type of link.
    uri: PeerURI,
    // /// The linked node's signing public key.
    // signing_pub_key: SigningPublicKey,

    // /// The linked node's encryption public key.
    // box_pub_key: BoxPublicKey,
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
    /// Starts the `LinkAdapter`, opening `Link`s for each address listed in [`ListenAddresses`].
    ///
    /// [`ListenAddresses`]: ../../core_types/struct.ListenAddresses.html
    #[inline]
    pub async fn start(core: Addr<C>) -> Result<Addr<Self>, Error> {
        let adapter = Self {
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

    // ///
    // peer: Addr<IPeer<C>>,
    ///
    reader: LinkReader,

    ///
    writer: LinkWriter,
}

#[async_trait::async_trait]
impl<C: Core> link::Link<C, LinkAdapter<C>> for Link<C> {}

#[async_trait::async_trait]
impl<C: Core> peer::PeerInterface for Link<C> {
    type Reader = LinkReader;
    type Writer = LinkWriter;
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

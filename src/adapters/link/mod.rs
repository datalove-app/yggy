mod interface;
mod udp;

use self::interface::{LinkReader, LinkWriter};
use std::{collections::HashMap, hash, time::Duration};
use yggy_core::{
    dev::*,
    interfaces::{link, peer},
    types::{BoxPublicKey, PeerURI, SigningPublicKey},
};

lazy_static! {
    ///
    static ref PING_INTERVAL: Duration = (DEFAULT_TIMEOUT * 2) / 3;

    // /// Time to wait before closing the link.
    // static ref CLOSE_TIMEOUT: Duration = ROOT_TIMEOUT * 2;
}

///
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(6);

/// Time to wait before sending a keep-alive message if we have no real traffic
/// to send.
const KEEP_ALIVE_TIMEOUT: Duration = Duration::from_secs(2);

/// Time to wait before deciding a send is blocked.
const SEND_TIMEOUT: Duration = Duration::from_secs(1);

///
const STALL_TIMEOUT: Duration = Duration::from_secs(6);

type IPeer<C> = <<C as Core>::PeerManager as peer::PeerManager<C>>::Peer;
type Links<C> = HashMap<LinkInfo, Addr<Link<C>>>;

///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkInfo {
    /// The URI and type of link.
    listen_uri: PeerURI,
    // /// The linked node's signing public key.
    // signing_pub_key: Once<SigningPublicKey>,

    // /// The linked node's encryption public key.
    // box_pub_key: Once<BoxPublicKey>,
}

// TODO
impl hash::Hash for LinkInfo {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.listen_uri.hash(state);
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
            links: Default::default(),
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

        // initialize links
        for uri in config.listen_addrs.into_iter() {
            let adapter = ctx.address();
            let link_info = LinkInfo {
                listen_uri: uri.clone(),
            };
            let link_addr = Link::start_link(uri, adapter).await?;
            (&mut self.links).insert(link_info, link_addr);
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
    adapter: Addr<LinkAdapter<C>>,

    ///
    info: LinkInfo,

    // ///
    // peer: Addr<IPeer<C>>,
    ///
    reader: LinkReader,

    ///
    writer: LinkWriter,
}

impl<C: Core> Link<C> {
    // #[inline]
    // pub fn info(&self) -> &LinkInfo {
    //     &self.info
    // }
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

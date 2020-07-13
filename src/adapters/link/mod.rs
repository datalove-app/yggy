mod interface;
mod tcp;
mod udp;

use self::interface::{LinkInterface, LinkReader, LinkWriter};
use futures::stream;
use std::{
    collections::{HashMap, HashSet},
    hash,
    sync::Arc,
    time::Duration,
};
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
type Interfaces = HashSet<Arc<LinkInterface>>;

///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkInfo {
    /// The non-protocol URI for communicating with the linked peer.
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

    ///
    interfaces: Interfaces,
}

impl<C: Core> LinkAdapter<C> {
    /// Starts the `LinkAdapter`, opening `Link`s for each address listed in [`ListenAddresses`].
    ///
    /// [`ListenAddresses`]: ../../core_types/struct.ListenAddresses.html
    #[inline]
    pub async fn start(core: Addr<C>) -> Result<Addr<Self>, Error> {
        Ok(Actor::start(Self {
            core,
            links: Default::default(),
            interfaces: Default::default(),
        })
        .await?)
    }

    /// Starts a listener for incoming connections.
    pub fn listen(&mut self, listen_uri: PeerURI, ctx: &Context<Self>) -> Result<(), Error> {
        let info = Arc::new(LinkInfo { listen_uri });
        let interface = Arc::new(LinkInterface::new(info.clone())?);
        (&mut self.interfaces).insert(interface.clone());

        // TODO start the link
        // ctx.add_stream(mut stream: S)

        Ok(())
    }

    /// Opens a `Link` to an outbound peer.
    pub async fn open(&mut self, self_addr: Addr<Self>, peer_uri: PeerURI) -> Result<(), Error> {
        // let link = Link::start(info.clone(), self_addr).await?;
        // (&mut self.links).insert(info, link);
        unimplemented!()
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

        // initialize links for incoming connections
        // TODO do these in parallel
        for listen_uri in config.listen_addrs.iter() {
            // self.listen(ctx, listen_uri.clone());
            // self.listen(ctx.address(), listen_uri.clone()).await?;
        }

        // initialize links for outgoing connections
        // TODO do these in parallel
        // TODO set a timer to attempt to add peers from config
        // for peer_uri in config.peers.iter() {
        //     self.open(ctx.address(), peer_uri.clone()).await?;
        // }
        // for peer_uri in config.peers_by_interface.iter() {
        //     self.open(peer_uri.clone()).await?;
        // }

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
    info: LinkInfo,
    // ///
    // peer: Addr<IPeer<C>>,
    // ///
    // interface: LinkInterface,
    ///
    reader: LinkReader,
    ///
    writer: LinkWriter,
}

impl<C: Core> Link<C> {
    /// Starts a `Link` that reads and writes packets on the provided [`PeerURI`].
    ///
    /// [`PeerURI`]:
    pub async fn start(
        info: LinkInfo,
        adapter: Addr<LinkAdapter<C>>,
        reader: LinkReader,
        writer: LinkWriter,
    ) -> Result<Addr<Self>, Error> {
        // let (reader, writer) = interface.listen(info.listen_uri.clone());

        let link = Link {
            info,
            // peer: IPeer<C> as Peer<C>
            adapter,
            // interface,
            reader,
            writer,
        };

        Ok(Actor::start(link).await?)
    }
}

#[async_trait::async_trait]
impl<C: Core> link::Link<C, LinkAdapter<C>> for Link<C> {}

// #[async_trait::async_trait]
// impl<C: Core> peer::PeerInterface for Link<C> {
//     type Reader = LinkReader;
//     type Writer = LinkWriter;
// }

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

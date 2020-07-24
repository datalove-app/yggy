mod interface;
mod tcp;

use self::interface::{LinkInterface, LinkReader, LinkWriter};
use futures::future::{select, Either};
use smol::Timer;
use std::{
    collections::{HashMap, HashSet},
    hash, io,
    sync::Arc,
    time::Duration,
};
use yggy_core::{
    dev::*,
    interfaces::{link, peer},
    types::{BoxKeypair, BoxPublicKey, PeerURI, SigningPublicKey},
    version::{Metadata, MetadataKeys},
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
type Interfaces = HashSet<LinkInterface>;

///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkInfo {
    /// The non-protocol URI for communicating with the linked peer.
    addr: PeerURI,

    /// The linked node's signing public key.
    signing_pub_key: Option<SigningPublicKey>,

    /// The linked node's encryption public key.
    box_pub_key: Option<BoxPublicKey>,
}

impl LinkInfo {
    pub const fn new(addr: PeerURI) -> Self {
        LinkInfo {
            addr,
            signing_pub_key: None,
            box_pub_key: None,
        }
    }
}

// TODO
impl hash::Hash for LinkInfo {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.addr.hash(state);
    }
}

///
#[derive(Debug)]
pub struct LinkAdapter<C: Core> {
    core: Addr<C>,

    /// Our opened and connected `Link`s.
    links: Links<C>,

    /// Our opened and connected [`LinkInterface`]s.
    ///
    /// [`LinkInterface`]: ./interfaces/struct.LinkInterface.html
    interfaces: Interfaces,
}

impl<C: Core> LinkAdapter<C> {
    /// Starts the `LinkAdapter`, opening [`LinkInterface`]s for each interface
    /// address listed in [`ListenAddresses`].
    ///
    /// [`LinkInterface`]: ./interfaces/struct.Link.html
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
    pub async fn accept(
        &mut self,
        ctx: &Context<Self>,
        info: LinkInfo,
        reader: LinkReader,
        writer: LinkWriter,
    ) -> Result<(), Error> {
        let config = C::current_config(&mut self.core).await?;
        let link = Link::start(
            ctx.address(),
            config.encryption_public_key.clone(),
            config.signing_public_key,
            info.clone(),
            reader,
            writer,
        )
        .await?;
        (&mut self.links).insert(info, link);
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

/// Initializes listeners for incoming `Link`s and configured outgoing `Link`s.
#[async_trait::async_trait]
impl<C: Core> Actor for LinkAdapter<C> {
    async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {
        let config = C::current_config(&mut self.core).await?;

        // initialize links for incoming connections
        for listen_uri in config.listen_addrs.iter() {
            let (handle, listener) = LinkInterface::new(listen_uri.clone())?;
            (&mut self.interfaces).insert(handle);
            ctx.add_stream(listener);
        }

        // initialize links for peers
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

// #[async_trait::async_trait]
// impl<C: Core> Handler<link::messages::Listen> for LinkAdapter<C> {
//     async fn handle(&mut self, ctx: &Context<Self>, msg: link::messages::Listen) {
//         // self.listen(msg.addr, ctx);
//         unimplemented!()
//     }
// }

/// Handles new incoming `Link`s.
#[async_trait::async_trait]
impl<C: Core> StreamHandler<(LinkInfo, LinkReader, LinkWriter)> for LinkAdapter<C> {
    #[inline]
    async fn handle(&mut self, ctx: &Context<Self>, msg: (LinkInfo, LinkReader, LinkWriter)) {
        // TODO handle result
        self.accept(ctx, msg.0, msg.1, msg.2).await;
    }

    async fn finished(&mut self, ctx: &Context<Self>) {}
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
        adapter: Addr<LinkAdapter<C>>,
        our_box_pub_key: BoxPublicKey,
        our_signing_pub_key: SigningPublicKey,
        info: LinkInfo,
        mut reader: LinkReader,
        mut writer: LinkWriter,
    ) -> Result<Addr<Self>, Error> {
        let link_keypair = BoxKeypair::new();
        let mut our_meta = Metadata::default();
        (&mut our_meta).keys = Some(MetadataKeys {
            r#box: our_box_pub_key,
            sig: our_signing_pub_key,
            link: link_keypair.public.clone(),
        });
        Self::trade_meta(our_meta, &mut reader, &mut writer).await?;

        let link = Link {
            info,
            // peer: IPeer<C> as Peer<C>
            adapter,
            reader,
            writer,
        };
        Ok(Actor::start(link).await?)
    }

    async fn trade_meta(
        our_meta: Metadata,
        reader: &mut LinkReader,
        writer: &mut LinkWriter,
    ) -> Result<Metadata, Error> {
        // send meta bytes or timeout
        let timeout = Timer::after(Duration::from_secs(30));
        match select(Metadata::sink(writer).send(our_meta), timeout).await {
            Either::Left((Ok(_), _)) => (),
            Either::Left((Err(e), _)) => Err(e)?,
            Either::Right((_, _)) => Err(ConnError::Link("timed out sending metadata"))?,
        };

        // recv meta bytes or timeout
        // TODO? check against allowed keys?
        let timeout = Timer::after(Duration::from_secs(30));
        let meta = match select(Metadata::stream(reader).try_next(), timeout).await {
            Either::Left((Ok(Some(meta)), _)) => meta,
            Either::Left((Err(e), _)) => Err(e)?,
            Either::Left((Ok(None), _)) => Err(ConnError::Link("connection yielded nothing"))?,
            Either::Right((_, _)) => Err(ConnError::Link("timed out receiving metadata"))?,
        };

        if meta > CURRENT_METADATA {
            return Err(ConnError::Link("failed to connect: wrong version"))?;
        }

        Ok(meta)
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

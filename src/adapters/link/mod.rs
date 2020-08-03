mod interface;
mod tcp;

use self::interface::{LinkHandle, LinkReader, LinkWriter};
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
    interfaces::{
        link::{self, messages},
        peer,
    },
    types::{BoxKeypair, BoxPublicKey, PeerURI, SigningPublicKey},
    version::{Metadata, MetadataKeys},
};

lazy_static! {
    /// TODO?
    static ref PING_INTERVAL: Duration = (DEFAULT_TIMEOUT * 2) / 3;

    // /// Time to wait before closing the link.
    // static ref CLOSE_TIMEOUT: Duration = ROOT_TIMEOUT * 2;
}

/// TODO?
const DEFAULT_TIMEOUT: Duration = Duration::from_secs(6);

/// Time to wait before sending a keep-alive message if we have no real traffic
/// to send.
const KEEP_ALIVE_TIMEOUT: Duration = Duration::from_secs(2);

/// Time to wait before deciding a send is blocked.
const SEND_TIMEOUT: Duration = Duration::from_secs(1);

///
const STALL_TIMEOUT: Duration = Duration::from_secs(6);

type IPeer<C> = <<C as Core>::PeerManager as peer::PeerManager<C>>::Peer;
type Links<C> = HashMap<Arc<LinkInfo>, Addr<Link<C>>>;
type LinkHandles = HashSet<LinkHandle>;

///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LinkInfo {
    /// The non-protocol URI of the remote peer.
    remote_addr: PeerURI,

    /// The linked peer's signing public key.
    signing_pub_key: Option<SigningPublicKey>,

    /// The linked peer's encryption public key.
    box_pub_key: Option<BoxPublicKey>,
}

impl LinkInfo {
    pub const fn new(remote_addr: PeerURI) -> Self {
        LinkInfo {
            remote_addr,
            signing_pub_key: None,
            box_pub_key: None,
        }
    }

    /// Sets the keys in the `LinkInfo`.
    ///
    /// # Panics
    ///
    /// Panics if the metadata keys have not been set.
    pub fn set_meta(&mut self, keys: MetadataKeys) {
        let MetadataKeys { r#box, sig, link } = keys;
        self.signing_pub_key.replace(sig);
        self.box_pub_key.replace(r#box);
    }
}

// TODO
impl hash::Hash for LinkInfo {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.remote_addr.hash(state);
    }
}

///
#[derive(Debug)]
pub struct LinkAdapter<C: Core> {
    core: Addr<C>,

    /// Our opened and connected `Link`s.
    links: Links<C>,

    /// Our opened [`LinkHandle`]s, awaiting connections to new `Link`s.
    ///
    /// [`LinkHandle`]: ./interfaces/struct.LinkHandle.html
    handles: LinkHandles,
}

impl<C: Core> LinkAdapter<C> {
    /// Starts the `LinkAdapter`, opening [`LinkHandle`]s for each interface
    /// address listed in [`ListenAddresses`].
    ///
    /// [`LinkHandle`]: ./interfaces/struct.Link.html
    /// [`ListenAddresses`]: ../../core_types/struct.ListenAddresses.html
    #[inline]
    pub async fn start(core: Addr<C>) -> Result<Addr<Self>, Error> {
        Ok(Actor::start(Self {
            core,
            links: Default::default(),
            handles: Default::default(),
        })
        .await?)
    }

    /// Accepts an incoming connection.
    async fn accept(
        &mut self,
        ctx: &Context<Self>,
        info: LinkInfo,
        reader: LinkReader,
        writer: LinkWriter,
    ) -> Result<(), Error> {
        let config = C::current_config(&mut self.core).await?;
        let (initialized_info, link) =
            Link::start(ctx.address(), config, info, reader, writer).await?;
        (&mut self.links).insert(initialized_info, link);
        Ok(())
    }

    /// Opens a `Link` to an outbound peer.
    async fn open(&mut self, self_addr: Addr<Self>, peer_uri: PeerURI) -> Result<(), Error> {
        // let link = Link::start(info.clone(), self_addr).await?;
        // (&mut self.links).insert(info, link);
        unimplemented!()
    }

    /// Closes a `Link` to a linked peer.
    async fn close(&mut self, remote_addr: &PeerURI) -> Result<(), Error> {
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
            let (handle, listener) = LinkHandle::new(listen_uri.clone())?;
            (&mut self.handles).insert(handle);
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

/// Represents an active, direct, link to another peer established via a `LinkHandle`.
#[derive(Debug)]
pub struct Link<C: Core> {
    adapter: Addr<LinkAdapter<C>>,
    info: Arc<LinkInfo>,

    ///
    peer: Addr<IPeer<C>>,

    // ///
    // interface: LinkHandle,
    ///
    reader: LinkReader,
    ///
    writer: Addr<LinkWriter>,
}

impl<C: Core> Link<C> {
    /// Starts a `Link` that reads and writes packets on the provided [`PeerURI`].
    ///
    /// [`PeerURI`]:
    pub async fn start(
        adapter: Addr<LinkAdapter<C>>,
        config: Arc<Config>,
        mut info: LinkInfo,
        mut reader: LinkReader,
        mut writer: LinkWriter,
    ) -> Result<(Arc<LinkInfo>, Addr<Self>), Error> {
        let link_keypair = BoxKeypair::new();
        let our_meta = Metadata::new(
            config.encryption_public_key.clone(),
            config.signing_public_key,
            link_keypair.public.clone(),
        );

        let peer = Self::init(config, our_meta, &mut info, &mut reader, &mut writer).await?;

        let info = Arc::from(info);
        let writer = Actor::start(writer).await?;
        let mut link = Link {
            info: info.clone(),
            peer,
            adapter,
            reader,
            writer,
        };

        Ok((info, Actor::start(link).await?))
    }

    /// Initializes the peer, performing the initial handshake, key validation,
    /// and peer creation.
    async fn init(
        config: Arc<Config>,
        our_meta: Metadata,
        info: &mut LinkInfo,
        reader: &mut LinkReader,
        writer: &mut LinkWriter,
    ) -> Result<Addr<IPeer<C>>, Error> {
        // send meta bytes or timeout
        match select(
            Metadata::sink(writer).send(our_meta),
            Timer::after(Duration::from_secs(30)),
        )
        .await
        {
            Either::Left((Ok(_), _)) => (),
            Either::Left((Err(e), _)) => Err(e)?,
            Either::Right((_, _)) => Err(ConnError::Link("timed out sending metadata"))?,
        };

        // recv meta bytes or timeout
        let meta = match select(
            Metadata::stream(reader).try_next(),
            Timer::after(Duration::from_secs(30)),
        )
        .await
        {
            Either::Left((Ok(Some(meta)), _)) => meta,
            Either::Left((Err(e), _)) => Err(e)?,
            Either::Left((Ok(None), _)) => Err(ConnError::Link("connection yielded nothing"))?,
            Either::Right((_, _)) => Err(ConnError::Link("timed out receiving metadata"))?,
        };

        if meta > CURRENT_METADATA {
            return Err(ConnError::Link("failed to connect: wrong version"))?;
        }

        // TODO? check against allowed keys
        // FIXME: compare signatures

        // TODO check if we have already have a link to the node
        info.set_meta(meta.keys.expect("metadata keys were not received"));

        // TODO init peer in peermanager

        // TODO ctx.add_stream of reader events

        // TODO establish timers

        unimplemented!()
    }

    #[inline]
    fn notify(link: &mut Addr<Self>, msg: messages::Notification) -> Result<(), Error> {
        Ok(link
            .send(msg)
            .map_err(|_| ConnError::Link("failed to notify link"))?)
    }
}

#[async_trait::async_trait]
impl<C: Core> link::Link<C, LinkAdapter<C>> for Link<C> {}

#[async_trait::async_trait]
impl<C: Core> Actor for Link<C> {
    // async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {
    //     unimplemented!()
    // }
}

#[async_trait::async_trait]
impl<C: Core> link::LinkInterface for Link<C> {
    // type Reader = LinkReader;
    // type Writer = LinkWriter;

    fn out(intf: Addr<Self>) {
        // intf.send()
    }

    fn link_out(intf: Addr<Self>) {}

    fn close(intf: Addr<Self>) {}

    fn name(&self) -> &str {
        unimplemented!()
    }

    fn local(&self) -> &PeerURI {
        unimplemented!()
    }

    fn remote(&self) -> &PeerURI {
        unimplemented!()
    }

    fn interface_type(&self) -> &str {
        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<C: Core> Handler<messages::Notification> for Link<C> {
    async fn handle(&mut self, ctx: &Context<Self>, msg: link::messages::Notification) {
        unimplemented!()
    }
}

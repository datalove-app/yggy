mod interface;
mod tcp;

use self::interface::*;
use futures::future::{join_all, select, Either};
use futures_locks::{Mutex, RwLock};
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
    types::{BoxKeypair, BoxPublicKey, BoxSecretKey, BoxSharedKey, PeerURI, SigningPublicKey},
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

type IPeer<C> = <IPeerManager<C> as peer::PeerManager<C>>::Peer;
type IPeerManager<C> = <C as Core>::PeerManager;
type Links<C> = HashMap<Arc<LinkInfo>, Addr<Link<C>>>;
// TODO make a hashmap HashMap<String, LinkHandle>
type LinkHandles = HashSet<LinkHandle>;
type LinkCalls = HashSet<PeerURI>;

///
#[derive(Debug)]
pub struct LinkInfo {
    /// The non-protocol URI of the remote peer.
    peer_uri: PeerURI,

    /// The linked peer's signing public key.
    sig: SigningPublicKey,

    /// The linked peer's encryption public key.
    r#box: BoxPublicKey,

    /// Link-specific secret key (ours) and public key (theirs).
    link: BoxKeypair,
}

impl Eq for LinkInfo {}
impl PartialEq for LinkInfo {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        return self.peer_uri == other.peer_uri
            && self.sig == other.sig
            && self.r#box == other.r#box;
    }
}

// TODO
impl hash::Hash for LinkInfo {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.peer_uri.hash(state);
    }
}

///
#[derive(Debug)]
pub struct LinkManager<C: Core> {
    core: Addr<C>,

    /// Our opened and connected `Link`s.
    links: RwLock<Links<C>>,

    /// Our opened [`LinkHandle`]s, awaiting connections to new `Link`s.
    ///
    /// [`LinkHandle`]: ./interfaces/struct.LinkHandle.html
    handles: RwLock<LinkHandles>,

    ///
    calls: RwLock<LinkCalls>,
}

#[async_trait::async_trait]
impl<C: Core> link::LinkManager<C> for LinkManager<C> {
    type Link = Link<C>;

    fn reconfigure(&mut self) {
        unimplemented!()
    }
}

// Public methods.
impl<C: Core> LinkManager<C> {
    /// Starts the `LinkManager`, opening [`LinkHandle`]s for each interface
    /// address listed in [`ListenAddresses`].
    ///
    /// [`LinkHandle`]: ./interfaces/struct.Link.html
    /// [`ListenAddresses`]: ../../core_types/struct.ListenAddresses.html
    pub async fn new(core: Addr<C>) -> Result<Arc<Self>, Error> {
        let mut manager = Arc::from(Self {
            core,
            links: Default::default(),
            handles: Default::default(),
            calls: Default::default(),
        });

        manager.init().await?;
        Ok(manager)
    }
}

// Internal methods.
impl<C: Core> LinkManager<C> {
    async fn init(self: &Arc<Self>) -> Result<(), Error> {
        let mut core = self.core.clone();
        let config = C::current_config(&mut core).await?;

        // initialize links for incoming connections
        for listen_uri in config.listen_addrs.iter() {
            let (handle, mut listener) = LinkHandle::new(listen_uri.clone())?;
            self.handles.write().await.insert(handle);

            let self_ = Arc::clone(&self);
            spawn(async move {
                while let Some((uri, r, w)) = listener.next().await {
                    // TODO? handle error
                    self_.accept(uri, r, w).await;
                }
            });
            // spawn(listener.for_each(|(uri, r, w)| self_.accept(uri, r, w))).await;
        }

        // initialize links for pre-configured peers
        // TODO set a timer to attempt to add peers from config
        join_all(
            config
                .peers
                .iter()
                // .chain(config.peers_by_interface.iter())
                .map(move |peer_uri| self.open(peer_uri.clone())),
        )
        .await;

        Ok(())
    }

    /// Accepts an incoming connection, creating a new `Link`.
    async fn accept(
        self: &Arc<Self>,
        peer_uri: PeerURI,
        r: LinkReader,
        w: LinkWriter,
    ) -> Result<(), Error> {
        let (info, link) = Link::start(self, peer_uri, r, w, true).await?;
        self.links.write().await.insert(info, link);
        Ok(())
    }

    /// Opens a `Link` to an outbound peer.
    async fn open(self: &Arc<Self>, peer_uri: PeerURI) -> Result<(), Error> {
        let (r, w) = LinkListener::open(&peer_uri).await?;
        let (info, link) = Link::start(self, peer_uri, r, w, false).await?;
        self.links.write().await.insert(info, link);
        Ok(())
    }

    /// Closes a `Link` to a linked peer.
    async fn close(self: &Arc<Self>, peer_uri: &PeerURI) -> Result<(), Error> {
        unimplemented!()
    }

    #[inline]
    async fn has_link(self: &Arc<Self>, info: &LinkInfo) -> bool {
        self.links.read().await.get(info).is_some()
    }
}

// #[async_trait::async_trait]
// impl<C: Core> Handler<link::messages::Listen> for LinkManager<C> {
//     async fn handle(&mut self, ctx: &Context<Self>, msg: link::messages::Listen) {
//         // self.listen(msg.addr, ctx);
//         unimplemented!()
//     }
// }

#[async_trait::async_trait]
impl<C: Core> link::Link<C, LinkManager<C>> for Link<C> {}

/// Represents an active, direct, link to another peer established via a `LinkHandle`.
#[derive(Debug)]
pub struct Link<C: Core> {
    core: Addr<C>,
    adapter: Arc<LinkManager<C>>,

    // link info
    info: Arc<LinkInfo>,
    peer: Option<Addr<IPeer<C>>>,

    // interface: LinkHandle,
    reader: Option<LinkReader>,
    writer: Addr<LinkWriter>,
}

// Public methods.
impl<C: Core> Link<C> {
    /// Starts a `Link` that reads and writes packets on the provided [`PeerURI`].
    ///
    /// [`PeerURI`]:
    async fn start(
        adapter: &Arc<LinkManager<C>>,
        peer_uri: PeerURI,
        mut reader: LinkReader,
        mut writer: LinkWriter,
        incoming: bool,
    ) -> Result<(Arc<LinkInfo>, Addr<Self>), Error> {
        let mut core = adapter.core.clone();
        let config = C::current_config(&mut core).await?;
        let info = Self::init(
            &config,
            adapter,
            peer_uri,
            &mut reader,
            &mut writer,
            incoming,
        )
        .await?;

        // TODO establish timers

        let info = Arc::from(info);
        let mut link = Link {
            core,
            info: info.clone(),
            peer: None,
            adapter: adapter.clone(),
            reader: Some(reader),
            writer: Actor::start(writer).await?,
        };

        Ok((info, Actor::start(link).await?))
    }

    ///
    #[inline]
    fn notify(link: &mut Addr<Self>, msg: messages::Notification) -> Result<(), Error> {
        Ok(link
            .send(msg)
            .map_err(|_| ConnError::Link("failed to notify link"))?)
    }
}

// Internal methods.
impl<C: Core> Link<C> {
    /// Initializes the peer connection, performing the initial handshake and
    /// key validation.
    async fn init(
        config: &Arc<Config>,
        adapter: &Arc<LinkManager<C>>,
        peer_uri: PeerURI,
        reader: &mut LinkReader,
        writer: &mut LinkWriter,
        incoming: bool,
    ) -> Result<LinkInfo, Error> {
        let BoxKeypair { secret, public } = BoxKeypair::random();
        let our_meta = Metadata::new(
            config.encryption_public_key,
            config.signing_public_key,
            public,
        );

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

        // FIXME: compare signatures for greater security
        if meta > CURRENT_METADATA {
            return Err(ConnError::Link("failed to connect: wrong version"))?;
        }

        let MetadataKeys { sig, r#box, link } =
            meta.keys.expect("metadata keys should have been set");
        let info = LinkInfo {
            peer_uri,
            sig,
            r#box,
            link: BoxKeypair::new(secret, link),
        };

        // assert that this key is allowed to connect with us
        if incoming && !config.allowed_peer_keys.is_allowed(&info.r#box) {
            // TODO improve error message
            return Err(ConnError::Link("connection refused"))?;
        }

        // TODO check if we have already have a link to the node
        if adapter.has_link(&info).await {
            // TODO? handle this
        }

        Ok(info)
    }
}

#[async_trait::async_trait]
impl<C: Core> Actor for Link<C> {
    /// Starts `Read`ing packets from the underlying [`LinkReader`].
    ///
    /// [`LinkReader`]:
    async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {
        use messages::Notification;

        let mut peer_manager = C::peer_manager(&mut self.core).await?;
        let mut peer = <IPeerManager<C> as peer::PeerManager<C>>::new_peer(
            &mut peer_manager,
            self.info.sig,
            self.info.r#box,
            self.info.link.shared_key(),
            Box::new(ctx.address()),
        )
        .await?;
        self.peer.replace(peer.clone());

        let peer = peer.clone();
        let reader = self.reader.take().expect("uninitialized LinkReader");
        let stream = wire::Packet::stream(reader)
            .map_err(Error::Wire)
            .and_then(move |packet| {
                let mut peer = peer.clone();
                <IPeer<C> as peer::Peer<C, C::PeerManager>>::handle_packet(peer, packet)
            })
            .take_while(|res| future::ready(res.is_ok()))
            .map(|res| Notification::Read(res.unwrap()));
        ctx.add_stream(stream);

        Ok(())
    }
}

#[async_trait::async_trait]
impl<C: Core> Handler<messages::Notification> for Link<C> {
    async fn handle(&mut self, ctx: &Context<Self>, msg: link::messages::Notification) {
        <Self as StreamHandler<messages::Notification>>::handle(self, ctx, msg).await
    }
}

#[async_trait::async_trait]
impl<C: Core> StreamHandler<messages::Notification> for Link<C> {
    async fn handle(&mut self, ctx: &Context<Self>, msg: link::messages::Notification) {
        unimplemented!()
    }

    async fn finished(&mut self, ctx: &Context<Self>) {
        ctx.stop(None);
    }
}

#[async_trait::async_trait]
impl<C: Core> link::LinkInterface for Link<C> {
    type Inner = Addr<Self>;
    // type Reader = LinkReader;
    // type Writer = LinkWriter;

    fn out<T: Wire>(intf: &mut Self::Inner, msg: T) {
        // intf.send()
    }

    fn link_out<T: Wire>(intf: &mut Self::Inner, msg: T) {}

    fn close(intf: &mut Self::Inner) {}

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

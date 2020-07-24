use futures_locks::Mutex;
use std::{
    collections::HashMap,
    convert::TryFrom,
    sync::Arc,
    time::{Duration, Instant},
};
use wg::{Tunn, TunnResult};
use yggy_core::{
    dev::*,
    interfaces::{router::session, switch},
    types::{
        Address, AllowedEncryptionPublicKeys, BoxNonce, BoxPublicKey, BoxSharedKey, Coords, Handle,
        NodeID, Subnet, MTU,
    },
};

type ILookupTable<C> = <ISwitch<C> as switch::Switch<C>>::LookupTable;
type ISwitch<C> = <C as Core>::Switch;

// #[derive(Debug)]
// pub struct SessionManager<C: Core>(Mutex<InnerSessionManager<C>>);

/// Maintains all active sessions, indexed by their [`Handle`].
///
/// [`Handle`]: ../../core_types/crypto/struct.Handle.html
#[derive(Debug)]
pub struct SessionManager<C: Core> {
    // external state
    core: Addr<C>,
    router: Addr<C::Router>,
    listener: Addr<C::Listener>,
    pub(crate) lookup_table: ILookupTable<C>,

    // session state
    allowed_peer_keys: AllowedEncryptionPublicKeys,
    max_allowed_mtu: MTU,
    last_cleanup: Instant,

    // active and indexed sessions
    pub(crate) sessions: Mutex<HashMap<Handle, Addr<Session<C>>>>,
    pub(crate) handles: Mutex<HashMap<BoxPublicKey, Handle>>,
    // pub(crate) shared_keys: HashMap<BoxPublicKey, BoxSharedKey>,
}

impl<C: Core> SessionManager<C> {
    pub async fn new(mut core: Addr<C>) -> Result<Self, Error> {
        let config = C::current_config(&mut core).await?;
        let router = C::router(&mut core).await?;
        let listener = C::listener(&mut core).await?;

        let mut switch = C::switch(&mut core).await?;
        let lookup_table = <ISwitch<C> as switch::Switch<C>>::get_lookup_table(&mut switch)
            .await
            .ok_or_else(|| Error::Init(anyhow::Error::msg("unable to retrieve lookup table")))?;
        Ok(Self {
            core,
            router,
            listener,
            lookup_table,
            allowed_peer_keys: config.allowed_peer_keys.clone(),
            max_allowed_mtu: config.interface_max_mtu, // ? default?
            sessions: Default::default(),
            // shared_keys: Default::default(),
            handles: Default::default(),
            last_cleanup: Instant::now(),
        })
    }

    async fn add_session(
        &self,
        self_handle: Handle,
        their_key: BoxPublicKey,
        session: Addr<Session<C>>,
    ) -> Result<(), Error> {
        self.sessions.lock().await.insert(self_handle, session);
        self.handles.lock().await.insert(their_key, self_handle);
        // self.shared_keys.lock().await.insert(their_key, )
        Ok(())
    }
}

#[async_trait::async_trait]
impl<C: Core> session::SessionManager<C> for SessionManager<C> {
    type Session = Session<C>;

    fn reconfigure(&mut self) {
        unimplemented!()
    }

    async fn session_by_handle(&self, handle: &Handle) -> Option<Addr<Self::Session>> {
        self.sessions.lock().await.get(handle).map(Clone::clone)
    }

    async fn session_by_pub_key(&self, key: &BoxPublicKey) -> Option<Addr<Self::Session>> {
        let mut handles = self.handles.lock().await;
        match handles.get(key) {
            Some(handle) => self.session_by_handle(handle).await,
            None => None,
        }
    }

    async fn create_session(
        mut self: Arc<Self>,
        their_key: BoxPublicKey,
    ) -> Result<Addr<Self::Session>, Error> {
        let self_handle = Handle::new();
        let session =
            Session::start(self.clone(), self.core.clone(), self_handle, &their_key).await?;

        Arc::get_mut(&mut self)
            .unwrap()
            .add_session(self_handle, their_key, session.clone())
            .await?;

        Ok(session)
    }
}

///
pub struct Session<C: Core> {
    // external state
    core: Addr<C>,
    // conn: Addr<<C as Core>::Conn>,
    session_manager: Arc<SessionManager<C>>,
    lookup_table: ILookupTable<C>,

    // session state
    /// Represents the underlying point-to-point WireGuard connection.
    tunn: Box<Tunn>,
    is_initialized: bool,
    was_mtu_fixed: bool,
    opened: Instant,
    last_packet: Instant,
    last_mtu_change: Instant,
    first_ping_since_last_packet: Instant,

    // peer properties
    self_handle: Handle,
    self_nonce: BoxNonce,
    self_mtu: MTU,
    their_addr: Address,
    their_subnet: Subnet,
    // their_handle: Handle,
    // their_coords: Coords,
    // their_nonce: BoxNonce,
    their_mtu: MTU,
}

impl<C: Core> Session<C> {
    pub async fn start(
        session_manager: Arc<SessionManager<C>>,
        mut core: Addr<C>,
        self_handle: Handle,
        their_key: &BoxPublicKey,
    ) -> Result<Addr<Self>, Error> {
        let config = C::current_config(&mut core).await?;
        let lookup_table = session_manager.lookup_table.clone();

        let self_mtu = session_manager.max_allowed_mtu;
        let their_nodeid = NodeID::try_from(their_key)?;

        let now = Instant::now();
        let session = Self {
            core,
            session_manager,
            lookup_table,
            tunn: Tunn::new(
                config.encryption_private_key.clone().into(),
                Arc::new(their_key.as_bytes().into()),
                None,
                None,
                100, // TODO
                None,
            )
            .unwrap(), // TODO
            is_initialized: false,
            was_mtu_fixed: false,
            opened: now,
            last_packet: now,
            last_mtu_change: now,
            first_ping_since_last_packet: now,

            // peer properties
            self_handle,
            self_nonce: BoxNonce::new(),
            self_mtu,
            their_addr: Address::from(&their_nodeid),
            their_subnet: Subnet::from(&their_nodeid),
            // their_handle: Handle::new(),
            // their_coords: Coords,
            // their_nonce: BoxNonce::new(), TODO higher key -> odd, else even
            their_mtu: MTU::MIN,
        };

        Ok(Actor::start(session).await?)
    }
}

impl<C: Core> session::Session<C, SessionManager<C>> for Session<C> {}

#[async_trait::async_trait]
impl<C: Core> Actor for Session<C> {
    async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {
        unimplemented!()
    }
}

use super::Router;
use crate::{
    core_interfaces::{router::session, switch, Core},
    core_types::{
        Address, AllowedEncryptionPublicKeys, BoxPublicKey, BoxSharedKey, Coords, Handle, NodeID,
        Subnet, MTU,
    },
    error::Error,
};
use boringtun::noise::{Tunn, TunnResult};
use std::{
    collections::HashMap,
    convert::TryFrom,
    sync::Arc,
    time::{Duration, Instant},
};
use xactor::{Actor, Addr, Context, Handler, StreamHandler};

type ISwitch<C> = <C as Core>::Switch;
type ILookupTable<C> = <ISwitch<C> as switch::Switch<C>>::LookupTable;

/// Maintains all active sessions, indexed by their [`Handle`].
///
/// [`Handle`]: ../../core_types/crypto/struct.Handle.html
#[derive(Debug)]
pub struct SessionManager<C: Core> {
    core: Addr<C>,
    router: Addr<C::Router>,
    listener: Addr<C::Listener>,
    allowed_peer_keys: AllowedEncryptionPublicKeys,
    max_allowed_mtu: MTU,
    sessions: HashMap<Handle, Addr<Session<C>>>,
    shared_keys: HashMap<BoxPublicKey, BoxSharedKey>,
    handles: HashMap<BoxPublicKey, Handle>,
    last_cleanup: Instant,
}

impl<C: Core> SessionManager<C> {
    #[inline]
    pub async fn new(mut core: Addr<C>) -> Result<Self, Error> {
        let config = C::current_config(&mut core).await?;
        let router = C::router(&mut core).await?;
        let listener = C::listener(&mut core).await?;
        Ok(Self {
            core,
            router,
            listener,
            allowed_peer_keys: config.allowed_peer_keys,
            max_allowed_mtu: config.interface_max_mtu, // ? default?
            sessions: Default::default(),
            shared_keys: Default::default(),
            handles: Default::default(),
            last_cleanup: Instant::now(),
        })
    }
}

#[async_trait::async_trait]
impl<C: Core> session::SessionManager<C> for SessionManager<C> {
    type Session = Session<C>;

    fn reconfigure(&mut self) {
        unimplemented!()
    }

    fn session_by_handle(&self, handle: &Handle) -> Option<Addr<Self::Session>> {
        self.sessions.get(handle).map(|addr| addr.clone())
    }

    fn session_by_pub_key(&self, key: &BoxPublicKey) -> Option<Addr<Self::Session>> {
        self.handles
            .get(key)
            .map(|handle| self.session_by_handle(handle))
            .flatten()
    }

    async fn create_session(
        self: Arc<Self>,
        their_key: &BoxPublicKey,
    ) -> Result<Addr<Self::Session>, Error> {
        // let lookup_table = <ISwitch<C> as switch::Switch<C>>::get_look
        let session = Session::start(self.core.clone(), self.clone(), their_key).await?;

        // TODO

        Ok(session)
    }
}

///
pub struct Session<C: Core> {
    // external state
    core: Addr<C>,
    // conn: Addr<<C as Core>::Conn>,
    session_manager: Arc<SessionManager<C>>,
    // lookup_table: Arc<ILookupTable<C>>,

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
    self_mtu: MTU,
    their_addr: Address,
    their_subnet: Subnet,
    // their_handle: Handle,
    // their_coords: Coords,
    their_mtu: MTU,
}

impl<C: Core> Session<C> {
    #[inline]
    pub async fn start(
        mut core: Addr<C>,
        session_manager: Arc<SessionManager<C>>,
        // lookup_table: Arc<ILookupTable<C>>,
        their_key: &BoxPublicKey,
    ) -> Result<Addr<Self>, Error> {
        let config = C::current_config(&mut core).await?;
        let mut switch = C::switch(&mut core).await?;

        let self_mtu = session_manager.max_allowed_mtu;
        let their_nodeid = NodeID::try_from(their_key)?;
        let now = Instant::now();

        let session = Self {
            core,
            session_manager,
            // lookup_table,
            tunn: Tunn::new(
                Arc::new(config.encryption_private_key.into()),
                Arc::new(their_key.as_bytes().into()),
                None,
                None,
                100,
                None,
            )
            .unwrap(),
            is_initialized: false,
            was_mtu_fixed: false,
            opened: now,
            last_packet: now,
            last_mtu_change: now,
            first_ping_since_last_packet: now,

            // peer properties
            self_handle: Handle::new(),
            self_mtu,
            their_addr: Address::from(&their_nodeid),
            their_subnet: Subnet::from(&their_nodeid),
            // their_handle: Handle::new(),
            // their_coords: Coords,
            their_mtu: MTU::MIN,
        };

        unimplemented!()
    }
}

impl<C: Core> session::Session<C, SessionManager<C>> for Session<C> {}

#[async_trait::async_trait]
impl<C: Core> Actor for Session<C> {
    async fn started(&mut self, ctx: &Context<Self>) -> Result<(), anyhow::Error> {
        unimplemented!()
    }
}

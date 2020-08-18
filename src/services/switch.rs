use futures_locks::RwLock;
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use yggy_core::{
    dev::*,
    interfaces::switch::{self, messages},
    types::{NodeID, SigningPublicKey, SwitchLocator, SwitchPort},
};

lazy_static! {
    /// TODO?
    static ref UPDATE_INTERVAL: Duration = ROOT_TIMEOUT / 2;

    /// TODO?
    static ref THROTTLE_INTERVAL: Duration = *UPDATE_INTERVAL / 2;
}

/// TODO?
const ROOT_TIMEOUT: Duration = Duration::from_secs(60);

/// Number of switch updates before switching to a faster parent.
const PARENT_UPDATE_THRESHOLD: u8 = 240;

/// Minimum allowed total size of switch queues.
const MIN_TOTAL_QUEUE_SIZE: u64 = 4 * 1024 * 1024;

///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeerInfo {
    /// ID of the peer.
    /// ? TreeID?
    key: SigningPublicKey,

    ///
    locator: SwitchLocator,

    /// Self-reported degree.
    degree: u64,

    /// Interface number of this peer.
    port: SwitchPort,

    /// Time this node was last seen
    /// ? u32?
    last_seen: Instant,

    // msg: RootUpdate,
    is_blocked: bool,
    // /// Counter of how often a node is faster than the current parent, penalized extra if slower.
    // faster_nodes: HashMap<SwitchPort, u64>,
}

///
/// TODO
#[derive(Clone, Debug, Eq, PartialEq)]
struct SwitchData {
    locator: SwitchLocator,
    seq: u64,
    peers: HashMap<SwitchPort, PeerInfo>,
    // msg: RootUpdate
}

///
#[derive(Debug)]
pub struct Switch<C: Core> {
    core: Addr<C>,

    // ///
    // self_signing_key: SigningPublicKey,
    ///
    switch_data: SwitchData,

    /// Time
    last_locator_update: Option<Instant>,

    /// Port of whatever peer is our parent, or our own port if we're the root.
    parent_port: Option<SwitchPort>,

    ///
    dropped_roots: HashMap<SigningPublicKey, Instant>,

    ///
    lookup_table: LookupTable,
}

impl<C: Core> Switch<C> {
    ///
    #[inline]
    pub async fn start(mut core: Addr<C>) -> Result<Addr<Self>, Error> {
        //     let config = C::current_config(&mut core).await?;

        //     let switch = Self {
        //         core,
        //         self_signing_key: config.signing_public_key,
        //         last_locator_update: Default::default(),
        //         dropped_roots: Default::default(),
        //         lookup_table:
        //     };

        unimplemented!()
    }
}

#[async_trait::async_trait]
impl<C: Core> switch::Switch<C> for Switch<C> {
    type LookupTable = LookupTable;
}

#[async_trait::async_trait]
impl<C: Core> Actor for Switch<C> {}

#[async_trait::async_trait]
impl<C: Core> Handler<switch::messages::GetLookupTable<C, Self>> for Switch<C> {
    async fn handle(
        &mut self,
        ctx: &Context<Self>,
        msg: switch::messages::GetLookupTable<C, Self>,
    ) -> LookupTable {
        self.lookup_table.clone()
    }
}

/// Subset of information about a peer needed to make routing decisions.
#[derive(Debug)]
pub struct LookupTableElem {
    port: SwitchPort,
    last_seen: Instant,
    locator: SwitchLocator,
    // next:
}

/// Subset of information about all peers needed to make routing decisions.
#[derive(Clone, Debug)]
pub struct LookupTable(RwLock<InnerLookupTable>);

impl switch::LookupTable for LookupTable {
    type Item = LookupTableElem;
}

impl LookupTable {
    ///
    ///
    /// TODO: look at python impl below for "pre-computing" the table
    /// # Pre-computes a lookup table for destination coords
    /// # Insert parent first so you prefer them as a next-hop
    /// self.table.clear()
    /// parent = self.info.nodeID
    /// if len(self.info.coords) >= 2: parent = self.info.coords[-2]
    /// for peer in self.peers.itervalues():
    /// current = self.table
    /// for coord in peer.coords:
    ///     if coord not in current: current[coord] = (peer.nodeID, dict())
    ///     old = current[coord]
    ///     next = old[1]
    ///     oldPeer = self.peers[old[0]]
    ///     oldDist = len(oldPeer.coords)
    ///     oldDeg = oldPeer.degree
    ///     newDist = len(peer.coords)
    ///     newDeg = peer.degree
    ///     # Prefer parent
    ///     # Else prefer short distance from root
    ///     # If equal distance, prefer high degree
    ///     if peer.nodeID == parent: current[coord] = (peer.nodeID, next)
    ///     elif newDist < oldDist: current[coord] = (peer.nodeID, next)
    ///     elif newDist == oldDist and newDeg > oldDeg: current[coord] = (peer.nodeID, next)
    ///     current = next
    /// return None
    fn init(self_info: &PeerInfo) -> Self {
        // let mut parent: NodeID = info.key.into();
        // if info.locator.coords().len() >= 2 {
        //     parent = info.coords.get(-2);
        // };

        unimplemented!()
    }
}

#[derive(Debug)]
struct InnerLookupTable {
    /// Our current `SwitchLocator`.
    self_locator: SwitchLocator,

    /// All switch peers, just for sanity checks + API/debugging.
    peers: HashMap<SwitchPort, LookupTableElem>,
    // _start
    // _msg
}

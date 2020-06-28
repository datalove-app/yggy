use std::{
    collections::HashMap,
    fmt,
    sync::Arc,
    time::{Duration, Instant},
};
use xactor::{Actor, Addr, Context, Handler};
use yggy_core::{
    interfaces::{switch, Core},
    types::{NodeID, SigningPublicKey, SwitchLocator, SwitchPort},
};

///
const ROOT_TIMEOUT: Duration = Duration::from_secs(60);

///
const UPDATE_INTERVAL: Duration = Duration::from_secs(ROOT_TIMEOUT.as_secs() >> 1);

///
const THROTTLE_INTERVAL: Duration = Duration::from_secs(UPDATE_INTERVAL.as_secs() >> 1);

/// Number of switch updates before switching to a faster parent.
const PARENT_UPDATE_THRESHOLD: u8 = 240;

/// Minimum allowed total size of switch queues.
const MIN_TOTAL_QUEUE_SIZE: u64 = 4 * 1024 * 1024;

///
#[derive(Debug)]
pub struct Switch<C: Core> {
    ///
    core: Addr<C>,

    ///
    self_signing_key: SigningPublicKey,

    ///
    last_locator_update: Instant,

    ///
    dropped_roots: HashMap<SigningPublicKey, Instant>,

    ///
    parent_port: SwitchPort,
    // ///
    // switch_data: SwitchData,
    ///
    lookup_table: Arc<<Self as switch::Switch<C>>::LookupTable>,
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
    ) -> Arc<<Self as switch::Switch<C>>::LookupTable> {
        unimplemented!()
    }
}

///
/// TODO
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwitchData {
    locator: SwitchLocator,
    seq: u64,
    peers: HashMap<SwitchPort, PeerInfo>,
    // msg: SwitchMessage
}

// ///
// #[derive(Debug)]
// pub struct LookupTable {
//     our_locator: SwitchLocator,
//     elems:
// }

// #[derive(Debug)]
// pub struct LookupTableElement

/// Subset of information about all peers needed to make routing decisions.
pub struct LookupTable {
    /// Our current [`SwitchLocator`].
    ///
    /// [`SwitchLocator`]:
    self_locator: SwitchLocator,

    /// All switch peers, just for sanity checks + API/debugging.
    peers: HashMap<SwitchPort, LookupTableElem>,
}

impl switch::LookupTable for LookupTable {}

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
    fn init(info: &PeerInfo) -> Self {
        // let mut parent: NodeID = info.key.into();
        // if info.locator.coords().len() >= 2 {
        //     parent = info.coords.get(-2);
        // };

        unimplemented!()
    }
}

impl fmt::Debug for LookupTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        unimplemented!()
    }
}

/// Subset of information about a peer needed to make routing decisions.
#[derive(Debug)]
pub struct LookupTableElem {
    port: SwitchPort,
    last_seen: Instant,
    locator: SwitchLocator,
}

///
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PeerInfo {
    /// ID of this peer
    /// ? TreeID?
    key: SigningPublicKey,

    /// Self-reported degree.
    degree: u64,

    /// Time this node was last seen.
    last_seen: Instant,

    locator: SwitchLocator,
    port: SwitchPort,

    // /// The wire [`SwitchMessage`] used.
    // ///
    // /// [`SwitchMessage`]:
    // msg: SwitchMessage,
    /// Used to avoid parenting a blocked link.
    is_blocked: bool,
}

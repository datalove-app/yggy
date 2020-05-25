use crate::core::{
    types::{NodeID, PeerInfo, SigningPublicKey, SwitchLocator, SwitchPort},
    Core,
};
use std::{collections::HashMap, time::Duration};

///
///
pub trait Switch<C: Core> {}

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

pub struct LookupTable {
    map: HashMap<SwitchPort, (NodeID)>,
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
    fn init(info: &PeerInfo) -> Self {
        // let mut parent: NodeID = info.key.into();
        // if info.locator.coords().len() >= 2 {
        //     parent = info.coords.get(-2);
        // };

        unimplemented!()
    }
}

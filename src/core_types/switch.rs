use super::SigningPublicKey;
use crate::error::Error;
use smallvec::SmallVec;
use std::{cmp::Ordering, time::Duration};

/// Represents a path from the root to a node.
/// This path is generally part of a spanning tree, except possibly the last hop
/// (it can loop when sending coords to your parent, but they will see this and
/// know not to use a looping path).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Coords(SmallVec<[SwitchPort; Self::DEFAULT_SIZE]>);

impl Coords {
    const DEFAULT_SIZE: usize = 8;

    ///
    #[inline]
    pub fn distance(&self, other: &Self) -> i64 {
        // TODO: other might need to be bytes from the wire protocol
        unimplemented!()
    }
}

impl std::convert::TryFrom<&WireCoords> for Coords {
    type Error = Error;
    fn try_from(coords: &WireCoords) -> Result<Self, Self::Error> {
        unimplemented!()
    }
}

/// Represents an encoded, compressed representation of [`Coords`].
///
/// [`Coords`]: struct.Coords
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WireCoords(SmallVec<[u8; Self::DEFAULT_BYTES_SIZE]>);

impl WireCoords {
    const DEFAULT_BYTES_SIZE: usize = 32;

    ///
    #[inline]
    pub fn distance(&self, other: &Self) -> i64 {
        unimplemented!()
    }
}

impl From<&Coords> for WireCoords {
    #[inline]
    fn from(coords: &Coords) -> Self {
        unimplemented!()
    }
}

// impl PartialOrd for WireCoords {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         if self.0.len() > other.0.len() {
//             Some(Ordering::Greater)
//         } else {
//             for i in self.iter() {
//                 if self.0[i] != other.0[i] {
//                     return Some(Ordering::Greater);
//                 }
//             }

//             Some(Ordering::Less)
//         }
//     }
// }

// impl Ord for WireCoords {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.partial_cmp(other).unwrap()
//     }
// }

///
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SwitchPort(u64);

/// Contains the root node's signing key, timestamp, and signed per-hop info
/// about a path from the root node to some other node in the network.
/// This is exchanged with peers to construct the spanning tree.
/// A subset of this information, excluding signatures, is used to construct
/// [`SwitchLocator`]s.
///
/// [`SwitchLocator`]: struct.SwitchLocator
#[derive(Clone, Copy, Debug)]
pub struct SwitchMessage {
    root: SigningPublicKey,
    timestamp: i64,
    // hops: TODO:
}

/// Represents the signed information about the path leading from the root to
/// the `next` node, via the `port` specified here.
#[derive(Clone, Copy, Debug)]
pub struct SwitchMessageHop {
    port: SwitchPort,
    next: SigningPublicKey,
    // signature: TODO:
}

/// Represents the topology and network state-dependent info about a node, sans
/// the signatures that accompany it. Nodes will pick the best root they see,
/// provided that the root continues to push out updates with new timestamps.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SwitchLocator {
    root: SigningPublicKey,
    timestamp: i64,
    coords: Coords,
}

impl SwitchLocator {
    #[inline]
    pub fn distance(&self) -> i64 {
        unimplemented!()
    }

    #[inline]
    pub fn coords(&self) -> &Coords {
        &self.coords
    }

    #[inline]
    pub fn wirecoords(&self) -> WireCoords {
        unimplemented!()
    }

    /// Returns `true` if this locator represents an ancestor of the locator
    /// given as an argument.
    #[inline]
    pub fn is_ancestor_of(&self, other: &Self) -> bool {
        self < other
    }
}

/// Returns an ordering of `SwitchLocator`s, with the lesser being closer to
/// the root, i.e. the ancestor of the other.
impl PartialOrd for SwitchLocator {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        unimplemented!()
    }
}

/// Returns an ordering of `SwitchLocator`s, with the lesser being closer to
/// the root, i.e. the ancestor of the other.
impl Ord for SwitchLocator {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        unimplemented!()
    }
}

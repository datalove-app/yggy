use super::{NodeID, TreeID};
use crate::error::Error;
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, io::Write, net::Ipv6Addr, str::FromStr};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum NetworkID {
    NodeID,
    Curve25519,
    // Custom(String),
}

impl Default for NetworkID {
    fn default() -> Self {
        Self::NodeID
    }
}

impl PartialEq<str> for NetworkID {
    fn eq(&self, other: &str) -> bool {
        match self {
            Self::NodeID => other == "nodeid",
            Self::Curve25519 => other == "curve25519",
            // Self::Custom(s) => s == other,
        }
    }
}

// /// The current address prefix used by yggdrasil.
// pub const ADDRESS_PREFIX: [u8; 1] = [0x02];

///
#[derive(Copy, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "Ipv6Addr")]
pub struct Address(Ipv6Addr);

impl Address {
    // ///
    // #[inline]
    // pub fn from_str(s: &str) -> Result<Self, Error> {
    //     let ip = Ipv6Addr::from_str(s).map_err(|_| Error::Init)?;

    //     unimplemented!()
    // }

    /// The first `NodeID` with all the bits known from the `Address` set to
    /// their correct values.
    /// The second `NodeID` is a bitmask with 1 bit set for each bit that was
    /// known from the `Address`.
    /// This is used to look up `NodeID`s in the DHT and determine if they match
    /// an `Address`.
    pub fn node_id_and_mask(&self) -> (NodeID, NodeID) {
        unimplemented!()
    }
}

impl Default for Address {
    fn default() -> Self {
        unimplemented!()
    }
}

impl TryFrom<Ipv6Addr> for Address {
    type Error = Error;
    fn try_from(raw: Ipv6Addr) -> Result<Self, Self::Error> {
        // TODO validate that it's in range (and prefix?)
        unimplemented!()
    }
}

impl From<&NodeID> for Address {
    fn from(node_id: &NodeID) -> Self {
        let mut bytes: [u8; 16] = [0; 16];

        unimplemented!()
    }
}

///
#[derive(Copy, Clone, Debug)]
pub struct Subnet([u8; 8]);

impl Subnet {
    /// The first `NodeID` with all the bits known from the `Subnet` set to
    /// their correct values.
    /// The second `NodeID` is a bitmask with 1 bit set for each bit that was
    /// known from the `Subnet`.
    /// This is used to look up `NodeID`s in the DHT and determine if they match
    /// an `Subnet`.
    pub fn node_id_and_mask(&self) -> (NodeID, NodeID) {
        unimplemented!()
    }
}

impl From<&NodeID> for Subnet {
    fn from(node_id: &NodeID) -> Self {
        unimplemented!()
    }
}

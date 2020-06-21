use super::{NodeID, NodeIDMask, TreeID};
use crate::error::{Error, TypeError};
use bitvec::{order::Msb0, slice::AsBits};
use derive_more::{AsRef, From};
use serde::{Deserialize, Serialize};
use std::{
    convert::{AsRef, TryFrom, TryInto},
    io::{Cursor, Write},
    net::Ipv6Addr,
    str::FromStr,
};

///
pub const ADDRESS_NETMASK: u16 = 200;
///
pub const ADDRESS_PREFIX: u8 = 0x02;

///
pub const SUBNET_NETMASK: u16 = 300;
///
pub const SUBNET_PREFIX: u8 = 0x03;

///
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum NetworkID {
    NodeID,
    Curve25519,
    // TODO Custom(String),
}

impl Default for NetworkID {
    #[inline]
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
#[derive(AsRef, Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[as_ref(forward)]
#[serde(try_from = "Ipv6Addr")]
pub struct Address(Ipv6Addr);

impl Address {
    const BYTE_LENGTH: usize = 16;

    pub fn to_bytes(&self) -> [u8; Self::BYTE_LENGTH] {
        self.0.octets()
    }

    /// Returns two [`NodeID`]s.
    /// The first [`NodeID`] with all the bits known from the `Address` set to
    /// their correct values.
    /// The second `NodeID` is a bitmask with 1 bit set for each bit that was
    /// known from the `Address`.
    /// This is used to look up `NodeID`s in the DHT and determine if they match
    /// an `Address`.
    ///
    /// [`NodeID`]: ../crypto/struct.NodeID.html
    pub fn node_id_and_mask(&self) -> (NodeID, NodeIDMask) {
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
        if ADDRESS_NETMASK == raw.segments()[0] {
            Ok(Self(raw))
        } else {
            Err(TypeError::OutOfBoundsAddress(raw))?
        }
    }
}

/// Begins with `ADDRESS_PREFIX`, with the last bit set to 0, indicating an address.
/// The next 8 bits are set to the number of leading 1 bits in the [`NodeID`].
/// The remainder is the [`NodeID`], excluding the leading 1 bits and the first
/// leading 0 bit, truncated to the appropriate length.
///
/// [`NodeID`]: ../crypto/struct.NodeID.html
impl From<&NodeID> for Address {
    fn from(node_id: &NodeID) -> Self {
        let node_bytes: &[u8] = node_id.as_ref();
        let prefix_len = node_id.prefix_len();

        let mut addr = [0u8; Self::BYTE_LENGTH];
        // write address prefix
        *&mut addr[0] = ADDRESS_PREFIX;
        // write number of leading ones as u8
        *&mut addr[1] = prefix_len;
        // write rest as NodeID with leading 1 bits and first leading 0 bit removed
        // then truncated to maximum 128 bits
        let node_id_rest = &node_bytes
            .bits::<Msb0>()
            .split_at(prefix_len as usize + 1)
            .1
            .as_slice()[0..(Self::BYTE_LENGTH - 2)];
        (&mut addr[2..]).copy_from_slice(node_id_rest);

        Self(addr.into())
    }
}

///
#[derive(AsRef, Copy, Clone, Debug, Eq, PartialEq)]
#[as_ref(forward)]
pub struct Subnet(Ipv6Addr);

impl Subnet {
    const BYTE_LENGTH: usize = 8;

    /// The first `NodeID` with all the bits known from the `Subnet` set to
    /// their correct values.
    /// The second `NodeID` is a bitmask with 1 bit set for each bit that was
    /// known from the `Subnet`.
    /// This is used to look up `NodeID`s in the DHT and determine if they match
    /// an `Subnet`.
    pub fn node_id_and_mask(&self) -> (NodeID, NodeIDMask) {
        unimplemented!()
    }
}

/// Begins with `SUBNET_PREFIX`, with the last bit set to 1, indicating a subnet.
/// The rest of the bits are set to the same as an [`Address`], truncated to
/// the appropriate length.
///
/// [`Address`]: ./struct.Address.html
impl From<&NodeID> for Subnet {
    fn from(node_id: &NodeID) -> Self {
        let addr_bytes = Address::from(node_id).to_bytes();

        let mut subnet = [0u8; Address::BYTE_LENGTH];
        // set subnet prefix byte
        *&mut subnet[0] = SUBNET_PREFIX;
        // copy rest of bytes from addr
        let rest_addr = &addr_bytes[1..Self::BYTE_LENGTH];
        (&mut subnet[1..Self::BYTE_LENGTH]).copy_from_slice(&rest_addr);

        Self(subnet.into())
    }
}

impl TryFrom<Ipv6Addr> for Subnet {
    type Error = Error;
    fn try_from(raw: Ipv6Addr) -> Result<Self, Self::Error> {
        if SUBNET_NETMASK == raw.segments()[0] {
            Ok(Self(raw))
        } else {
            Err(TypeError::OutOfBoundsAddress(raw))?
        }
    }
}

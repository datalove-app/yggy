use super::{NodeID, TreeID};
use crate::error::{Error, TypeError};
use bitvec::{order::Msb0, slice::BitSlice};
use derive_more::{AsRef, From};
use ipnet::{Ipv6Net, Ipv6Subnets};
use serde::{Deserialize, Serialize};
use std::{
    convert::{AsRef, TryFrom, TryInto},
    io::{Cursor, Write},
    net::Ipv6Addr,
    str::FromStr,
};

lazy_static! {
    ///
    pub static ref NETWORK_MASK: Ipv6Net = "200::/7".parse().unwrap();
    ///
    pub static ref ADDRESS_NETMASK: Ipv6Net = "200::/8".parse().unwrap();
    ///
    pub static ref SUBNET_NETMASK: Ipv6Net = "300::/8".parse().unwrap();
}

///
pub const ADDRESS_PREFIX: u8 = 0x02;
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
#[serde(try_from = "Ipv6Net")]
pub struct Address(Ipv6Net);

impl Address {
    const BYTE_LENGTH: usize = 16;

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

impl TryFrom<Ipv6Net> for Address {
    type Error = Error;
    fn try_from(raw: Ipv6Net) -> Result<Self, Self::Error> {
        if (*ADDRESS_NETMASK).contains(&raw) {
            Ok(Self(raw))
        } else {
            Err(TypeError::OutOfBoundsAddress(raw))?
        }
    }
}

///
/// Begins with the `ADDRESS_PREFIX`, with the last bit of 0 indicating an address.
/// The next 8 bits are set to the number of leading 1 bits in the [`NodeID`].
/// The remainder is the [`NodeID`], excluding the leading 1 bits and the first
/// leading 0 bit.
///
/// [`NodeID`]: ../crypto/struct.NodeID.html
impl From<&NodeID> for Address {
    fn from(node_id: &NodeID) -> Self {
        let node_bytes: &[u8] = node_id.as_ref();
        let leading_ones = NodeID::leading_ones(node_bytes).expect("this should never fail");

        let mut addr: [u8; Self::BYTE_LENGTH] = [0; Self::BYTE_LENGTH];
        // write address prefix
        *&mut addr[0] = ADDRESS_PREFIX;
        // write number of leading ones as u8
        *&mut addr[1] = leading_ones;
        // write rest as NodeID with leading 1 bits and first leading 0 bit removed
        let (_, rest_id): (&BitSlice<Msb0, u8>, &BitSlice<Msb0, u8>) =
            BitSlice::<Msb0, u8>::from_slice(node_bytes).split_at(leading_ones as usize + 1);

        // truncate to max 128 bits
        {
            let rest_id = &rest_id.as_slice()[0..(Self::BYTE_LENGTH - 2)];
            let (_, rest) = addr.split_at_mut(2);
            rest.copy_from_slice(&rest_id);
        }
        Self(Ipv6Addr::from(addr).into())
    }
}

///
#[derive(AsRef, Copy, Clone, Debug, Eq, PartialEq)]
#[as_ref(forward)]
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

///
/// This subnet begins with the address prefix, with the last bit set to 1 to indicate a prefix.
/// The following 8 bits are set to the number of leading 1 bits in the NodeID.
/// The NodeID, excluding the leading 1 bits and the first leading 0 bit, is truncated to the appropriate length and makes up the remainder of the subnet.
impl From<&NodeID> for Subnet {
    fn from(node_id: &NodeID) -> Self {
        unimplemented!()
    }
}

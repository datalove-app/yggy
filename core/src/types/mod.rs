//! Core protocol types.

mod address;
mod crypto;
mod peer;
mod switch;
pub mod wire;

pub use address::*;
pub use crypto::*;
pub use peer::*;
pub use switch::*;
// pub use wire::{Header as WireHeader, Wire};

use crate::error::{Error, TypeError};
use derive_more::{AsRef, IntoIterator};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
    iter::Iterator,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
};

///
/// TODO
pub type AllowedEncryptionPublicKeys = HashSet<BoxPublicKey>;

///
/// TODO
#[derive(AsRef, Clone, Debug, Deserialize, Eq, IntoIterator, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ListenAddresses(Vec<PeerURI>);

impl ListenAddresses {
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &PeerURI> {
        self.0.iter()
    }
}

impl Default for ListenAddresses {
    #[inline]
    fn default() -> Self {
        Self(vec![PeerURI::default_listen()])
    }
}

///
/// TODO
pub type Ipv4Subnets = HashMap<(), ()>;

///
/// TODO
pub type Ipv6Subnets = HashMap<(), ()>;

///
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InterfaceName {
    ///
    #[serde(rename = "auto")]
    Auto,

    ///
    #[serde(rename = "none")]
    None,

    ///
    Custom(String),
}

impl Default for InterfaceName {
    #[inline]
    fn default() -> Self {
        Self::Auto
    }
}

///
#[derive(AsRef, Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(try_from = "u16")]
pub struct MTU(u16);

impl MTU {
    /// Minimum allowable MTU.
    /// ? platform-specific
    pub const MIN: Self = Self(1280);

    /// Maximum allowable MTU.
    // TODO handle platform-specific
    // #[cfg(any(target_os = "macos", target_os = "ios"))] and
    // #[cfg(target_os = "linux")]
    pub const MAX: Self = Self(65535);
}

impl Default for MTU {
    #[inline]
    fn default() -> Self {
        Self::MAX
    }
}

impl TryFrom<u16> for MTU {
    type Error = Error;

    #[inline]
    fn try_from(raw: u16) -> Result<Self, Self::Error> {
        if raw.lt(Self::MIN.as_ref()) || raw.gt(Self::MAX.as_ref()) {
            Err(TypeError::InvalidMTU(raw))?
        } else {
            Ok(Self(raw))
        }
    }
}

//! Core protocol types.

mod address;
mod crypto;
mod peer;
mod search;
mod session;
mod switch;
pub mod wire;

pub use address::*;
pub use crypto::*;
pub use peer::*;
pub use search::*;
pub use session::*;
pub use switch::*;
pub use wire::{Header as WireHeader, Wire};

use crate::error::{Error, TypeError};
use derive_more::AsRef;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::TryFrom,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
};

///
/// TODO
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ListenAddresses(Vec<PeerURI>);

impl Default for ListenAddresses {
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
    const MIN: Self = Self(1280);
    /// Maximum allowable MTU.
    /// TODO platform-specific
    const MAX: Self = Self(65535);
}

// TODO handle platform-specific
// #[cfg(any(target_os = "macos", target_os = "ios"))] and
// #[cfg(target_os = "linux")]
impl Default for MTU {
    fn default() -> Self {
        Self::MIN
    }
}

impl TryFrom<u16> for MTU {
    type Error = Error;

    fn try_from(raw: u16) -> Result<Self, Self::Error> {
        if raw.lt(Self::MIN.as_ref()) || raw.gt(Self::MAX.as_ref()) {
            Err(TypeError::InvalidMTU(raw))?
        } else {
            Ok(Self(raw))
        }
    }
}

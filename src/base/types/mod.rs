//! Core protocol types.

mod address;
mod crypto;
mod peer;

pub use address::*;
pub use crypto::*;
pub use peer::*;

use super::error::{ConfigError, Error};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    convert::TryFrom,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
};

///
pub type ListenAddresses = Vec<PeerURI>;

///
pub type Ipv4Subnets = HashMap<(), ()>;

///
pub type Ipv6Subnets = HashMap<(), ()>;

///
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum InterfaceName {
    #[serde(rename = "auto")]
    Auto,

    #[serde(rename = "none")]
    None,
    Custom(String),
}

impl Default for InterfaceName {
    fn default() -> Self {
        Self::Auto
    }
}

///
#[derive(Debug, Deserialize, Serialize)]
#[serde(try_from = "u16")]
pub struct MTU(u16);

// TODO handle platform-specific
// #[cfg(any(target_os = "macos", target_os = "ios"))] and
// #[cfg(target_os = "linux")]
impl Default for MTU {
    fn default() -> Self {
        unimplemented!()
    }
}

// TODO handle platform-specific maximum
// #[cfg(any(target_os = "macos", target_os = "ios"))] and
// #[cfg(target_os = "linux")]
impl TryFrom<u16> for MTU {
    type Error = Error;

    fn try_from(raw: u16) -> Result<Self, Self::Error> {
        if raw < 1280 {
            Err(ConfigError::InvalidMTU(raw).into())
        } else {
            Ok(Self(raw))
        }
    }
}

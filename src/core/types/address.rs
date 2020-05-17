use crate::core::{crypto::SigningPublicKey, error::Error};
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, net::Ipv6Addr, str::FromStr};

///
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(try_from = "Ipv6Addr")]
pub struct Address(Ipv6Addr);

impl Address {
    // ///
    // #[inline]
    // pub fn from_str(s: &str) -> Result<Self, Error> {
    //     let ip = Ipv6Addr::from_str(s).map_err(|_| Error::Init)?;

    //     unimplemented!()
    // }
}

impl TryFrom<Ipv6Addr> for Address {
    type Error = Error;
    fn try_from(raw: Ipv6Addr) -> Result<Self, Self::Error> {
        // Self(raw)
        unimplemented!()
    }
}

impl From<SigningPublicKey> for Address {
    fn from(pub_key: ed25519_dalek::PublicKey) -> Self {
        unimplemented!()
    }
}

///
#[derive(Debug)]
pub struct Subnet;

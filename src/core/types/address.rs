use crate::error::Error;
use derive_more::From;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, net::Ipv6Addr, str::FromStr};

///
#[derive(Debug, Deserialize, Serialize)]
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

// impl FromStr for Address {
//     type Err = Error;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         // Self(Ipv6Addr::from_str(s))
//         unimplemented!()
//     }
// }

///
#[derive(Debug)]
pub struct Subnet;

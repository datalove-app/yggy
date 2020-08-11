//! `yggy` core types and interfaces.
//!

#[macro_use]
extern crate lazy_static;

mod config;
pub mod error;
pub mod interfaces;
pub mod types;
pub mod version;
pub mod wire;

#[doc(inline)]
pub use config::Config;
#[doc(inline)]
pub use error::Error;
#[doc(inline)]
pub use interfaces::Core;
#[doc(inline)]
pub use version::CURRENT_METADATA;

/// Re-exports for developer convenience.
pub mod dev {
    pub use crate::{error::*, interfaces, types, version, wire, Config, Core};
    pub use version::CURRENT_METADATA;
    pub use wire::Wire;

    // dependency re-exports
    pub use async_trait::async_trait;
    pub use futures::{self, io, prelude::*, task};
    pub use futures_codec::{Bytes, BytesMut};
    pub use serde::{Deserialize, Deserializer, Serialize, Serializer};
    pub use std::pin::Pin;
    pub use xactor::{
        block_on, sleep, spawn, timeout, Actor, Addr, Context, Handler, Message, StreamHandler,
    };
}

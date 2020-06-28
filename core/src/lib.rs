//! `yggy` core types and interfaces.
//!

#[macro_use]
extern crate lazy_static;

mod config;
pub mod error;
pub mod interfaces;
pub mod types;
pub mod version;

#[doc(inline)]
pub use config::Config;
#[doc(inline)]
pub use error::Error;
#[doc(inline)]
pub use interfaces::Core;

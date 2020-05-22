//! `yggy`
//!

#![warn(rust_2018_idioms, missing_debug_implementations, missing_docs)]

#[macro_use]
extern crate log;

mod config;
pub mod core;
pub mod error;
pub mod notes;
pub mod types;

#[doc(inline)]
pub use config::Config;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

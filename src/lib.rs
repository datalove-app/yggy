//! `yggy`
//!

// #![warn(rust_2018_idioms, missing_debug_implementations, missing_docs)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

mod config;
pub mod core_interfaces;
pub mod core_types;
pub mod error;
pub mod multicast;
pub mod notes;
pub mod tun;

#[doc(inline)]
pub use config::Config;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

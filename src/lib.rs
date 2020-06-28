//! `yggy`
//!

// #![warn(rust_2018_idioms, missing_debug_implementations, missing_docs)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

pub mod adapters;
mod conn;
pub mod core;
mod dialer;
mod listener;
mod node;
pub mod notes;
pub mod services;
// pub mod whitepaper;

#[doc(inline)]
pub use crate::core::Core;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

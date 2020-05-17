//!

#![warn(rust_2018_idioms, missing_debug_implementations, missing_docs)]

pub mod config;
pub mod core;
pub mod crypto;
pub mod error;
pub mod tuntap;
pub mod version;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

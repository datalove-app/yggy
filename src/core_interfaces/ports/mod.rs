//! Ports required by core services.
//!
//! Ports are interfaces defined by core services that represent a
//! non-core use case or service
//!
//! Duplex ports:
//!     shields core services + types from details of underlying drivers that carry out the use cases
//!         impls are most likely driver-specific
//!     ====================
//!     >>>
//!
//! Input (outer) ports:
//!     represents external use case APIs exposed by core services
//!     initialized and owned (or implemented?) by a primary adapter
//!         impls are likely driver-specific
//!     ====================
//!     >>>
//!
//! Output ports:
//!     represents core service requirements for non-core use cases (persistance, network calls, etc)
//!     implemented by a secondary adapter
//!         impls are likely driver-specific
//!     ====================
//!     >>>

// mod admin;
pub mod link;
pub mod multicast;
#[path = "tun.rs"]
pub mod tun;

pub use self::tun::Tun;
pub use link::Link;
pub use multicast::Multicast;

//  Input (incoming) ports are interfaces that are:
//      - generic OVER core services, and will call core service methods
//      - ?? initialized and owned by a root application type
//      -
//  Output (outgoing) ports are interfaces that are:
//      - generics FOR core services, such will be called by core services
//      - ?? initalized and owned by core services
//      -

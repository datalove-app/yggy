//! Traits for adapters used by core services.
//!
//! The adapter traits in this module either:
//! - represent ...
//! - represent ...

mod multicast;
mod tun;

//  Driver/Driving (primary, use-case, UI-facing) adapters:
//      ** translates driver input to core types + service calls
//      owns/is given/generic over a port impl/interface
//          can also
//      are given/has access to a "port impl" (to call core services)
//          can be constructed by core services (w/ a "port impl")
//          "port impl" is likely an addr to a port actor/system
//      ====================
//      >>> DRIVING ADAPTER can be generic over a PORT
//          which shields adapters from protocol changes
//      == examples:
//          rest/news.rs: takes `impl NewsPort`
//          AdminService: would wrap an `impl AdminAPI` that would translate (cli, socket) reqs to AdminAPI service calls
//          ?TunConn:
//
//  Driven (secondary, infra-facing) adapters:
//      ** translates core use cases + types into driver types + method calls
//      owns/is given/has access to a driver instance
//          can also be a collection of interfaces
//      are given to/inited by core services (to be called)
//          can be constructed by main/root service
//              likely an addr to the adapter actor/system
//      ====================
//      >>> Driven adaapter
//      >>> CORE SERVICES can be generic over a PORT
//              aka driven adapter
//              or can have `type Port: Port = DrivenAdapter`
//          which shields core from adapter impl changes
//      == examples:
//          gateway/news_gateway.rs: calls `news_driver` (db)
//          Tun:

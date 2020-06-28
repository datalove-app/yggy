use crate::interfaces::Core;
use xactor::Actor;

///
/// TODO
/// Multicast represents the multicast advertisement and discovery mechanism used
/// by Yggdrasil to find peers on the same subnet. When a beacon is received on a
/// configured multicast interface, Yggdrasil will attempt to peer with that node
/// automatically.
///
/// From https://yggdrasil-network.github.io/2018/07/28/addressing.html:
/// "Having a /64 address range allows a Yggdrasil node to operate as a router, where it advertises a route (using normal IPv6 tools, like radvd) allowing the rest of its LAN to reach Yggdrasil without needing to run the code. This allows unsupported and legacy devices (phones, network printers, game consoles, old OSs, etc.) to reach the network, or low-power / battery-powered devices (IoT) to off-load cryptography and routing logic onto something with fewer constraints.""
pub trait MulticastAdapter<C: Core>
where
    Self: Actor,
{
}

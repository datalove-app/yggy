use super::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Contains configuration options necessary for an Yggdrasil node to run. You
/// will need to supply one of these structs to the Yggdrasil core when starting
/// a node.
/// TODO? generic over key pairs?
#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    /// List of connection strings for outbound peer connections in URI format,
    /// e.g. `tcp://a.b.c.d:e` or `socks://a.b.c.d:e/f.g.h.i:j`.
    /// These connections will obey the operating system routing table, therefore
    /// you should use this section when you may connect via different interfaces.
    #[serde(rename = "Peers")]
    peers: Peers,

    /// List of connection strings for outbound peer connections in URI format,
    /// arranged by source interface, e.g. `{ "eth0": [ tcp://a.b.c.d:e ] }`.
    /// Note that SOCKS peerings will NOT be affected by this option and should
    /// go in the "Peers" section instead.
    #[serde(rename = "InterfacePeers")]
    interface_peers: InterfacePeers,

    /// Listen addresses for incoming connections. You will need to add listeners
    /// in order to accept incoming peerings from non-local nodes. Multicast
    /// peer discovery will work regardless of any listeners set here. Each
    /// listener should be specified in URI format as above,
    /// e.g. `tcp://0.0.0.0:0` or `tcp://[::]:0` to listen on all interfaces.
    #[serde(rename = "Listen", default)]
    listen: ListenAddresses,

    /// Listen address for admin connections. Default is to listen for local
    /// connections either on TCP/9001 or a UNIX socket depending on your
    /// platform. Use this value for `yggyctl -endpoint=X`. To disable the admin
    /// socket, use the value "none" instead.
    #[serde(rename = "AdminListen", default)]
    admin_listen: Option<PeerURI>,
    // /// Regular expressions for which interfaces multicast peer discovery\nshould be enabled on. If none specified, multicast peer discovery is\ndisabled. The default value is .* which uses all interfaces."`
    // MulticastInterfaces         []string
    // /// List of peer encryption public keys to allow incoming TCP peering\nconnections from. If left empty/undefined then all connections will\nbe allowed by default. This does not affect outgoing peerings, nor\ndoes it affect link-local peers discovered via multicast."`
    // AllowedEncryptionPublicKeys []string
    // /// Your public encryption key. Your peers may ask you for this to put\ninto their AllowedEncryptionPublicKeys configuration."`
    // EncryptionPublicKey         string
    // /// Your private encryption key. DO NOT share this with anyone!"`
    // EncryptionPrivateKey        string
    // /// Your public signing key. You should not ordinarily need to share\nthis with anyone."`
    // SigningPublicKey            string
    // /// Your private signing key. DO NOT share this with anyone!"`
    // SigningPrivateKey           string
    /// The port number to be used for the link-local TCP listeners for the
    /// configured MulticastInterfaces. This option does not affect listeners
    /// specified in the Listen option. Unless you plan to firewall link-local
    /// traffic, it is best to leave this as the default value of 0. This option
    /// cannot currently be changed by reloading config during runtime.
    #[serde(rename = "LinkLocalTCPPort", default)]
    link_local_tcp_port: u16,

    /// Local network interface name for TUN adapter, or "auto" to select an
    /// interface automatically, or "none" to run without TUN.
    #[serde(rename = "IfName", default)]
    interface_name: InterfaceName,

    /// Maximum Transmission Unit (MTU) size for your local TUN
    /// interface.
    /// Default is the largest supported size for your platform. The lowest
    /// possible value is 1280.
    #[serde(rename = "IfMTU")]
    interface_mtu: MTU,

    /// Controls who can send/receive network traffic to/from this node. This is
    /// useful if you want to protect this node without resorting to using a
    /// real firewall. This does not affect traffic being routed via this node
    /// to somewhere else.
    ///
    /// Rules are prioritised as follows: blacklist, whitelist, always allow
    /// outgoing, direct, remote.
    #[serde(rename = "SessionFirewall")]
    firewall: SessionFirewall,

    /// Allow tunneling non-Yggdrasil traffic over Yggdrasil. This effectively
    /// allows you to use Yggdrasil to route or bridge to other networks,
    /// similar to a VPN tunnel. Tunnelling works between any two nodes and does
    /// not require them to be directly peered.
    #[serde(rename = "TunnelRouting")]
    tunnel_routing: TunnelRouting,

    /// Advanced options for tuning the switch. Normally you will not need nto
    /// edit these options.
    #[serde(rename = "SwitchOptions")]
    switch_opts: SwitchOptions,

    /// By default, nodeinfo contains some defaults including the platform,
    /// architecture and Yggdrasil version. These can help when surveying the
    /// network and diagnosing network routing problems. Enabling nodeinfo
    /// privacy prevents this, so that only items specified in `"NodeInfo"` are
    /// sent back if specified.
    #[serde(rename = "NodeInfoPrivacy")]
    node_info_privacy: bool,
    // /// Optional node info. This must be a { \"key\": \"value\", ... } map\nor set as null. This is entirely optional but, if set, is visible\nto the whole network on request."`
    // NodeInfo                    map[string]interface{}
}

/// Controls the session firewall configuration.
#[derive(Debug, Deserialize, Serialize)]
pub struct SessionFirewall {
    /// Enable or disable the session firewall. If disabled, network traffic
    /// from any node will be allowed. If enabled, the below rules apply.
    #[serde(rename = "Enable")]
    enabled: bool,

    /// Allow network traffic from directly connected peers.
    #[serde(rename = "AllowFromDirect")]
    allow_from_direct: bool,

    /// Allow network traffic from remote nodes on the network that you are not
    /// directly peered with.
    #[serde(rename = "AllowFromRemote")]
    allow_from_remote: bool,

    /// Allow outbound network traffic regardless of `AllowFromDirect`
    /// or `AllowFromRemote`. This does allow a remote node to send unsolicited
    /// traffic back to you for the length of the session.
    #[serde(rename = "AlwaysAllowOutbound")]
    always_allow_outbound: bool,

    /// List of public keys from which network traffic is always accepted,
    /// regardless of `AllowFromDirect` or `AllowFromRemote`.
    /// TODO
    #[serde(rename = "WhitelistEncryptionPublicKeys")]
    whitelisted_encryption_pub_keys: Vec<String>,

    /// List of public keys from which network traffic is always rejected,
    /// regardless of the whitelist, `AllowFromDirect` or `AllowFromRemote`.
    /// TODO
    #[serde(rename = "BlacklistEncryptionPublicKeys")]
    blacklisted_encryption_pub_keys: Vec<String>,
}

/// Contains the crypto-key routing tables for tunneling regular IPv4 or IPv6
/// subnets across the Yggdrasil network.
#[derive(Debug, Deserialize, Serialize)]
pub struct TunnelRouting {
    /// Enable or disable tunnel routing.
    #[serde(rename = "Enable")]
    enable: bool,

    /// IPv6 subnets belonging to remote nodes, mapped to the node's public key,
    /// e.g. `{ "aaaa:bbbb:cccc::/e": "boxpubkey", ... }`.
    #[serde(rename = "Ipv4RemoteSubnets")]
    ipv4_remote_subnets: Ipv4Subnets,

    /// IPv6 subnets belonging to this node's end of the tunnels. Only traffic
    /// from these ranges (or the Yggdrasil node's IPv6 address/subnet) will be
    /// tunnelled.
    #[serde(rename = "Ipv4LocalSubnets")]
    ipv4_local_subnets: Ipv4Subnets,

    /// IPv4 subnets belonging to remote nodes, mapped to the node's public key,
    /// e.g. `{ "a.b.c.d/e": "boxpubkey", ... }`.
    #[serde(rename = "Ipv6RemoteSubnets")]
    ipv6_remote_subnets: Ipv6Subnets,

    /// IPv4 subnets belonging to this node's end of the tunnels. Only traffic
    /// from these ranges will be tunnelled.
    #[serde(rename = "Ipv6LocalSubnets")]
    ipv6_local_subnets: Ipv6Subnets,
}

/// Contains tuning options for the switch. These are advanced options and
/// shouldn't be changed unless necessary.
#[derive(Debug, Deserialize, Serialize)]
pub struct SwitchOptions {
    /// Maximum size of all switch queues combined (in bytes).
    #[serde(rename = "MaxTotalQueueSize")]
    max_total_queue_size: u64,
}

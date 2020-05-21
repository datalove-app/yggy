use crate::base::Tun;

const SIOAIFADDR_IN6: u32           = 2155899162;
const IN6_IFF_NODAD: u8             = 0x0020;
const IN6_IFF_SECURED: u8           = 0x0400;
const ND6_INFINITE_LIFETIME: u64    = 0xFFFFFFFF;

#[derive(Debug)]
struct In6AddrLifetime {
    expire: f64,
    preferred: f64,
    vltime: u32,
    pltime: u32,
}

#[derive(Debug)]
struct SockAddrIn6 struct {
	len: u8,
	family: u8,
	port: u8,
	flowinfo: u32,
	addr: [u16; 8],
	scope_id: u32,
}

#[derive(Debug)]
struct In6AliasReq struct {
	name: [u8; 16],
	addr: SockAddrIn6,
	dst_addr: SockAddrIn6,
	prefix_mask: SockAddrIn6,
	flags: u32,
	lifetime: In6AddrLifetime,
}

#[derive(Debug)]
struct IFreq struct {
	name: [u8; 16],
	mtu: u32,
}

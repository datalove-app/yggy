use super::{BoxNonce, BoxPublicKey, Coords, Handle, RootUpdate, WireCoords, MTU};
use crate::error::Error;
use std::io::{Read, Write};

///
/// TODO:
#[derive(Clone, Debug)]
pub struct Traffic;

///
/// TODO:
#[derive(Clone, Debug)]
pub struct ProtocolTraffic;

///
/// TODO:
#[derive(Clone, Debug)]
pub struct LinkProtocolTraffic;

// ///
// /// TODO
// pub enum Header {
//     Traffic = 0,
//     ProtocolTraffic,
//     LinkProtocolTraffic,
//     RootUpdate,
//     SessionPing,
//     SessionPong,
//     // TODO
//     // DHTLookupRequest,
//     // DHTLookupResponse,
//     // NodeInfoRequest,
//     // NodeInfoResponse,
// }

// /// Encodes and decodes
// pub trait Wire: Sized {
//     fn wire_len(&self) -> Option<usize> {
//         None
//     }

//     fn decode<R: Read>(reader: &mut R) -> Result<(Self, usize), Error>;

//     fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error>;
// }

// impl Wire for i64 {
//     fn wire_len(&self) -> Option<usize> {
//         // let mut i = 1;
//         // for
//         unimplemented!()
//     }

//     fn decode<R: Read>(reader: &mut R) -> Result<(Self, usize), Error> {
//         unimplemented!()
//     }

//     fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
//         unimplemented!()
//     }
// }

// impl Wire for u64 {
//     fn decode<R: Read>(reader: &mut R) -> Result<(Self, usize), Error> {
//         unimplemented!()
//     }

//     fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
//         // let mut b = [0u8; 10];
//         // let mut i = b.len() - 1;
//         // let mut e = self.to_be_bytes();

//         // b[i] =
//         // while e >> 7 != 0 {
//         //     i -= 1;
//         //     b[i] = self | 0x80;
//         // }

//         unimplemented!()
//     }
// }

// impl Wire for Coords {
//     fn decode<R: Read>(reader: &mut R) -> Result<(Self, usize), Error> {
//         unimplemented!()
//     }

//     fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
//         unimplemented!()
//     }
// }

// impl Wire for WireCoords {
//     fn decode<R: Read>(reader: &mut R) -> Result<(Self, usize), Error> {
//         unimplemented!()
//     }

//     fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
//         // writer.write_all(self.0).map_err(Error::WireWriteError)
//         unimplemented!()
//     }
// }

// impl Wire for RootUpdate {
//     fn decode<R: Read>(reader: &mut R) -> Result<(Self, usize), Error> {
//         unimplemented!()
//     }

//     fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
//         unimplemented!()
//     }
// }

// impl Wire for SessionPingPong {
//     fn decode<R: Read>(reader: &mut R) -> Result<(Self, usize), Error> {
//         unimplemented!()
//     }

//     fn encode<W: Write>(&self, writer: &mut W) -> Result<usize, Error> {
//         unimplemented!()
//     }
// }

// impl Wire for u64
// impl Wire for i64 (? encoded as a special u64)
// impl Wire for Coords
// impl Wire for RootUpdate
// impl Wire for TrafficPacket
// impl Wire for ProtocolTrafficPacket
// impl Wire for LinkProtocolTrafficPacket
// impl Wire for SessionPingPong
// impl Wire for NodeInfoReqRes
// impl Wire for DHTRequest
// impl Wire for DHTResponse

use super::{
    BoxNonce, BoxPublicKey, Coords, Handle, SessionPingPong, SwitchMessage, WireCoords, MTU,
};
use crate::error::Error;
use std::io::{Read, Write};

///
/// TODO
pub enum Header {
    Traffic = 0,
    ProtocolTraffic,
    LinkProtocolTraffic,
    SwitchMessage,
    SessionPing,
    SessionPong,
    // DHTLookupRequest,
    // DHTLookupResponse,
    // NodeInfoRequest,
    // NodeInfoResponse,
}

/// Encodes and decodes
pub trait Wire: Sized {
    fn wire_len(&self) -> Option<usize> {
        None
    }

    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error>;

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error>;
}

impl Wire for i64 {
    fn wire_len(&self) -> Option<usize> {
        None
    }

    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
        unimplemented!()
    }

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error> {
        unimplemented!()
    }
}

impl Wire for u64 {
    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
        unimplemented!()
    }

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error> {
        unimplemented!()
    }
}

impl Wire for Coords {
    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
        unimplemented!()
    }

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error> {
        unimplemented!()
    }
}

impl Wire for WireCoords {
    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
        unimplemented!()
    }

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error> {
        // writer.write_all(self.0).map_err(Error::WireWriteError)
        unimplemented!()
    }
}

///
/// TODO:
#[derive(Clone, Debug)]
pub struct Traffic;

impl Wire for Traffic {
    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
        unimplemented!()
    }

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error> {
        unimplemented!()
    }
}

///
/// TODO:
#[derive(Clone, Debug)]
pub struct ProtocolTraffic;

impl Wire for ProtocolTraffic {
    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
        unimplemented!()
    }

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error> {
        unimplemented!()
    }
}

///
/// TODO:
#[derive(Clone, Debug)]
pub struct LinkProtocolTraffic;

impl Wire for LinkProtocolTraffic {
    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
        unimplemented!()
    }

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error> {
        unimplemented!()
    }
}

impl Wire for SwitchMessage {
    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
        unimplemented!()
    }

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error> {
        unimplemented!()
    }
}

impl Wire for SessionPingPong {
    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
        unimplemented!()
    }

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error> {
        unimplemented!()
    }
}

// impl Wire for u64
// impl Wire for i64 (? encoded as a special u64)
// impl Wire for Coords
// impl Wire for SwitchMessage
// impl Wire for TrafficPacket
// impl Wire for ProtocolTrafficPacket
// impl Wire for LinkProtocolTrafficPacket
// impl Wire for SessionPingPong
// impl Wire for NodeInfoReqRes
// impl Wire for DHTRequest
// impl Wire for DHTResponse

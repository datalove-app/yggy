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

///
pub trait WireHeader: Sized {
    fn wire_len(&self) -> Option<usize> {
        None
    }

    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error>;

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error>;
}

impl WireHeader for i64 {
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

impl WireHeader for u64 {
    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
        unimplemented!()
    }

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error> {
        unimplemented!()
    }
}

impl WireHeader for Coords {
    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
        unimplemented!()
    }

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error> {
        unimplemented!()
    }
}

impl WireHeader for WireCoords {
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

impl WireHeader for Traffic {
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

impl WireHeader for ProtocolTraffic {
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

impl WireHeader for LinkProtocolTraffic {
    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
        unimplemented!()
    }

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error> {
        unimplemented!()
    }
}

impl WireHeader for SwitchMessage {
    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
        unimplemented!()
    }

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error> {
        unimplemented!()
    }
}

impl WireHeader for SessionPingPong {
    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
        unimplemented!()
    }

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error> {
        unimplemented!()
    }
}

// impl WireHeader for u64
// impl WireHeader for i64 (? encoded as a special u64)
// impl WireHeader for Coords
// impl WireHeader for SwitchMessage
// impl WireHeader for TrafficPacket
// impl WireHeader for ProtocolTrafficPacket
// impl WireHeader for LinkProtocolTrafficPacket
// impl WireHeader for SessionPingPong
// impl WireHeader for NodeInfoReqRes
// impl WireHeader for DHTRequest
// impl WireHeader for DHTResponse

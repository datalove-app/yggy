use crate::{
    error::{Error, WireError},
    types::{BoxNonce, BoxPublicKey, Coords, Handle, RootUpdate, WireCoords, MTU},
};
use std::io::{self, Read, Write};

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

/// Encodes and decodes
pub trait Wire: Sized {
    const LENGTH: usize;

    fn wire_len(&self) -> Option<usize> {
        None
    }

    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error>;

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error>;
}

impl Wire for u64 {
    const LENGTH: usize = 10;

    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
        let mut len = 0usize;
        let mut num = 0u64;
        for b in reader.bytes() {
            let b = b.map_err(WireError::Read)?;
            num <<= 7;
            num |= (b & 0x7f) as u64;
            len += 1;
            if b & 0x80 == 0 {
                break;
            }
        }

        Ok((num, len))
    }

    fn encode<W: Write>(&self, mut writer: W) -> Result<usize, Error> {
        let mut bytes = [0u8; Self::LENGTH];
        let mut idx = Self::LENGTH - 1;
        let mut num = *self;

        bytes[idx] = num as u8 & 0x7f;
        loop {
            num >>= 7;
            if num != 0 {
                idx -= 1;
                bytes[idx] = num as u8 | 0x80;
            } else {
                break;
            }
        }

        writer.write_all(&bytes[idx..]).map_err(WireError::Write)?;
        Ok(Self::LENGTH - idx)
    }
}


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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};

    macro_rules! test_wire {
        ($ty:ty => $decoded:expr, $encoded:expr, $len:expr) => {
            let mut b = vec![];
            let len = <$ty>::encode(&$decoded, &mut b).unwrap();
            assert_eq!(len, $len);
            assert_eq!(b, $encoded);

            let (decoded, size_read) = <$ty>::decode(&*b).unwrap();
            assert_eq!(size_read, $len);
            assert_eq!($decoded, decoded);
        };
    }

    #[test]
    fn wire_u64() {
        test_wire!(u64 => 127, vec![0x7f], 1);
        test_wire!(u64 => 128, vec![0x81, 0x00], 2);
        test_wire!(u64 => 255, vec![0x81, 0x7f], 2);
        test_wire!(u64 => 256, vec![0x82, 0x00], 2);
        test_wire!(u64 => 16384, vec![0x81, 0x80, 0x00], 3);
        test_wire!(
            u64 => u64::MAX,
            vec![0x81, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0x7f],
            10
        );
    }
}

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
    /// Maximum length of the type on the wire, sans the payload.
    const LENGTH: usize;

    // /// Length of the type on the wire.
    // fn len(&self) -> Option<usize> {
    //     None
    // }

    /// Decodes the type from a `Read`.
    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error>;

    /// Encodes the type to a `Write`.
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

impl Wire for i64 {
    const LENGTH: usize = 10;

    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
        let (uint64, size) = <u64>::decode(reader)?;
        let int64 = (((uint64 >> 1) as i64) ^ -((uint64 & 1) as i64));
        Ok((int64, size))
    }

    fn encode<W: Write>(&self, writer: W) -> Result<usize, Error> {
        let uint64 = ((self >> 63) ^ (self << 1)) as u64;
        <u64>::encode(&uint64, writer)
    }
}

// impl Wire for Coords {
//     const LENGTH: usize = 0;

//     fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
//         unimplemented!()
//     }

//     fn encode<W: Write>(&self, writer: W) -> Result<usize, Error> {
//         let coords_len = <u64>::encode(&(self.len() as u64), writer.by_ref())?;
//         let bytes = [0u8; coords_len + self.len()];
//         // writer.by_ref().write_all().map_err(WireError::Write)?;
//         unimplemented!()
//     }
// }

impl Wire for WireCoords {
    const LENGTH: usize = 0;

    fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
        unimplemented!()
    }

    fn encode<W: Write>(&self, mut writer: W) -> Result<usize, Error> {
        let coords_len = <u64>::encode(&(self.len() as u64), writer.by_ref())?;
        // let bytes = [0u8; coords_len + self.len()];
        // writer.by_ref().write_all().map_err(WireError::Write)?;
        unimplemented!()
    }
}

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

    macro_rules! test {
        ($ty:ty => $dec:expr, $enc:expr, $len:expr) => {
            let mut enc = vec![];
            let len = <$ty>::encode(&$dec, &mut enc).unwrap();
            assert_eq!(len, $len);
            assert_eq!(enc, $enc);

            let (dec, len) = <$ty>::decode(&*enc).unwrap();
            assert_eq!(len, $len);
            assert_eq!(dec, $dec);
        };
    }

    #[test]
    fn wire_u64() {
        test!(u64 => 0, vec![0x00], 1);
        test!(u64 => 127, vec![0x7f], 1);
        test!(u64 => 128, vec![0x81, 0x00], 2);
        test!(u64 => 255, vec![0x81, 0x7f], 2);
        test!(u64 => 256, vec![0x82, 0x00], 2);
        test!(u64 => 16384, vec![0x81, 0x80, 0x00], 3);
        test!(
            u64 => u64::MAX,
            vec![0x81, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0x7f],
            10
        );
    }

    #[test]
    fn wire_i64() {
        test!(i64 => 0, vec![0x00], 1);
        test!(i64 => 127, vec![0x81, 0x7e], 2);
        test!(i64 => 128, vec![0x82, 0x00], 2);
        test!(i64 => 255, vec![0x83, 0x7e], 2);
        test!(i64 => 256, vec![0x84, 0x00], 2);
        test!(i64 => 16384, vec![0x82, 0x80, 0x00], 3);

        test!(i64 => -127, vec![0x81, 0x7d], 2);
        test!(i64 => -128, vec![0x81, 0x7f], 2);
        test!(i64 => -255, vec![0x83, 0x7d], 2);
        test!(i64 => -256, vec![0x83, 0x7f], 2);
        test!(i64 => -16384, vec![0x81, 0xff, 0x7f], 3);
        test!(
            i64 => i64::MIN,
            vec![0x81, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0x7f],
            10
        );
        test!(
            i64 => i64::MAX,
            vec![0x81, 0xff, 0xff, 0xff, 0xff,
                0xff, 0xff, 0xff, 0xff, 0x7e],
            10
        );
    }
}

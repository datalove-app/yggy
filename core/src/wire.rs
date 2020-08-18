use crate::{
    dev::*,
    error::{Error, WireError},
    types::{BoxNonce, BoxPublicKey, Coords, Handle, RootUpdate, WireCoords, MTU},
};
use futures_codec::{Decoder, Encoder, FramedRead, FramedWrite};
use std::marker::PhantomData;

///
#[derive(Debug)]
pub enum Packet {
    Traffic(Traffic),
    Protocol(ProtocolTraffic),
    LinkProtocol(LinkProtocolTraffic),
}

impl Packet {
    #[inline]
    pub fn len(&self) -> usize {
        unimplemented!()
    }
}

impl Wire for Packet {
    #[inline]
    fn decode(src: &mut BytesMut) -> Result<Option<Self>, WireError> {
        unimplemented!()
    }

    #[inline]
    fn encode(self, dst: &mut BytesMut) -> Result<(), WireError> {
        unimplemented!()
    }
}

///
/// TODO:
#[derive(Clone, Debug)]
pub struct Traffic;

impl Wire for Traffic {
    #[inline]
    fn decode(src: &mut BytesMut) -> Result<Option<Self>, WireError> {
        unimplemented!()
    }

    #[inline]
    fn encode(self, dst: &mut BytesMut) -> Result<(), WireError> {
        unimplemented!()
    }
}

///
/// TODO:
#[derive(Clone, Debug)]
pub enum ProtocolTraffic {
    SessionPing,
    SessionPong,
    // DHTRequest,
    // DHTResponse
}

impl Wire for ProtocolTraffic {
    #[inline]
    fn decode(src: &mut BytesMut) -> Result<Option<Self>, WireError> {
        unimplemented!()
    }

    #[inline]
    fn encode(self, dst: &mut BytesMut) -> Result<(), WireError> {
        unimplemented!()
    }
}

///
/// TODO:
#[derive(Clone, Debug)]
pub enum LinkProtocolTraffic {
    RootUpdate,
}

impl Wire for LinkProtocolTraffic {
    #[inline]
    fn decode(src: &mut BytesMut) -> Result<Option<Self>, WireError> {
        unimplemented!()
    }

    #[inline]
    fn encode(self, dst: &mut BytesMut) -> Result<(), WireError> {
        unimplemented!()
    }
}

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

/// A wire-encodable type.
pub trait Wire: Sized {
    /// Returns a framed `Stream` that wraps the provided `AsyncRead` and from
    /// which this type can be read.
    #[inline]
    fn stream<R: AsyncRead + Unpin>(reader: R) -> WireReader<Self, R> {
        WireReader::<Self, R>::from(reader)
    }

    /// Returns a framed 'Sink` that can write this type to the provided
    /// `AsyncWrite`.
    #[inline]
    fn sink<W: AsyncWrite + Unpin>(writer: W) -> WireWriter<Self, W> {
        WireWriter::<Self, W>::from(writer)
    }

    fn decode(src: &mut BytesMut) -> Result<Option<Self>, WireError>;

    fn encode(self, dst: &mut BytesMut) -> Result<(), WireError>;
}

impl Wire for u64 {
    #[inline]
    fn decode(src: &mut BytesMut) -> Result<Option<Self>, WireError> {
        if src.is_empty() {
            return Ok(None);
        }

        let mut len = 0usize;
        let mut num = 0u64;
        for (i, b) in src.iter().enumerate() {
            num <<= 7;
            num |= (b & 0x7f) as u64;
            len += 1;
            if b & 0x80 == 0 {
                src.split_to(i);
                break;
            }

            if i > 9 {
                return Err(WireError::Codec("expected u64"));
            }
        }

        Ok(Some(num))
    }

    #[inline]
    fn encode(self, dst: &mut BytesMut) -> Result<(), WireError> {
        let mut bytes = [0u8; 10];
        let mut idx = 9usize;
        let mut src = self;

        bytes[idx] = src as u8 & 0x7f;
        loop {
            src >>= 7;
            if src == 0 {
                break;
            }
            idx -= 1;
            bytes[idx] = src as u8 | 0x80;
        }

        dst.extend_from_slice(&bytes[idx..]);
        Ok(())
    }
}

/// Converts `i64` to `u64`, then writes it to the wire.
///
/// Non-negative integers are mapped to even integers: 0 -> 0, 1 -> 2, etc.
/// Negative integers are mapped to odd integers: -1 -> 1, -2 -> -3, etc.
/// This means that the least significant bit is a sign bit.
/// This is known as zigzag encoding.
impl Wire for i64 {
    #[inline]
    fn decode(src: &mut BytesMut) -> Result<Option<Self>, WireError> {
        match <u64>::decode(src).or(Err(WireError::Codec("expected i64")))? {
            None => Ok(None),
            Some(uint64) => {
                let int64 = ((uint64 >> 1) as i64) ^ -((uint64 & 1) as i64);
                Ok(Some(int64))
            }
        }
    }

    #[inline]
    fn encode(self, dst: &mut BytesMut) -> Result<(), WireError> {
        let uint64 = ((self >> 63) ^ (self << 1)) as u64;
        uint64.encode(dst)
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

// impl Wire for WireCoords {
//     const LENGTH: usize = 0;

//     fn decode<R: Read>(reader: R) -> Result<(Self, usize), Error> {
//         unimplemented!()
//     }

//     fn encode<W: Write>(&self, mut writer: W) -> Result<usize, Error> {
//         // let coords_len = <u64>::encode(&(self.len() as u64), writer.by_ref())?;
//         // let bytes = [0u8; coords_len + self.len()];
//         // writer.by_ref().write_all().map_err(WireError::Write)?;
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

/// Produces `Wire` types from a wrapped `AsyncRead`.
#[derive(Debug)]
pub struct WireReader<T: Wire, R: AsyncRead + Unpin>(FramedRead<R, WireCodec<T>>);

impl<T: Wire, R: AsyncRead + Unpin> From<R> for WireReader<T, R> {
    fn from(reader: R) -> Self {
        Self(FramedRead::new(reader, WireCodec::<T>::default()))
    }
}

impl<T: Wire, R: AsyncRead + Unpin> Stream for WireReader<T, R> {
    type Item = Result<T, WireError>;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Option<Self::Item>> {
        let inner = &mut self.0;
        futures::pin_mut!(inner);
        inner.poll_next(cx)
    }
}

/// Produces `Wire` types from a wrapped `AsyncRead`.
#[derive(Debug)]
pub struct WireWriter<T: Wire, W: AsyncWrite + Unpin>(FramedWrite<W, WireCodec<T>>);

impl<T: Wire, W: AsyncWrite + Unpin> From<W> for WireWriter<T, W> {
    fn from(writer: W) -> Self {
        Self(FramedWrite::new(writer, WireCodec::<T>::default()))
    }
}

impl<T: Wire, W: AsyncWrite + Unpin> Sink<T> for WireWriter<T, W> {
    type Error = WireError;
    fn poll_ready(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), Self::Error>> {
        let inner = &mut self.0;
        futures::pin_mut!(inner);
        inner.poll_ready(cx)
    }
    fn start_send(mut self: Pin<&mut Self>, item: T) -> Result<(), Self::Error> {
        let inner = &mut self.0;
        futures::pin_mut!(inner);
        inner.start_send(item)
    }
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), Self::Error>> {
        let inner = &mut self.0;
        futures::pin_mut!(inner);
        inner.poll_flush(cx)
    }
    fn poll_close(
        mut self: Pin<&mut Self>,
        cx: &mut task::Context,
    ) -> task::Poll<Result<(), Self::Error>> {
        let inner = &mut self.0;
        futures::pin_mut!(inner);
        inner.poll_close(cx)
    }
}

/// Zero-sized type representing the `Wire` codec.
#[derive(Debug)]
struct WireCodec<T: Wire>(PhantomData<T>);

impl<T: Wire> Default for WireCodec<T> {
    #[inline]
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: Wire> Decoder for WireCodec<T> {
    type Item = T;
    type Error = WireError;
    #[inline]
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        T::decode(src)
    }
}

impl<T: Wire> Encoder for WireCodec<T> {
    type Item = T;
    type Error = WireError;
    #[inline]
    fn encode(&mut self, mut src: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        T::encode(src, dst)
    }
}

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

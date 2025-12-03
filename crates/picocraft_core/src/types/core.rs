use crate::prelude::*;

impl Encode for bool {
    async fn encode<W: Write>(&self, mut buffer: W) -> Result<(), EncodeError<W::Error>> {
        Ok(buffer.write_u8(u8::from(*self)).await?)
    }
}

impl Decode for bool {
    async fn decode<R: Read>(mut buffer: R) -> Result<Self, DecodeError<R::Error>> {
        match buffer.read_u8().await? {
            0x00 => Ok(false),
            0x01 => Ok(true),
            _ => Err(DecodeError::<R::Error>::InvalidBoolean),
        }
    }
}

impl Encode for () {
    async fn encode<W: Write>(&self, _buffer: W) -> Result<(), EncodeError<W::Error>> {
        Ok(())
    }
}

impl Decode for () {
    async fn decode<R: Read>(_buffer: R) -> Result<Self, DecodeError<R::Error>> {
        Ok(())
    }
}

macro_rules! impl_decode_integer {
    ($($ty:ty), *) => {
        $(
        impl crate::packet::Decode for $ty {

            async fn decode<R>(mut buffer: R) -> ::core::result::Result<Self, crate::packet::DecodeError<R::Error>>
            where R: ::embedded_io_async::Read
            {
                const SIZE: usize = core::mem::size_of::<$ty>();

                let mut buf = [0; SIZE];

                buffer.read_exact(&mut buf).await?;
                Ok(<$ty>::from_be_bytes(buf[..SIZE].try_into().expect("decoding numbers should always work")))
            }
        }
        )*
    };
}

impl_decode_integer!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, f32, f64
);

macro_rules! impl_encode_integer {
    ($($ty:ty), *) => {
        $(
        impl crate::packet::Encode for $ty {

            async fn encode<W>(&self, mut buffer: W) -> ::core::result::Result<(), crate::packet::EncodeError<W::Error>>
            where W: ::embedded_io_async::Write
            {
                buffer.write_all(&self.to_be_bytes()).await?;
                Ok(())
            }

        }
        )*
    };
}

impl_encode_integer!(
    u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, f32, f64
);

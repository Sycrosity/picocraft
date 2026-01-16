use crate::prelude::*;

impl Encode for () {
    async fn encode<W: Write>(&self, _buffer: W) -> Result<(), EncodeError> {
        Ok(())
    }
}

impl Decode for () {
    async fn decode<R: Read>(_buffer: R) -> Result<Self, DecodeError> {
        Ok(())
    }
}

impl Encode for &'static [u8] {
    async fn encode<W: Write>(&self, mut buffer: W) -> Result<(), EncodeError> {
        buffer.write_all(self).await?;
        Ok(())
    }
}

macro_rules! impl_decode_integer {
    ($($ty:ty), *) => {
        $(
        impl crate::packet::Decode for $ty {

            async fn decode<R>(mut buffer: R) -> ::core::result::Result<Self, crate::errors::DecodeError>
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

            async fn encode<W>(&self, mut buffer: W) -> ::core::result::Result<(), crate::errors::EncodeError>
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

macro_rules! impl_encode_decode_tuples {
    ( $( $name:ident )+ ) => {

        impl<$($name: Encode),+> crate::packet::Encode for ($($name,)+)
        {

            #[inline]
            async fn encode<W>(
                &self,
                mut buffer: W
            ) -> ::core::result::Result<(), crate::errors::EncodeError>
            where
                W: ::embedded_io_async::Write,
            {
                #[allow(non_snake_case)]
                let ($($name,)+) = self;

                $($name.encode(&mut buffer).await?;)+
                Ok(())
            }
        }

        impl<$($name: Decode),+> crate::packet::Decode for ($($name,)+) {

            #[inline]
            async fn decode<R>(
                mut buffer: R
            ) -> ::core::result::Result<Self, crate::errors::DecodeError>
            where
                R: ::embedded_io_async::Read,
            {
                Ok((
                    $(<$name>::decode(&mut buffer).await?,)+
                ))
            }
        }
    };
}

impl_encode_decode_tuples! { A }
impl_encode_decode_tuples! { A B }
impl_encode_decode_tuples! { A B C}
impl_encode_decode_tuples! { A B C D }
impl_encode_decode_tuples! { A B C D E }
impl_encode_decode_tuples! { A B C D E F }
impl_encode_decode_tuples! { A B C D E F G }
impl_encode_decode_tuples! { A B C D E F G H }

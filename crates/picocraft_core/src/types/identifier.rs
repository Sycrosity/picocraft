use crate::prelude::*;

const NAMESPACE: &str = "minecraft";

impl<const N: usize> Identifier<N> {
    pub fn new<S: Into<String<N>>>(s: S) -> Self {
        Identifier(s.into())
    }
}

impl<const N: usize> Encode for Identifier<N> {
    async fn encode<W: embedded_io_async::Write>(
        &self,
        mut buffer: W,
    ) -> Result<(), EncodeError<W::Error>> {
        VarInt(NAMESPACE.len() as i32 + self.0.len() as i32)
            .encode(&mut buffer)
            .await?;
        buffer.write(NAMESPACE.as_bytes()).await?;
        buffer.write_u8(b':').await?;
        buffer.write(self.0.as_bytes()).await?;
        Ok(())
    }
}

impl<const N: usize> Decode for Identifier<N> {
    async fn decode<R: embedded_io_async::Read>(
        mut buffer: R,
    ) -> Result<Self, DecodeError<R::Error>> {
        let length = *VarInt::decode(&mut buffer).await?;

        if length < NAMESPACE.len() as i32 + 1 {
            return Err(DecodeError::VarIntTooSmall(VarInt(0)));
        }

        if length as usize > NAMESPACE.len() + 1 + N {
            return Err(DecodeError::VarIntTooBig);
        }

        // This warning is possibly a bug? See https://github.com/rust-lang/rust/issues/76200#issuecomment-3604535575
        // #[allow(const_evaluatable_unchecked)]
        let mut namespace_buf = [0u8; NAMESPACE.len() + 1];

        buffer.read_exact(&mut namespace_buf).await?;

        if &namespace_buf[..NAMESPACE.len()] != NAMESPACE.as_bytes()
            || namespace_buf[NAMESPACE.len()] != b':'
        {
            return Err(DecodeError::InvalidNamespace);
        }

        let mut path_buf = Vec::<u8, N>::new();

        let path_length = length as usize - NAMESPACE.len() - 1;

        //the error from this shouldn't be possible as we've already checked the
        // length.
        path_buf
            .resize_default(path_length)
            .expect("length already checked");

        buffer.read_exact(&mut path_buf).await?;

        Ok(Self(
            String::from_utf8(path_buf).map_err(DecodeError::InvalidUtf8)?,
        ))
    }
}

// trait Identifier: Encode + Decode {
//     const NAMESPACE: &'static str;

//     /// The longest block identifier path I could find was
//     /// 'cracked_polished_blackstone_bricks' (34), while the longest registry
//     /// Identifier path I could find was
// 'worldgen/flat_level_generator_preset'     /// (36). Giving some extra space
// incase I missed something, 48 should be     /// enough.
//     fn path(&self) -> &String<48>;

//     async fn encode_identifier<W: embedded_io_async::Write>(
//         &self,
//         mut buffer: W,
//     ) -> Result<(), EncodeError<W::Error>> {
//         VarInt(Self::NAMESPACE.len() as i32 + self.path().len() as i32)
//             .encode(&mut buffer)
//             .await?;
//         buffer.write(Self::NAMESPACE.as_bytes()).await?;
//         buffer.write(self.path().as_bytes()).await?;
//         Ok(())
//     }

//     async fn decode_identifier<R: embedded_io_async::Read>(
//         mut buffer: R,
//     ) -> Result<Self, DecodeError<R::Error>>
//     where
//         Self: Sized,
//     {

//         let string = String::<48>::decode(&mut buffer).await?;
//         Ok(Self::new(string))
//     }

// }

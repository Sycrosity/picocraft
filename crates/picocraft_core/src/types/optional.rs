use crate::prelude::*;

impl<T: Encode> Encode for Optional<T> {
    async fn encode<W: Write>(&self, mut buffer: W) -> Result<(), EncodeError<W::Error>> {
        match &self {
            Some(t) => t.encode(&mut buffer).await,
            None => Ok(()),
        }
    }
}

impl<T: Decode> Decode for Optional<T> {
    async fn decode<R: Read>(mut buffer: R) -> Result<Self, DecodeError<R::Error>> {
        match T::decode(&mut buffer).await {
            Ok(t) => Ok(Some(t)),
            Err(error) => match error {
                DecodeError::UnexpectedEof => Ok(None),
                e => Err(e),
            },
        }
    }
}

impl<T: Encode> Encode for PrefixedOptional<T> {
    async fn encode<W: Write>(&self, mut buffer: W) -> Result<(), EncodeError<W::Error>> {
        match &self.0 {
            Some(t) => {
                true.encode(&mut buffer).await?;
                t.encode(&mut buffer).await
            }
            None => false.encode(&mut buffer).await,
        }
    }
}

impl<T: Decode> Decode for PrefixedOptional<T> {
    async fn decode<R: Read>(mut buffer: R) -> Result<Self, DecodeError<R::Error>> {
        Ok(PrefixedOptional(match bool::decode(&mut buffer).await? {
            true => Some(T::decode(&mut buffer).await?),
            false => None,
        }))
    }
}

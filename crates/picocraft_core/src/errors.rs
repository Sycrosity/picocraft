use thiserror::Error;

#[derive(Debug, Error)]
pub enum PicocraftError {
    #[error("unknown error")]
    Unknown,
    #[error("client couldn't connect")]
    CouldntGetClient,
}

use crate::prelude::*;

#[derive(Debug)]
pub struct PacketSocket {
    // pub socket: tokio::io::BufWriter<tokio::net::TcpStream>,
    inner: tokio::net::TcpStream,
}

impl PacketSocket {
    pub fn new(socket: tokio::net::TcpStream) -> Self {
        Self { inner: socket }
    }

    pub async fn peek(&mut self, buf: &mut [u8]) -> Result<usize, SocketError> {
        self.inner
            .peek(buf)
            .await
            .inspect_err(|e| warn!("failed to peek at upcoming data: {e}"))
            .map_err(|_| SocketError::IoError)
    }

    pub async fn shutdown(&mut self) -> Result<(), SocketError> {
        use tokio::io::AsyncWriteExt;

        self.inner
            .shutdown()
            .await
            .inspect_err(|e| warn!("failed to send shutdown command: {e}"))
            .map_err(|_| SocketError::IoError)
    }

    pub async fn readable(&mut self) -> Result<(), SocketError> {
        self.inner
            .readable()
            .await
            .inspect_err(|e| warn!("socket not readable: {e}"))
            .map_err(|_| SocketError::NotReadable)
    }

    pub fn remote_endpoint(&self) -> Option<core::net::SocketAddr> {
        self.inner.peer_addr().ok()
    }
}

impl embedded_io_async::Read for PacketSocket {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        use tokio::io::AsyncReadExt;

        match self.inner.read(buf).await {
            Ok(n) => Ok(n),
            Err(_) => Err(SocketError::IoError),
        }
    }
}

// impl embedded_io_async::ReadReady for PacketSocket {
//     async fn read_ready(&mut self) -> Result<(), Self::Error> {

//     }
// }

impl embedded_io_async::Write for PacketSocket {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        use tokio::io::AsyncWriteExt;

        match self.inner.write(buf).await.inspect_err(|e| error!("{e:?}")) {
            Ok(n) => Ok(n),
            Err(_) => Err(SocketError::IoError),
        }
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        use tokio::io::AsyncWriteExt;

        self.inner
            .split()
            .1
            .flush()
            .await
            .map_err(|_| SocketError::IoError)
    }
}

impl embedded_io::ErrorType for PacketSocket {
    type Error = SocketError;
}

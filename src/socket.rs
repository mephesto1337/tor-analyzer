use std::pin::Pin;
use std::task::Poll;
use tokio::io::{AsyncRead, AsyncWrite, ReadBuf};
use tokio::net::{TcpStream, UnixStream};

pub enum Socket {
    Net(TcpStream),
    Unix(UnixStream),
}

impl std::convert::From<TcpStream> for Socket {
    fn from(s: TcpStream) -> Self {
        Self::Net(s)
    }
}

impl std::convert::From<UnixStream> for Socket {
    fn from(s: UnixStream) -> Self {
        Self::Unix(s)
    }
}

impl AsyncRead for Socket {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::result::Result<(), std::io::Error>> {
        match self.get_mut() {
            Self::Net(ref mut s) => AsyncRead::poll_read(Pin::new(s), cx, buf),
            Self::Unix(ref mut s) => AsyncRead::poll_read(Pin::new(s), cx, buf),
        }
    }
}

impl AsyncWrite for Socket {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        buf: &[u8],
    ) -> Poll<std::result::Result<usize, std::io::Error>> {
        match self.get_mut() {
            Self::Net(ref mut s) => AsyncWrite::poll_write(Pin::new(s), cx, buf),
            Self::Unix(ref mut s) => AsyncWrite::poll_write(Pin::new(s), cx, buf),
        }
    }

    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::result::Result<(), std::io::Error>> {
        match self.get_mut() {
            Self::Net(ref mut s) => AsyncWrite::poll_flush(Pin::new(s), cx),
            Self::Unix(ref mut s) => AsyncWrite::poll_flush(Pin::new(s), cx),
        }
    }

    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<std::result::Result<(), std::io::Error>> {
        match self.get_mut() {
            Self::Net(ref mut s) => AsyncWrite::poll_shutdown(Pin::new(s), cx),
            Self::Unix(ref mut s) => AsyncWrite::poll_shutdown(Pin::new(s), cx),
        }
    }
}

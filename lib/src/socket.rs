use std::io::{self, Read, Write};
use std::net::TcpStream;

#[cfg(any(
    doc,
    target_os = "android",
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "freebsd",
    target_os = "linux",
    target_os = "netbsd",
    target_os = "openbsd",
))]
use std::os::unix::net::UnixStream;

pub enum Socket {
    Net(TcpStream),
    #[cfg(any(
        doc,
        target_os = "android",
        target_os = "dragonfly",
        target_os = "emscripten",
        target_os = "freebsd",
        target_os = "linux",
        target_os = "netbsd",
        target_os = "openbsd",
    ))]
    Unix(UnixStream),
}

impl Socket {
    pub fn new<S: AsRef<str>>(s: S) -> io::Result<Self> {
        let s = s.as_ref();
        #[cfg(any(
            doc,
            target_os = "android",
            target_os = "dragonfly",
            target_os = "emscripten",
            target_os = "freebsd",
            target_os = "linux",
            target_os = "netbsd",
            target_os = "openbsd",
        ))]
        {
            let path = std::path::Path::new(s);
            if path.exists() {
                return Ok(Self::Unix(UnixStream::connect(s)?));
            }
        }
        Ok(Self::Net(TcpStream::connect(s)?))
    }
}

impl std::convert::From<TcpStream> for Socket {
    fn from(s: TcpStream) -> Self {
        Self::Net(s)
    }
}

#[cfg(any(
    doc,
    target_os = "android",
    target_os = "dragonfly",
    target_os = "emscripten",
    target_os = "freebsd",
    target_os = "linux",
    target_os = "netbsd",
    target_os = "openbsd",
))]
impl std::convert::From<UnixStream> for Socket {
    fn from(s: UnixStream) -> Self {
        Self::Unix(s)
    }
}

impl Read for Socket {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            Self::Net(ref mut tcp) => tcp.read(buf),
            #[cfg(any(
                doc,
                target_os = "android",
                target_os = "dragonfly",
                target_os = "emscripten",
                target_os = "freebsd",
                target_os = "linux",
                target_os = "netbsd",
                target_os = "openbsd",
            ))]
            Self::Unix(ref mut unix) => unix.read(buf),
        }
    }
}

impl Write for Socket {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            Self::Net(ref mut tcp) => tcp.write(buf),
            #[cfg(any(
                doc,
                target_os = "android",
                target_os = "dragonfly",
                target_os = "emscripten",
                target_os = "freebsd",
                target_os = "linux",
                target_os = "netbsd",
                target_os = "openbsd",
            ))]
            Self::Unix(ref mut unix) => unix.write(buf),
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match self {
            Self::Net(ref mut tcp) => tcp.flush(),
            #[cfg(any(
                doc,
                target_os = "android",
                target_os = "dragonfly",
                target_os = "emscripten",
                target_os = "freebsd",
                target_os = "linux",
                target_os = "netbsd",
                target_os = "openbsd",
            ))]
            Self::Unix(ref mut unix) => unix.flush(),
        }
    }
}

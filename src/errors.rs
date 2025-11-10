use std::fmt;

/// Error type untuk library WhatsApp
#[derive(Debug)]
pub enum ErrorKind {
    /// Kesalahan dalam format data
    InvalidFormat(String),
    /// Kesalahan koneksi
    ConnectionError(String),
    /// Kesalahan otentikasi
    AuthenticationError(String),
    /// Kesalahan enkripsi
    CryptoError(String),
    /// Kesalahan dalam payload
    InvalidPayload(String),
    /// Kesalahan protokol
    ProtocolError(String),
    /// Kesalahan I/O
    IOError(String),
    /// Kesalahan lainnya
    Other(String),
}

#[derive(Debug)]
pub struct Error {
    pub kind: ErrorKind,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.kind {
            ErrorKind::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            ErrorKind::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            ErrorKind::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
            ErrorKind::CryptoError(msg) => write!(f, "Crypto error: {}", msg),
            ErrorKind::InvalidPayload(msg) => write!(f, "Invalid payload: {}", msg),
            ErrorKind::ProtocolError(msg) => write!(f, "Protocol error: {}", msg),
            ErrorKind::IOError(msg) => write!(f, "IO error: {}", msg),
            ErrorKind::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

impl From<&str> for Error {
    fn from(s: &str) -> Self {
        Error { kind: ErrorKind::Other(s.to_string()) }
    }
}

impl From<String> for Error {
    fn from(s: String) -> Self {
        Error { kind: ErrorKind::Other(s) }
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error { kind: ErrorKind::IOError(e.to_string()) }
    }
}

impl From<base64::DecodeError> for Error {
    fn from(e: base64::DecodeError) -> Self {
        Error { kind: ErrorKind::InvalidFormat(e.to_string()) }
    }
}

#[macro_export]
macro_rules! bail {
    ($msg:expr) => {
        return Err(::std::convert::From::from($msg));
    };
    ($fmt:expr, $($arg:tt)*) => {
        return Err(::std::convert::From::from(format!($fmt, $($arg)*)));
    };
}

pub type Result<T> = std::result::Result<T, Error>;
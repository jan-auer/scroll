use core::fmt::{self, Display};
use core::result;

#[cfg(feature = "std")]
use std::io;
#[cfg(feature = "std")]
use std::error;

#[derive(Debug)]
/// A custom Scroll error
pub enum Error<T = usize> {
    /// The type you tried to read was too big
    TooBig { size: T, len: T },
    /// The requested offset to read/write at is invalid
    BadOffset(T),
    #[cfg(feature = "std")]
    /// A custom Scroll error for reporting messages to clients
    Custom(String),
    #[cfg(feature = "std")]
    /// Returned when IO based errors are encountered
    IO(io::Error),
}

#[cfg(feature = "std")]
impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::TooBig{ .. } => { "TooBig" }
            Error::BadOffset(_) => { "BadOffset" }
            Error::Custom(_) => { "Custom" }
            Error::IO(_) => { "IO" }
        }
    }
    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::TooBig{ .. } => { None }
            Error::BadOffset(_) => { None }
            Error::Custom(_) => { None }
            Error::IO(ref io) => { io.cause() }
        }
    }
}

#[cfg(feature = "std")]
impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::TooBig{ ref size, ref len } => { write! (fmt, "type is too big ({}) for {}", size, len) },
            Error::BadOffset(ref offset) => { write! (fmt, "bad offset {}", offset) },
            #[cfg(feature = "std")]
            Error::Custom(ref msg) => { write! (fmt, "{}", msg) },
            #[cfg(feature = "std")]
            Error::IO(ref err) => { write!(fmt, "{}", err) },
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

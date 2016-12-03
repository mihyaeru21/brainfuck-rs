use std::io;
use std::fmt;
use std::error;

#[derive(Debug)]
pub enum Error {
    Io(io::Error),
    Memory(&'static str),
    Jump(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref e) => write!(f, "IO error: {}", e),
            Error::Memory(ref s) => write!(f, "Memory error: {}", s),
            Error::Jump(ref s) => write!(f, "Jump error: {}", s),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref e) => e.description(),
            Error::Memory(ref s) => s,
            Error::Jump(ref s) => s,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref e) => Some(e),
            Error::Memory(_) => None,
            Error::Jump(_) => None,
        }
    }
}

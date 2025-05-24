

#[derive(Debug)]
pub enum Error {
    LockFailed,
    InvalidId,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Error::LockFailed => f.write_str("lock failed"),
            Error::InvalidId => f.write_str("invalid id"),
        }
    }
}

impl std::error::Error for Error {}



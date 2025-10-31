use thiserror::Error;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Could not parse the time: {0}")]
    Time(String),

    #[error("Invalid time: {0}:{0}")]
    InvalidTime(String, String),

    #[error("Current time is ambiguous")]
    AmbiguousTime,

    #[error("Could not find the local time")]
    NoLocalTime,

    #[error(transparent)]
    ParseInt(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Rusqlite(#[from] rusqlite::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    String(String),
}

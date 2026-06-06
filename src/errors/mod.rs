use std::{num::ParseIntError, string::FromUtf8Error};

use thiserror::Error;


#[derive(Debug, Error)]
pub enum ReplayParsingError {
    #[error("Invalid header (found: {found:?}, expected: {expected:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },

    #[error("Unknown header format, file is likely corrupted, not a shavit replay file, or extremly old")]
    UnknownVersion,

    #[error("Unable to covert from steamid3 (`{0}`) to steamid64")]
    InvalidSteamID(String),

    #[error("Invalid movetype value: {0}")]
    InvalidMoveType(i32),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    Utf8DecodeError(#[from] FromUtf8Error),

    #[error(transparent)]
    InvalidMinorVersion(#[from] ParseIntError),
}
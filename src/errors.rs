use std::io;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

use failure::Fail;

#[derive(Fail, Debug)]
pub enum RpcError {
    #[fail(display = "IO error: {}", _0)]
    Io(#[cause] io::Error),
    #[fail(display = "serde_json error: {}", _0)]
    Serde(#[cause] serde_json::Error),
    // #[fail(display = "Unexpected command type")]
    // UnexpectedCommandType,
    #[fail(display = "UTF-8 error: {}", _0)]
    Utf8(#[cause] FromUtf8Error),
    // #[fail(display = "{}", _0)]
    // StringError(String),
    #[fail(display = "reqwest error: {}", _0)]
    Reqwest(#[cause] reqwest::Error),
    #[fail(display = "postgres error: {}", _0)]
    Postgres(#[cause] postgres::Error),
    #[fail(display = "hostname contain fragment: {}", _0)]
    ParseFragment(String),
    #[fail(display = "hostname contain query: {}", _0)]
    ParseQuery(String),
    #[fail(display = "not parse scheme: {}", _0)]
    ParseBadScheme(String),
    #[fail(display = "hostname not contain scheme: {}", _0)]
    ParseMissingScheme(String),
    #[fail(display = "user info in hostname not supported {}", _0)]
    ParseBadUserInfo(String),
    #[fail(display = "{} hostname contain path {}", _0, _1)]
    ParseHavePath(String, String),
    #[fail(display = "not parse host: {}", _0)]
    ParseHost(String),
    #[fail(display = "not parse port: {}", _0)]
    ParsePort(String),
    #[fail(display = "not parse ipv6: {}", _0)]
    ParseIpv6(String),
    #[fail(display = "parse int error: {}", _0)]
    ParseInt(#[cause] ParseIntError),
    #[fail(display = "sled error: {}", _0)]
    Sled(#[cause] sled::Error),
}

impl From<io::Error> for RpcError {
    fn from(err: io::Error) -> RpcError {
        RpcError::Io(err)
    }
}

impl From<serde_json::Error> for RpcError {
    fn from(err: serde_json::Error) -> RpcError {
        RpcError::Serde(err)
    }
}

impl From<FromUtf8Error> for RpcError {
    fn from(err: FromUtf8Error) -> RpcError {
        RpcError::Utf8(err)
    }
}

impl From<reqwest::Error> for RpcError {
    fn from(err: reqwest::Error) -> RpcError {
        RpcError::Reqwest(err)
    }
}

impl From<postgres::Error> for RpcError {
    fn from(err: postgres::Error) -> RpcError {
        RpcError::Postgres(err)
    }
}

impl From<ParseIntError> for RpcError {
    fn from(err: ParseIntError) -> RpcError {
        RpcError::ParseInt(err)
    }
}

// pub type Result<T> = std::result::Result<T, RpcError>;

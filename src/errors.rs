#[derive(Debug, thiserror::Error)]
pub enum RssError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serde_json error: {0}")]
    Serde(#[from] serde_json::Error),
    // #[error("Unexpected command type")]
    // UnexpectedCommandType,
    #[error("UTF-8 error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    // #[error("{0}")]
    // StringError(String),
    #[error("DeadPool error: {0}")]
    DeadPool(#[from] deadpool_postgres::PoolError),
    #[error("Netc error: {0}")]
    Netc(#[from] netc::Error),
    #[error("postgres error: {0}")]
    TokioPostgres(#[from] tokio_postgres::Error),
    #[error("hostname contain fragment: {0}")]
    ParseFragment(String),
    #[error("hostname contain query: {0}")]
    ParseQuery(String),
    #[error("not parse scheme: {0}")]
    ParseBadScheme(String),
    #[error("hostname not contain scheme: {0}")]
    ParseMissingScheme(String),
    #[error("user info in hostname not supported {0}")]
    ParseBadUserInfo(String),
    #[error("{} hostname contain path {0}", _1)]
    ParseHavePath(String, String),
    #[error("not parse host: {0}")]
    ParseHost(String),
    #[error("not parse port: {0}")]
    ParsePort(String),
    #[error("not parse ipv6: {0}")]
    ParseIpv6(String),
    #[error("parse int error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("sled error: {0}")]
    Sled(#[from] sled::Error),
}

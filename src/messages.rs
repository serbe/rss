use crossbeam::channel::{Receiver, Sender};
use serde::{Deserialize, Serialize};

use crate::proxy::Proxy;

// pub type RStr = Receiver<String>;
// pub type RVecStr = Receiver<Vec<String>>;
// pub type RProxy = Receiver<Proxy>;

pub type SndSrvExt = Sender<PgExt>;
pub type RcvSrvExt = Receiver<PgExt>;

pub type SndWorkExt = Sender<WorkExt>;
pub type RcvWorkExt = Receiver<WorkExt>;

pub type SndPgExt = Sender<PgExt>;
pub type RcvPgExt = Receiver<PgExt>;

#[derive(Debug, Serialize, Deserialize)]
pub enum Request {
    Join,
    Get(i64),
    GetAnon(i64),
    Set(Vec<String>),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Join,
    Set,
    Err(String),
    Urls(Vec<String>),
}

pub enum WorkExt {
    Proxy(Proxy),
    Url(String),
}

pub enum PgExt {
    Urls(Vec<String>),
    Proxy(Proxy),
    Get(PgGetter),
}

pub struct PgGetter {
    pub limit: i64,
    pub anon: Option<bool>,
    pub work: bool,
    pub hours: Option<i64>,
}

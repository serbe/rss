use crossbeam::channel::{Receiver, Sender};

use crate::proxy::Proxy;

pub type RStr = Receiver<String>;
pub type RVecStr = Receiver<Vec<String>>;
pub type RProxy = Receiver<Proxy>;

pub type SndSrvExt = Sender<PgExt>;
pub type RcvSrvExt = Receiver<PgExt>;

pub type SndWorkExt = Sender<WorkExt>;
pub type RcvWorkExt = Receiver<WorkExt>;

pub type SndPgExt = Sender<PgExt>;
pub type RcvPgExt = Receiver<PgExt>;

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
    pub work: bool,
    pub anon: bool,
    pub num: i64,
}

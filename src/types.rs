use crossbeam::channel::{Receiver, Sender};

use crate::proxy::Proxy;

pub type RStr = Receiver<String>;
pub type RVecStr = Receiver<Vec<String>>;
pub type RProxy = Receiver<Proxy>;

pub type SStr = Sender<String>;
pub type SProxy = Sender<Proxy>;

pub type RcvWorkExt = Receiver<WorkExt>;
pub type SndWorkExt = Sender<WorkExt>;

pub type RcvPgExt = Receiver<PgExt>;
pub type SndPgExt = Sender<PgExt>;

pub enum WorkExt {
    Proxy(Proxy),
    Url(String),
}

pub enum PgExt {
    Urls(Vec<String>),
    Proxy(Proxy),
    Get(PgGetter)
}

pub struct PgGetter {
    pub work: bool, 
    pub anon: bool,
    pub num: i64,
}
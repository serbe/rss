use crossbeam::channel::{Receiver, Sender};

use crate::proxy::Proxy;

pub type RStr = Receiver<String>;
pub type RVecStr = Receiver<Vec<String>>;
pub type RProxy = Receiver<Proxy>;

pub type SStr = Sender<String>;
pub type SProxy = Sender<Proxy>;
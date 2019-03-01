use uuid::Uuid;
use crate::channel;

pub struct Id(Uuid);

impl Id {
    pub fn new() -> Self {
        Id(Uuid::new_v4())
    }
}

pub struct Data {
    raw: Box<[u8]>,
}

pub struct Msg {
    id: Id,
    chan: channel::Path,
    data: Data,
}

impl Msg {
    pub fn new(chan: channel::Path, data: Data) -> Self {
        let id = Id::new();
        Msg { id, chan, data }
    }
}
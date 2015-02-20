use super::persist::{Persister};
use super::Timespec;

pub type ItemId = usize;

pub struct Item {
    id: ItemId,
    desc: String,
    last_reviewed: Timespec,
    ff: f64,
    int_step: i32,
    iri: f64,
}

pub struct Conductor {
    persister: Persister
}

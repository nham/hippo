use super::{Item, ItemSchedData};
use time::{now_utc, Tm};

static INITIAL_FF: f64 = 2.5;

pub fn init_item() -> ItemSchedData {
    ItemSchedData {
        last_reviewed: now_utc().to_timespec(),
        ff: INITIAL_FF,
        int_step: 0,
        iri: 0.0,
    }
}


pub fn list_display_item(item: Item) -> String {
    format!("{:3} : {}", item.id, item.desc)
}

pub fn full_display_item(item: Item) -> String {
    format!("{:3} : {}\nLast reviewed: {}\nFF: {}\nint_step: {}\nIRI: {}", 
            item.id, item.desc, item.data.last_reviewed.sec, item.data.ff, 
            item.data.int_step, item.data.iri)
}

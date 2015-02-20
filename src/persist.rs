use rusqlite::SqliteConnection;
use super::{Timespec, Item, ItemId};

static SQLITE_DBFILE: &'static str = "hippo.sqlite";

pub trait Persister {
    fn add_item(&self, desc: &str) -> ItemId;
    fn change_item_desc(&self, id: ItemId, desc: &str) -> Result<(), ()>;
    fn update_item(&self, id: ItemId, last_reviewed: i32, ff: f64, int_step: i32, iri: f64) -> Result<(), ()>;
    fn remove_item(&self, id: ItemId) -> Result<(), ()>;
    fn get_items(&self) -> Vec<Item>;
}


struct SqlitePersister {
    conn: SqliteConnection
}

impl SqlitePersister {
    fn new() -> Self {
        SqlitePersister {
            conn: SqliteConnection::open(SQLITE_DBFILE).unwrap()
        }
    }
}

impl Persister for SqlitePersister {
    fn add_item(&self, desc: &str) -> ItemId {
        0
    }

    fn change_item_desc(&self, id: ItemId, desc: &str) -> Result<(), ()> {
        Err(())
    }

    fn update_item(&self, id: ItemId, last_reviewed: i32, ff: f64, int_step: i32, iri: f64) -> Result<(), ()> {
        Err(())
    }

    fn remove_item(&self, id: ItemId) -> Result<(), ()> {
        Err(())
    }

    fn get_items(&self) -> Vec<Item> {
        vec![]
    }
}

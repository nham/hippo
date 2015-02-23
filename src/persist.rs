use rusqlite::SqliteConnection;
use super::{Timespec, Item, ItemId, ItemSchedData};

static SQLITE_DBFILE: &'static str = "hippo.sqlite";

pub trait Persister {
    fn add_item(&self, desc: &str, data: ItemSchedData) -> ItemId;
    fn change_item_desc(&self, id: ItemId, desc: &str) -> Result<(), &str>;
    fn update_item(&self, id: ItemId, data: ItemSchedData) -> Result<(), ()>;
    fn get_item(&self, id: ItemId) -> Result<Item, ()>;
    fn remove_item(&self, id: ItemId) -> Result<(), ()>;
    fn get_items(&self) -> Vec<Item>;
}


pub struct SqlitePersister {
    conn: SqliteConnection
}

impl SqlitePersister {
    pub fn new() -> Self {
        SqlitePersister {
            conn: SqliteConnection::open(SQLITE_DBFILE).unwrap()
        }
    }
}

impl Persister for SqlitePersister {
    fn add_item(&self, desc: &str, data: ItemSchedData) -> ItemId {
        0
    }

    fn change_item_desc(&self, id: ItemId, desc: &str) -> Result<(), &str> {
        Err("unimplemented")
    }

    fn update_item(&self, id: ItemId, data: ItemSchedData) -> Result<(), ()> {
        Err(())
    }

    fn get_item(&self, id: ItemId) -> Result<Item, ()> {
        Err(())
    }

    fn remove_item(&self, id: ItemId) -> Result<(), ()> {
        Err(())
    }

    fn get_items(&self) -> Vec<Item> {
        vec![]
    }
}

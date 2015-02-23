use super::persist::{Persister, SqlitePersister};
use super::Timespec;
use core;

pub type ItemId = usize;

pub struct Item {
    pub id: ItemId,
    pub desc: String,
    pub data: ItemSchedData,
}

pub struct ItemSchedData {
    pub last_reviewed: Timespec,
    pub ff: f64,
    pub int_step: i32,
    pub iri: f64,
}

pub struct Conductor<P: Persister> {
    persister: P
}

impl Conductor<SqlitePersister> {
    pub fn new() -> Self {
        Conductor { persister: SqlitePersister::new() }
    }
}

impl <P: Persister> Conductor<P> {
    pub fn add_item(&self, desc: &str) {
        let init_data = core::init_item();
        let id = self.persister.add_item(desc, init_data);
        println!("Item {} has been added", id);
    }

    pub fn edit_item(&self, id: ItemId, desc: &str) {
        match self.persister.change_item_desc(id, desc) {
            Ok(_) => println!("Item {}'s description has been updated", id),
            Err(e) => println!("{}", e),
        }
    }

    pub fn view_item(&self, id: ItemId) {
        let item = self.persister.get_item(id).unwrap();
        println!("{}", core::full_display_item(item));
    }

    pub fn remove_item(&self, id: ItemId) {
        println!("remove");
    }

    pub fn list_items(&self) {
        println!("list");
    }

    pub fn review(&self, n: usize) {
        println!("review");
    }
}

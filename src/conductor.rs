use std::old_io::stdin;

use super::persist::{Persister, SqlitePersister};
use super::Timespec;
use core;

// doesn't need to be signed, but sqlite uses i64 for rowids and
// rusqlite only implements ToSql for i64 (not any other integer types)
pub type ItemId = i64;

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
        match self.persister.add_item(desc, init_data) {
            Ok(id)   => println!("Item {} has been added", id),
            Err(err) => println!("{}", err),
        }
    }

    pub fn edit_item(&self, id: ItemId, desc: &str) {
        match self.persister.change_item_desc(id, desc) {
            Ok(_)  => println!("Item {}'s description has been updated", id),
            Err(e) => println!("{}", e),
        }
    }

    pub fn view_item(&self, id: ItemId) {
        match self.persister.get_item(id) {
            Ok(item) => println!("{}", core::full_display_item(item)),
            Err(e)   => println!("{}", e),
        }
    }

    pub fn remove_item(&self, id: ItemId) {
        match self.persister.remove_item(id) {
            Ok(_)  => println!("Item {} has been removed", id),
            Err(e) => println!("{}", e),
        }
    }

    pub fn list_items(&self) {
        match self.persister.get_items() {
            Ok(items) => for item in items {
                println!("{}", core::list_display_item(item));
            },
            Err(e)    => println!("{}", e),
        }
    }

    pub fn review(&self, n: usize) {
        match self.persister.get_items() {
            Ok(items) => self.do_review(core::prepare_review_items(items), n),
            Err(e)    => println!("{}", e),
        }
    }

    fn do_review(&self, items: Vec<Item>, n: usize) {
        let mut reviewed = 0;
        let mut items_iter = items.iter();

        while reviewed < n {
            let item = match items_iter.next() {
                Some(item) => item,
                None => { println!("\nNo more items to review"); return },
            };

            let mut inp: char;

            // loop for getting user input
            loop {
                print!("{}", core::review_display_item(reviewed+1, item));
                let input = stdin().read_line()
                            .ok().expect("Failed to read input");
                let input = input.as_slice().trim();

                if input.len() == 1 && "qs012345".find(input.char_at(0)).is_some() {
                    inp = input.char_at(0);
                    break;
                } else {
                    println!("\nInvalid input");
                }
            }

            match inp {
                'q' => break,
                's' => continue,
                n   => {
                    let fam = ((n as isize) - ('0' as isize)) as u8;
                    let new_item_data = core::assess_item(&item.data, fam);
                    match self.persister.update_item(item.id, new_item_data) {
                        Err(e) => {println!("{}", e); return },
                        _ => { reviewed += 1; },
                    }
                },
            }

        }

        println!("\nFinished reviewing.");
    }
}

use rusqlite::SqliteConnection;
use super::{Timespec, Item, ItemId, ItemSchedData};

static SQLITE_DBFILE: &'static str = "hippo.sqlite";

pub trait Persister {
    fn add_item(&self, desc: &str, data: ItemSchedData) -> Result<ItemId, String>;
    fn change_item_desc(&self, id: ItemId, desc: &str) -> Result<(), String>;
    fn update_item(&self, id: ItemId, data: ItemSchedData) -> Result<(), String>;
    fn get_item(&self, id: ItemId) -> Result<Item, String>;
    fn remove_item(&self, id: ItemId) -> Result<(), String>;
    fn get_items(&self) -> Result<Vec<Item>, String>;
}


pub struct SqlitePersister {
    conn: SqliteConnection
}

impl SqlitePersister {
    pub fn new() -> Self {
        let conn = SqliteConnection::open(SQLITE_DBFILE).unwrap();
        conn.execute("CREATE TABLE if not exists items (
                      id integer primary key autoincrement,
                      desc text UNIQUE,
                      last_reviewed int,
                      ff real,
                      int_step int,
                      iri real)", &[]).unwrap();

        SqlitePersister { conn: conn }
    }
}

impl Persister for SqlitePersister {
    fn add_item(&self, desc: &str, data: ItemSchedData) -> Result<ItemId, String> {
        let sql = "INSERT INTO items (desc, last_reviewed, ff, int_step, iri)
                   VALUES ($1, $2, $3, $4, $5)";

        let res = self.conn.execute(sql, &[&desc, &data.last_reviewed,
                                           &data.ff, &data.int_step, &data.iri]);

        match res {
            Ok(_) => Ok(self.conn.last_insert_rowid()),
            Err(err) => Err(err.message),
        }
    }

    fn change_item_desc(&self, id: ItemId, desc: &str) -> Result<(), String> {
        let res = self.conn.execute("UPDATE items SET desc=$1 WHERE id=$2",
                                    &[&desc, &id]);
        match res {
            Ok(num) =>
                if num > 0 {
                    Ok(())
                } else {
                    Err("Item not found".to_string())
                },
            Err(err) => Err(err.message)
        }
    }

    fn update_item(&self, id: ItemId, data: ItemSchedData) -> Result<(), String> {
        let sql = "UPDATE items SET last_reviewed=$1, ff=$2, iri=$3,
                   int_step=$4 WHERE id=$5";
        let res = self.conn.execute(sql, &[&data.last_reviewed, &data.ff,
                                           &data.iri, &data.int_step, &id]);
        match res {
            Ok(num) =>
                if num > 0 {
                    Ok(())
                } else {
                    Err("Item not found".to_string())
                },
            Err(err) => Err(err.message)
        }
    }

    fn get_item(&self, id: ItemId) -> Result<Item, String> {
        let sql = "SELECT id, desc, last_reviewed, ff, int_step, iri
                   FROM items WHERE id=$1";
        let mut stmt = match self.conn.prepare(sql) {
            Ok(s) => s,
            Err(err) => return Err(err.message),
        };

        let mut rows = match stmt.query(&[&id]) {
            Ok(s) => s,
            Err(err) => return Err(err.message),
        };

        let row = match rows.next() {
            Some(r) => match r {
                Ok(row) => row,
                Err(err) => return Err(err.message),
            },
            None => return Err("Item not found".to_string()),
        };

        Ok(Item {
            id: row.get(0),
            desc: row.get(1),
            data: ItemSchedData {
                last_reviewed: row.get(2),
                ff: row.get(3),
                int_step: row.get(4),
                iri: row.get(5),
            },
        })
    }

    fn remove_item(&self, id: ItemId) -> Result<(), String> {
        let res = self.conn.execute("DELETE FROM items WHERE id=$1", &[&id]);
        match res {
            Ok(_) => Ok(()),
            Err(err) => Err(err.message)
        }
    }

    fn get_items(&self) -> Result<Vec<Item>, String> {
        let sql = "SELECT id, desc, last_reviewed, ff, int_step, iri FROM items";
        let mut stmt = match self.conn.prepare(sql) {
            Ok(s) => s,
            Err(err) => return Err(err.message),
        };

        let mut rows = match stmt.query(&[]) {
            Ok(s) => s,
            Err(err) => return Err(err.message),
        };

        let mut v = vec![];
        for row in rows {
            match row {
                Ok(row) => v.push(
                    Item {
                        id: row.get(0),
                        desc: row.get(1),
                        data: ItemSchedData {
                            last_reviewed: row.get(2),
                            ff: row.get(3),
                            int_step: row.get(4),
                            iri: row.get(5),
                        },
                    }),
                Err(err) => return Err(err.message),
            }
        }

        Ok(v)
    }
}

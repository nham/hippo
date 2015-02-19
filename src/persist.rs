trait Persister {
    fn add_item(&self, desc: &str) -> i32;
    fn change_item_desc(&self, id: i32, desc: &str) -> Result<(), ()>;
    fn update_item(&self, id: i32, last_reviewed: i32, ff: f64, int_step: i32, iri: f64) -> Result<(), ()>;
    fn remove_item(&self, id: i32) -> Result<(), ()>;
    fn get_items(&self) -> Vec<Item>;
}

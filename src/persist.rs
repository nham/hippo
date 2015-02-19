trait Persister {
    fn add_item(&self, &Item) -> i32;
    fn change_item_desc(&self, i32, &str);
    fn update_item(&self, &Item);
    fn remove_item(&self, i32);
    fn get_items(&self) -> Vec<Item>;
}

trait Crypto {}

trait Storage {}

trait EntryManager {
    fn add_entry(&self, entry: &str) -> Result<(), String>;
    fn get_entry(&self, id: usize) -> Result<String, String>;
    fn delete_entry(&self, id: usize) -> Result<(), String>;
    fn update_entry(&self, id: usize, entry: &str) -> Result<(), String>;
}

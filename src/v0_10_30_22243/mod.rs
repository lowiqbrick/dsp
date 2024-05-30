// file for defining all items and recipes
mod essentials;

pub mod items {
    use super::essentials::hello_essentials;

    /// creates a big HashMap with all the items and buildings in the game
    pub fn get_items() {
        println!("placeholder");
    }
    /// testing only
    pub fn hello_mod() {
        println!("hello from mod");
    }
    /// testing only
    pub fn hello_combined() {
        hello_mod();
        hello_essentials();
    }
}
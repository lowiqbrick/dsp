// file for defining all items and recipes
// import everything from essentials.rs
mod essentials;
use crate::v0_10_30_22243::essentials::hello_essentials;

pub mod items {
    use super::{essentials::item_logic::{IsItem, Item, ItemAmount, ManFac, Recipe}, hello_essentials};


    /// testing only
    pub fn hello_mod() {
        println!("hello from mod");
    }
    /// testing only
    pub fn hello_combined() {
        hello_mod();
        hello_essentials();
    }
    /// creates a big HashMap with all the items and buildings in the game
    pub fn get_items() {
        // declare Vector for containing all items an recipes
        let mut items: Vec<Item<_>> = Vec::new();
        // item that are the origin of crafting chains
        let iron_ore: Item<_>;
        let iron_ore_rec: Recipe<_> = 
            Recipe::new(
                0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(1, &iron_ore))]);
        iron_ore = Item::new(
            String::from("Iron Ore"),
            ManFac::Origin,
            vec![iron_ore_rec])
    }
}
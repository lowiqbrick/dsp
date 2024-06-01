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
    pub fn get_items<'a>(mut result_vec: Vec<Item>) -> Vec<Item> {
        // declare Vector for containing all items an recipes
        // item for crafting recipes that is a placeholder for the calling item itself
        let item_self: Item;
        let item_self_rec: Recipe = 
            Recipe::new(
                0,
                vec![IsItem::new_nai()],
                vec![IsItem::new_nai()]);
        item_self = Item::new(
            "Item Self", 
            ManFac::Origin, 
            vec![item_self_rec]);
        result_vec.push(item_self);
        // item that are the origin of crafting chains
        let iron_ore: Item;
        let iron_ore_rec: Recipe = 
            Recipe::new(
                0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(1, &result_vec[0].clone()))]);
        iron_ore = Item::new(
            "Iron Ore",
            ManFac::Origin,
            vec![iron_ore_rec]);
        result_vec.push(iron_ore);
        // return the created vector with all the items
        result_vec
    }
}
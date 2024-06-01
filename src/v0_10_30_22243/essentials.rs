// file for all structs functions etc.
// imports
// HashMaps
#[allow(dead_code)]
use std::collections::HashMap;

pub mod item_logic {
    /// enum for manufacturing facilities
    #[derive(Debug, Clone, Copy)]
    pub enum ManFac {
        Origin,
        Assembler,
        Furnace,
        Lab,
        OilRefinery,
        ChemicalPlant,
        MiniatureParticleCollider,
    }

    /// struct for combining an item and an amount of said item
    #[derive(Debug, Clone, Copy)]
    pub struct ItemAmount<'a> {
        amount: u8,
        item: &'a Item<'a>,
    }

    impl<'a> ItemAmount<'a> {
        pub fn new(amount: u8, item: &'a Item) -> ItemAmount<'a> {
            ItemAmount{
                amount,
                item,
            }
        }
    }

    /// enum for differentiation between items and not an item (nai)
    /// which is relevant for crafting recepies for items which cannot be
    /// crafted and only be obtained/mined (ores, critical photons, etc.)
    #[derive(Debug, Clone, Copy)]
    pub enum IsItem<'a> {
        Item(ItemAmount<'a>),
        // NotAnItem
        NAI,
    }

    impl <'a> IsItem <'a> {
        pub fn new(item_amount: ItemAmount<'a>) -> IsItem {
            IsItem::Item(item_amount)
        }
        
        pub fn new_nai() -> IsItem<'a>{
            IsItem::NAI
        }
    }

    /// struct for recipes
    #[derive(Debug, Clone)]
    pub struct Recipe<'a> {
        // crafting time (in seconds)
        crafting_time: u8,
        ingredients: Vec<IsItem<'a>>,
        products: Vec<IsItem<'a>>,
    }

    impl<'a> Recipe<'a> {
        pub fn new(crafting_time: u8, ingredients: Vec<IsItem<'a>>, products: Vec<IsItem<'a>>) -> Recipe<'a> {
            Recipe {
                crafting_time,
                ingredients,
                products,
            }
        }
    }

    /// struct for representing an item
    #[derive(Debug, Clone)]
    pub struct Item <'a> {
        name: &'a str,
        creation_facility: ManFac,
        recipes: Vec<Recipe<'a>>,
    }

    impl<'a> Item<'a> {
        pub fn new(name: &'a str, creation_facility: ManFac, recipes: Vec<Recipe<'a>>) -> Item<'a> {
            Item {
                name,
                creation_facility,
                recipes,
            }
        }
    // /// the central method on which everything in the program is build
    //pub fn generate_crafting_cain(&self, prev_path: HashMap, item_rate: f32) -> HashMap {
    //    let _placeholder = prev_path;
    //    let _placeholder2 = item_rate;
    //    prev_path
    //}
    }
}

/// testing only
pub fn hello_essentials () {
    println!("hello from essentials");
}
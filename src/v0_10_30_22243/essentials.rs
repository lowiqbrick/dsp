// file for all structs functions etc.
// imports
// HashMaps
#[allow(dead_code)]
use std::collections::HashMap;

pub mod item_logic {
    /// enum for manufacturing facilities
    #[derive(Debug)]
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
    #[derive(Debug)]
    pub struct ItemAmount<'a, T> {
        amount: u8,
        item: &'a T,
    }

    impl<'a, T> ItemAmount<'a, T> {
        pub fn new(amount: u8, item: &'a T) -> ItemAmount<T> {
            ItemAmount{
                amount,
                item,
            }
        }
    }

    /// enum for differentiation between items and not an item (nai)
    /// which is relevant for crafting recepies for items which cannot be
    /// crafted and only be obtained/mined (ores, critical photons, etc.)
    #[derive(Debug)]
    pub enum IsItem<'a, T> {
        Item(ItemAmount<'a, T>),
        // NotAnItem
        NAI,
    }

    impl <'a, T> IsItem <'a, T> {
        pub fn new(item_amount: ItemAmount<'a, T>) -> IsItem<T> {
            IsItem::Item(item_amount)
        }
        
        pub fn new_nai() -> IsItem<'a, T>{
            IsItem::NAI
        }
    }

    /// struct for recipes
    #[derive(Debug)]
    pub struct Recipe<'a, T> {
        // crafting time (in seconds)
        crafting_time: u8,
        ingredients: Vec<IsItem<'a, T>>,
        products: Vec<IsItem<'a, T>>,
    }

    impl<'a, T> Recipe<'a, T> {
        pub fn new(crafting_time: u8, ingredients: Vec<IsItem<'a, T>>, products: Vec<IsItem<'a, T>>) -> Recipe<'a, T> {
            Recipe {
                crafting_time,
                ingredients,
                products,
            }
        }
    }

    /// struct for representing an item
    #[derive(Debug)]
    pub struct Item <'a, T> {
        name: String,
        creation_facility: ManFac,
        recipes: Vec<Recipe<'a, T>>,
    }

    impl<'a, T> Item<'a, T> {
        pub fn new(name: String, creation_facility: ManFac, recipes: Vec<Recipe<'a, T>>) -> Item<T> {
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
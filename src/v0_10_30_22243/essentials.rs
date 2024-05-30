// file for all structs functions etc.
// imports
// HashMaps
#[allow(dead_code)]
use std::collections::HashMap;

pub mod item_logic {
    /// enum for manufacturing facilities
    #[derive(Debug)]
    pub enum ManFac {
        Assembler,
        Furnace,
        Lab,
        OilRefinery,
        ChemicalPlant,
        MiniatureParticleCollider,
    }

    /// struct for combining an item and an amount of said item
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
    /// crafted and only be obtained (ores, critical photons, etc.)
    #[derive(Debug)]
    pub enum IsItem<T> {
        Item(T),
        // NotAnItem
        NAI,
    }

    /// struct for recipes
    #[derive(Debug)]
    pub struct Recipe<T> {
        // crafting time (in seconds)
        crafting_time: u8,
        ingredients: Vec<IsItem<T>>,
        products: Vec<IsItem<T>>,
    }

    impl<T> Recipe<T> {
        pub fn new(crafting_time: u8, ingredients: Vec<IsItem<T>>, products: Vec<IsItem<T>>) -> Recipe<T> {
            Recipe {
                crafting_time,
                ingredients,
                products,
            }
        }
    }

    /// struct for representing an item
    #[derive(Debug)]
    pub struct Item <T> {
        name: String,
        creation_facility: ManFac,
        recipes: Vec<Recipe<T>>,
    }

    impl<T> Item<T> {
        pub fn new(name: String, creation_facility: ManFac, recipes: Vec<Recipe<T>>) -> Item<T> {
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
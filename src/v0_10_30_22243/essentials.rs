// file for all structs functions etc.
// imports
// HashMaps
#[allow(dead_code)]

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
    #[derive(Debug, Clone)]
    pub struct ItemAmount {
        amount: u8,
        item: String,
    }

    impl ItemAmount {
        pub fn new(amount: u8, item: String) -> ItemAmount {
            ItemAmount{
                amount,
                item,
            }
        }
    }

    /// enum for differentiation between items and not an item (nai)
    /// which is relevant for crafting recepies for items which cannot be
    /// crafted and only be obtained/mined (ores, critical photons, etc.)
    #[derive(Debug, Clone)]
    pub enum IsItem {
        Item(ItemAmount),
        // NotAnItem
        NAI,
    }

    impl IsItem {
        pub fn new(item_amount: ItemAmount) -> IsItem {
            IsItem::Item(item_amount)
        }
        
        pub fn new_nai() -> IsItem{
            IsItem::NAI
        }
    }

    /// struct for recipes
    #[derive(Debug, Clone)]
    pub struct Recipe {
        // crafting time (in seconds)
        crafting_time: f32,
        ingredients: Vec<IsItem>,
        products: Vec<IsItem>,
    }

    impl Recipe {
        pub fn new(crafting_time: f32, ingredients: Vec<IsItem>, products: Vec<IsItem>) -> Recipe {
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
        creation_facility: Vec<ManFac>,
        recipes: Vec<Recipe>,
    }

    impl<'a> Item<'a> {
        pub fn new(name: &'a str, creation_facility: Vec<ManFac>, recipes: Vec<Recipe>) -> Item {
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

// a mountain of macros for the names of all items
// macro_rules!  {""}
//#[macro_export]
//macro_rules! iron_ore {"Iron Ore";}
//#[macro_export]
//macro_rules! copper_ore {"Copper Ore"}
//macro_rules! Stone {"Stone"}
//macro_rules! coal {"Coal"}
//macro_rules! silicon_ore {"Silicon Ore"}
//macro_rules! titanium_ore {"Titanium Ore"}
//macro_rules! water {"Water"}
//macro_rules! crude_oil {"Crude Oil"}
//macro_rules! hydrogen {"Hydrogen"}
//macro_rules! deuterium {"Deuterium"}
//macro_rules! anitmatter {"Anitmatter"}
//macro_rules! core_element {"Core Element"}
//macro_rules! critical_photon {"Critical Photon"}
//macro_rules! kimberlite_ore {"Kimberlite Ore"}
//macro_rules! iron_ingot {"Iron Ingot"}
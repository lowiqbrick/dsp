// file for all structs functions etc.
// imports
// HashMaps
#[allow(dead_code)]

pub mod item_logic {
    use std::collections::HashMap;
    use std::fmt::{self, Display};

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
        pub amount: u8,
        pub item: String,
    }

    impl ItemAmount {
        pub fn new(amount: u8, item: String) -> ItemAmount {
            ItemAmount { amount, item }
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

        pub fn new_nai() -> IsItem {
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
    /// Struct for storing the result of a part of the crafting chain
    #[derive(Debug, Clone)]
    struct ItemResult {
        pub describer: String,
        pub name: String,
        pub num_station: f32,
        pub station: ManFac,
        pub requirements: Vec<ItemAmount>,
    }

    impl Display for ItemResult {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // write all things with static values
            write!(f,"{}\n", self.describer)?;
            write!(f, "target rate: {}\n", self.num_station)?;
            write!(f, "requires: {} {:?}", self.num_station.round(), self.station)?;
            // write vector
            for amount in self.requirements.clone() {
                write!(f, "    {} {}", amount.amount, amount.item)?;
            }
            // hand over the results
            write!(f, "")
        }
    }
    /// struct for representing an item
    #[derive(Debug, Clone)]
    pub struct Item<'a> {
        pub name: &'a str,
        pub creation_facility: Vec<ManFac>,
        pub recipes: Vec<Recipe>,
    }

    impl<'a> Item<'a> {
        pub fn new(name: &'a str, creation_facility: Vec<ManFac>, recipes: Vec<Recipe>) -> Item {
            Item {
                name,
                creation_facility,
                recipes,
            }
        }
        /// the central method on which everything in the program is build
        pub fn crafting_chain(
            item_name: String,
            item_per_sec: f32,
            settings: ProgamInfo,
            result_order: &Vec<String>,
            result: HashMap<String, ItemAmount>,
            prev_path: String,
            is_proliferated: bool,
        ) -> HashMap<String, ItemAmount> {
            // create result variable
            let mut result_var: ItemResult;
            
            result
        }
    }

    // enums and structs for the execution of the main function
    /// enum for saving the type of lab used
    #[derive(Debug)]
    pub enum ChemLabMK {
        // Chemical Plant
        Lab,
        // Quantum Chemical Plant
        QuantumLab,
    }

    /// enum for saving the type of assembler used
    #[derive(Debug)]
    pub enum SmelterMK {
        ArcSmelter,
        PlaneSmelter,
        NegentropySmelter,
    }

    /// enum for saving the type of assembler used
    #[derive(Debug)]
    pub enum AssemblerMK {
        MKone,
        MKtwo,
        MKthree,
        MKfour,
    }

    /// enum for saving the type of assembler used
    #[derive(Debug)]
    pub enum LabMK {
        MatrixLab,
        SelfEvolutionLab,
    }

    /// enum for saving the type of assembler used
    #[derive(Debug)]
    pub enum Proliferator {
        MKone,
        MKtwo,
        MKthree,
        None,
    }

    /// struct for containing all information for the main function
    #[derive(Debug)]
    pub struct ProgamInfo {
        // enums to save the facilities used
        pub proliferators: Proliferator,
        pub chemlab: ChemLabMK,
        pub smelter: SmelterMK,
        pub assembler: AssemblerMK,
        pub lab: LabMK,
        pub no_proliferation: Vec<String>,
        pub additional_items: Vec<ItemAmount>,
        pub item_recipe: Vec<ItemAmount>,
        pub merge: bool,
        pub assume_basics: bool,
        pub produced_item: ItemAmount,
    }
    impl ProgamInfo {
        pub fn new(
            proliferators: Proliferator,
            chemlab: ChemLabMK,
            smelter: SmelterMK,
            assembler: AssemblerMK,
            lab: LabMK,
            no_proliferation: Vec<String>,
            additional_items: Vec<ItemAmount>,
            item_recipe: Vec<ItemAmount>,
            merge: bool,
            assume_basics: bool,
            produced_item: ItemAmount,
        ) -> ProgamInfo {
            ProgamInfo {
                proliferators,
                chemlab,
                smelter,
                assembler,
                lab,
                no_proliferation,
                additional_items,
                item_recipe,
                merge,
                assume_basics,
                produced_item,
            }
        }
    }

    /// enum for containing the current state of the processing
    /// of the arguments
    #[derive(PartialEq)]
    pub enum ArgState {
        // the default; only an item name and an amount
        Default,
        // items not to be proliferated
        NoProliferation,
        // additional items
        AdditionalItems,
        // specify proliferation
        ProlifLevel,
        // specify chemical labs
        ChemLabLevel,
        // specify furnace
        FurnaceLevel,
        // specify assembler
        AssemblerLevel,
        // specify laboratory
        LabLevel,
        // specify the recipe use for an item
        ItemRecipe,
    }

    /// function for determining whether a given String exists as a key
    /// in a hashmap
    fn check_for_key(key_str: String, map: HashMap<String, ItemResult>) -> bool {
        // compare all keys to given String
        // return true if a match is found
        for hashmap_item in map.keys() {
            if &key_str == hashmap_item {
                return true;
            }
        }
        // otherwise false
        false
    }

}

// macro for creating items in a more convienent manner
#[macro_export]
macro_rules! recipe {
    ($crafting_time: expr, ($($ingredients: tt)*), ($($products: tt)*)) => {
        Recipe::new($crafting_time, vec![$($ingredients)*], vec![$($products)*])
    };
}
#[macro_export]
macro_rules! recitem {
    ($amount: literal, $itemname: literal) => {
        IsItem::new(ItemAmount::new($amount, String::from($itemname)))
    };
}
#[macro_export]
macro_rules! item {
    ($name: literal, ($($item_vec: tt)*), ($($rec_vec: tt)*)) => {
        Item::new($name, vec![$($item_vec)*], vec![$($rec_vec)*])
    };
}
#[macro_export]
macro_rules! tohash {
    ($hashmap: ident, $item_name: ident, $item_str: literal, $item: expr) => {
        let $item_name: Item = $item;
        $hashmap.insert(String::from($item_str), $item_name);
    };
}

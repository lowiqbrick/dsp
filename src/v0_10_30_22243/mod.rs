// file for defining all items and recipes
// import everything from essentials.rs
mod essentials;
use crate::v0_10_30_22243::essentials::hello_essentials;

pub mod items {
    use std::collections::HashMap;

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
    pub fn get_items<'a>(mut res_hash: HashMap<String, Item>) -> HashMap<String, Item> {
        // all ores/origin items
        let iron_ore: Item;
        let iron_ore_rec: Recipe =
            Recipe::new(
                0, 
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(1, String::from("Iron Ore")))],
            );
        iron_ore =
            Item::new(
                "Iron Ore",
                vec![ManFac::Origin],
                vec![iron_ore_rec]);
        res_hash.insert(String::from("Iron Ore"), iron_ore);
        let copper_ore: Item =
            Item::new(
                "Copper Ore",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Copper Ore")))])]);
        res_hash.insert(String::from("Copper Ore"), copper_ore);
        let stone: Item =
            Item::new(
                "Stone", 
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Stone")))])]);
        res_hash.insert(String::from("Stone"), stone);
        let coal: Item =
            Item::new(
                "Coal",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Coal")))])]);
        res_hash.insert(String::from("Coal"), coal);
        let silicon_ore: Item =
            Item::new(
                "Silicon Ore",
                // also minable
                vec![ManFac::Furnace],
                vec![Recipe::new(
                    10,
                    vec![IsItem::new(ItemAmount::new(10, String::from("Stone")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Silicon Ore")))])]);
        res_hash.insert(String::from("Silicon Ore"), silicon_ore);
        let titanium_ore: Item =
            Item::new(
                "Titanium Ore",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Titanium Ore")))])]);
        res_hash.insert(String::from("Titanium Ore"), titanium_ore);
        let water: Item =
            Item::new(
                "Water",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Water")))])]);
        res_hash.insert(String::from("Water"), water);
        let crude_oil: Item =
            Item::new(
                "Crude Oil",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Crude Oil")))])]);
        res_hash.insert(String::from("Crude Oil"), crude_oil);
        // hydrogen requires a lot of other items to be declared
        let refined_oil: Item;
        let graphene: Item;
        let fire_ice: Item;
        let energetic_graphite: Item;
        let antimatter: Item;
        let critical_photon: Item;
        let hydrogen: Item =
            Item::new(
                "Hydrogen",
                vec![ManFac::ChemicalPlant, ManFac::OilRefinery],
                vec![Recipe::new(
                    4,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Crude Oil")))],
                    vec![IsItem::new(ItemAmount::new(2, String::from("Refined Oil"))),
                        IsItem::new(ItemAmount::new(1, String::from("Hydrogen")))]),
                    Recipe::new(
                    2,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Fire Ice")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Hydrogen"))),
                        IsItem::new(ItemAmount::new(2, String::from("Graphene")))]),
                    Recipe::new(
                    4,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Hydrogen"))),
                        IsItem::new(ItemAmount::new(1, String::from("Refined Oil")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Energetic Graphite"))),
                        IsItem::new(ItemAmount::new(3, String::from("Hydrogen")))]),
                    Recipe::new(
                    2,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Critical Photon")))],
                    vec![IsItem::new(ItemAmount::new(2, String::from("Hydrogen"))),
                        IsItem::new(ItemAmount::new(2, String::from("Anitmatter")))])]);
        // processed items
        let iron_ingot =
            Item::new(
                "Iron Ingot",
                vec![ManFac::Furnace],
                vec![Recipe::new(
                    1,
                    vec![IsItem::new(ItemAmount::new(1, String::from("Iron Ore")))], 
                    vec![IsItem::new(ItemAmount::new(1, String::from("Iron Ore")))])]);
        res_hash.insert(String::from("Iron Ingot"), iron_ingot);
        // return the hashmap with all the items
        res_hash
    }

    /// function for outputting all items
    pub fn print_items() {
        let mut map:HashMap<String, Item> = HashMap::new();
        map = get_items(map);
        println!("{:?}", map);
    }
}
// file for defining all items and recipes
// import everything from essentials.rs
mod essentials;
use crate::v0_10_30_22243::essentials::hello_essentials;

pub mod items {
    use std::{collections::HashMap, vec};

    use crate::{item, recipe, recitem, tohash};

    use super::{essentials::item_logic::{IsItem, Item, ItemAmount, ManFac, ManFac::{Origin, Assembler, Furnace, Lab, OilRefinery, ChemicalPlant, MiniatureParticleCollider}, Recipe}, hello_essentials};


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
                0.0, 
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
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Copper Ore")))])]);
        res_hash.insert(String::from("Copper Ore"), copper_ore);
        let stone: Item =
            Item::new(
                "Stone", 
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Stone")))])]);
        res_hash.insert(String::from("Stone"), stone);
        let coal: Item =
            Item::new(
                "Coal",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Coal")))])]);
        res_hash.insert(String::from("Coal"), coal);
        let silicon_ore: Item =
            Item::new(
                "Silicon Ore",
                // also minable
                vec![ManFac::Furnace],
                vec![Recipe::new(
                    10.0,
                    vec![IsItem::new(ItemAmount::new(10, String::from("Stone")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Silicon Ore")))])]);
        res_hash.insert(String::from("Silicon Ore"), silicon_ore);
        let titanium_ore: Item =
            Item::new(
                "Titanium Ore",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Titanium Ore")))])]);
        res_hash.insert(String::from("Titanium Ore"), titanium_ore);
        let water: Item =
            Item::new(
                "Water",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Water")))])]);
        res_hash.insert(String::from("Water"), water);
        let crude_oil: Item =
            Item::new(
                "Crude Oil",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0.0,
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
        let gear: Item;
        let hydrogen: Item =
            Item::new(
                "Hydrogen",
                vec![ManFac::ChemicalPlant, ManFac::OilRefinery],
                vec![Recipe::new(
                    4.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Crude Oil")))],
                    vec![IsItem::new(ItemAmount::new(2, String::from("Refined Oil"))),
                        IsItem::new(ItemAmount::new(1, String::from("Hydrogen")))]),
                    Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Fire Ice")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Hydrogen"))),
                        IsItem::new(ItemAmount::new(2, String::from("Graphene")))]),
                    Recipe::new(
                    4.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Hydrogen"))),
                        IsItem::new(ItemAmount::new(1, String::from("Refined Oil")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Energetic Graphite"))),
                        IsItem::new(ItemAmount::new(3, String::from("Hydrogen")))]),
                    Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Critical Photon")))],
                    vec![IsItem::new(ItemAmount::new(2, String::from("Hydrogen"))),
                        IsItem::new(ItemAmount::new(2, String::from("Anitmatter")))])]);
        res_hash.insert(String::from("Hydrogen"), hydrogen);
        let deuterium: Item =
            Item::new(
                "Deuterium",
                vec![ManFac::Origin, ManFac::MiniatureParticleCollider], 
                vec![Recipe::new(
                    2.5,
                    vec![IsItem::new(ItemAmount::new(10, String::from("Hydrogen")))],
                    vec![IsItem::new(ItemAmount::new(5, String::from("Deuterium")))])]);
        res_hash.insert(String::from("Deuterium"), deuterium);
        antimatter =
            Item::new(
                "Antimatter",
                vec![ManFac::MiniatureParticleCollider],
                vec![Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Critical Photon")))],
                    vec![IsItem::new(ItemAmount::new(2, String::from("Hydrogen"))),
                        IsItem::new(ItemAmount::new(2, String::from("Antimatter")))])]);
        res_hash.insert(String::from("Antimatter"), antimatter);
        let core_element: Item =
            Item::new(
                "Core Element",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Core Element")))])]);
        res_hash.insert(String::from("Core Element"), core_element);
        critical_photon =
            Item::new(
                "Critical Photon",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Critical Photon")))])]);
        res_hash.insert(String::from("Critical Photon"), critical_photon);
        let kimberlite_ore: Item =
            Item::new(
                "Kimberlite Ore",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Kimberlite Ore")))])]);
        res_hash.insert(String::from("Kimberlite Ore"), kimberlite_ore);
        let fractal_silicon: Item =
            Item::new(
                "Fractal Silicon",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Fractal Silicon")))])]);
        res_hash.insert(String::from("Fractal Silicon"), fractal_silicon);
        let grating_crystal: Item =
            Item::new(
                "Grating Crystal",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    1.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Grating Crystal")))])]);
        res_hash.insert(String::from("Grating Crystal"), grating_crystal);
        let stalagmite_crystal: Item =
            Item::new(
                "Stalagmite Crystal",
                vec![ManFac::Origin], 
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Stalagmite Crystal")))])]);
        res_hash.insert(String::from("Stalagmite Crystal"), stalagmite_crystal);
        let unipolar_magnet: Item =
            Item::new(
                "Unipolar Magnet", 
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Unipolar Magnet")))])]);
        res_hash.insert(String::from("Unipolar Magnet"), unipolar_magnet);
        fire_ice =
            Item::new(
                "Fire Ice",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Fire Ice")))])]);
        res_hash.insert(String::from("Fire Ice"), fire_ice);
        let log: Item =
            Item::new(
                "Log",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Log")))])]);
        res_hash.insert(String::from("Log"), log);
        let plant_fuel: Item =
            Item::new(
                "Plant Fuel",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Plant Fuel")))])]);
        res_hash.insert(String::from("Plant Fuel"), plant_fuel);
        let dark_fog_matix: Item =
            Item::new(
                "Dark Fog Matrix",
                vec![ManFac::Origin], 
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Dark Fog Matrix")))])]);
        res_hash.insert(String::from("Dark Fog Matrix"), dark_fog_matix);
        let energy_shard: Item =
            Item::new(
                "Energy Shard",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Energy Shard")))])]);
        res_hash.insert(String::from("Energy Shard"), energy_shard);
        let silicon_based_neuron: Item =
            Item::new(
                "Silicon-based Neuron",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Silicon-based Neuron")))])]);
        res_hash.insert(String::from("Silicon-based Neuron"), silicon_based_neuron);
        let negentropy_singularity: Item =
            Item::new(
                "Negentropy Singularity",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Negentropy Singularity")))])]);
        res_hash.insert(String::from("Negentropy Singularity"), negentropy_singularity);
        let matter_recombinator: Item =
            Item::new(
                "Matter Recombinator",
                vec![ManFac::Origin],
                vec![Recipe::new(
                    0.0,
                    vec![IsItem::new_nai()],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Matter Recombinator")))])]);
        res_hash.insert(String::from("Matter Recombinator"), matter_recombinator);
        // processed items
        let iron_ingot =
            Item::new(
                "Iron Ingot",
                vec![ManFac::Furnace],
                vec![Recipe::new(
                    1.0,
                    vec![IsItem::new(ItemAmount::new(1, String::from("Iron Ore")))], 
                    vec![IsItem::new(ItemAmount::new(1, String::from("Iron Ingot")))])]);
        res_hash.insert(String::from("Iron Ingot"), iron_ingot);
        let copper_ingot: Item =
            Item::new(
                "Copper Ingot",
                vec![ManFac::Furnace],
                vec![Recipe::new(
                    1.0,
                    vec![IsItem::new(ItemAmount::new(1, String::from("Copper Ore")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Copper Ingot")))])]);
        res_hash.insert(String::from("Copper Ingot"), copper_ingot);
        let stone_brick: Item =
            Item::new(
                "Stone Brick",
                vec![ManFac::Furnace],
                vec![Recipe::new(
                    1.0,
                    vec![IsItem::new(ItemAmount::new(1, String::from("Stone")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Stone Brick")))])]);
        res_hash.insert(String::from("Stone Brick"), stone_brick);
        energetic_graphite =
            Item::new(
                "Energetic Graphite",
                vec![ManFac::Furnace],
                vec![Recipe::new(
                    2.0, 
                    vec![IsItem::new(ItemAmount::new(2, String::from("Coal")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Energetic Graphite")))]),
                    Recipe::new(
                    4.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Hydrogen"))),
                        IsItem::new(ItemAmount::new(1, String::from("Refined Oil")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Energetic Graphite"))),
                        IsItem::new(ItemAmount::new(3, String::from("Hydrogen")))])]);
        res_hash.insert(String::from("Energetic Graphite"), energetic_graphite);
        let high_purity_silicon: Item =
            Item::new(
                "High-purity Silicon",
                vec![ManFac::Furnace],
                vec![Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Silicon Ore")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("High-purity Silicon")))])]);
        res_hash.insert(String::from("High-purity Silicon"), high_purity_silicon);
        let titanium_ingot: Item =
            Item::new(
                "Titanium Ingot",
                vec![ManFac::Furnace],
                vec![Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Titanium Ore")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Titanium Ingot")))])]);
        res_hash.insert(String::from("Titanium Ingot"), titanium_ingot);
        let sulfuric_acid: Item =
            Item::new(
                "Sulfuric Acid",
                vec![ManFac::ChemicalPlant, ManFac::Origin],
                vec![Recipe::new(
                    6.0,
                    vec![IsItem::new(ItemAmount::new(4, String::from("Water"))),
                        IsItem::new(ItemAmount::new(8, String::from("Stone"))),
                        IsItem::new(ItemAmount::new(6, String::from("Refined Oil")))],
                    vec![IsItem::new(ItemAmount::new(4, String::from("Sulfuric Acid")))])]);
        res_hash.insert(String::from("Sulfuric Acid"), sulfuric_acid);
        refined_oil =
            Item::new(
                "Refined Oil",
                vec![ManFac::OilRefinery],
                vec![Recipe::new(
                    4.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Crude Oil")))],
                    vec![IsItem::new(ItemAmount::new(2, String::from("Refined Oil"))),
                        IsItem::new(ItemAmount::new(1, String::from("Hydrogen")))]),
                    Recipe::new(
                    4.0,
                    vec![IsItem::new(ItemAmount::new(1, String::from("Coal"))),
                        IsItem::new(ItemAmount::new(1, String::from("Hydrogen"))),
                        IsItem::new(ItemAmount::new(2, String::from("Refined Oil")))],
                    vec![IsItem::new(ItemAmount::new(3, String::from("Refined Oil")))])]);
        res_hash.insert(String::from("Refined Oil"), refined_oil);
        let magnet: Item = 
            Item::new(
                "Magnet",
                vec![ManFac::Furnace], 
                vec![Recipe::new(
                    1.5,
                    vec![IsItem::new(ItemAmount::new(1, String::from("Iron Ore")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Magnet")))])]);
        res_hash.insert(String::from("Magnet"), magnet);
        let magentic_coil: Item =
            Item::new(
                "Magnetic Coil",
                vec![ManFac::Assembler],
                vec![Recipe::new(
                    1.0,
                    vec![IsItem::new(ItemAmount::new(1, String::from("Copper Ingot"))),
                        IsItem::new(ItemAmount::new(2, String::from("Magnet")))],
                    vec![IsItem::new(ItemAmount::new(2, String::from("Magnetic Coil")))])]);
        res_hash.insert(String::from("Magnetic Coil"), magentic_coil);
        let glass :Item =
            Item::new(
                "Glass",
                vec![ManFac::Furnace],
                vec![Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Stone")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Glass")))])]);
        res_hash.insert(String::from("Glass"), glass);
        let diamond: Item =
            Item::new(
                "Diamond",
                vec![ManFac::Furnace],
                vec![Recipe::new(
                    2.0, 
                    vec![IsItem::new(ItemAmount::new(1, String::from("Energetic Graphite")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Diamond")))]),
                    Recipe::new(
                    1.5,
                    vec![IsItem::new(ItemAmount::new(1, String::from("Kimberlite Ore")))],
                    vec![IsItem::new(ItemAmount::new(2, String::from("Diamond")))])]);
        res_hash.insert(String::from("Diamond"), diamond);
        let crystal_silicon: Item =
            Item::new(
                "Crystal Silicon",
                vec![ManFac::Furnace],
                vec![Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(1, String::from("High-purity Silicon")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Crystal Silicon")))]),
                    Recipe::new(
                    1.5,
                    vec![IsItem::new(ItemAmount::new(1, String::from("Fractal Silicon")))],
                    vec![IsItem::new(ItemAmount::new(2, String::from("Crystal Silicon")))])]);
        res_hash.insert(String::from("Crystal Silicon"), crystal_silicon);
        let steel: Item =
            Item::new(
                "Steel",
                vec![ManFac::Furnace],
                vec![Recipe::new(
                    3.0,
                    vec![IsItem::new(ItemAmount::new(3, String::from("Iron Ingot")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Steel")))])]);
        res_hash.insert(String::from("Steel"), steel);
        let circuit_board: Item =
            Item::new(
                "Circuit Board",
                vec![ManFac::Assembler],
                vec![Recipe::new(
                    1.0,
                    vec![IsItem::new(ItemAmount::new(1, String::from("Copper Ingot"))),
                        IsItem::new(ItemAmount::new(2, String::from("Iron Ingot")))], 
                    vec![IsItem::new(ItemAmount::new(2, String::from("Circuit Board")))])]);
        res_hash.insert(String::from("Circuit Board"), circuit_board);
        let prism: Item =
            Item::new(
                "Prism",
                vec![ManFac::Furnace],
                vec![Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(3, String::from("Glass")))],
                    vec![IsItem::new(ItemAmount::new(2, String::from("Prism")))])]);
        res_hash.insert(String::from("Prism"), prism);
        let electric_motor: Item =
            Item::new(
                "Electric Motor", 
                vec![ManFac::Assembler],
                vec![Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(1, String::from("Magnetic Coil"))),
                        IsItem::new(ItemAmount::new(1, String::from("Gear"))),
                        IsItem::new(ItemAmount::new(2, String::from("Iron Ingot")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Electric Motor")))])]);
        res_hash.insert(String::from("Electric Motor"), electric_motor);
        let microcrystalline_component: Item =
            Item::new(
                "Microcrystalline Component",
                vec![ManFac::Furnace],
                vec![Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(1, String::from("Copper Ingot"))),
                        IsItem::new(ItemAmount::new(2, String::from("High-purity Silicon")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Microcrystalline Component")))])]);
        res_hash.insert(String::from("Microcrystalline Component"), microcrystalline_component);
        gear =
            Item::new(
                "Gear",
                vec![ManFac::Assembler],
                vec![Recipe::new(
                    1.0,
                    vec![IsItem::new(ItemAmount::new(1, String::from("Iron Ingot")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Gear")))])]);
        res_hash.insert(String::from("Gear"), gear);
        let plasma_exciter: Item =
            Item::new(
                "Plasma Exciter",
                vec![ManFac::Assembler],
                vec![Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Prism"))),
                        IsItem::new(ItemAmount::new(4, String::from("Magnetic Coil")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Plasma Exciter")))])]);
        res_hash.insert(String::from("Plasma Exciter"), plasma_exciter);
        let photon_combiner:Item =
            Item::new(
                "Photon Combiner",
                vec![ManFac::Assembler],
                vec![Recipe::new(
                    3.0,
                    vec![IsItem::new(ItemAmount::new(1, String::from("Circuit Board"))),
                        IsItem::new(ItemAmount::new(2, String::from("Prism")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Photon Combiner")))]),
                    Recipe::new(
                    3.0,
                    vec![IsItem::new(ItemAmount::new(1, String::from("Circuit Board"))),
                        IsItem::new(ItemAmount::new(2, String::from("Grating Crystal")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Photon Combiner")))])]);
        res_hash.insert(String::from("Photon Combiner"), photon_combiner);
        let electromagentic_turbine: Item =
            Item::new(
                "Electromagnetic Turbine",
                vec![ManFac::Assembler],
                vec![Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Magnetic Coil"))),
                        IsItem::new(ItemAmount::new(2, String::from("Electric Motor")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Electromagnetic Turbine")))])]);
        res_hash.insert(String::from("Electromagnetic Turbine"), electromagentic_turbine);
        let processor: Item =
            Item::new(
                "Processor",
                vec![ManFac::Assembler],
                vec![Recipe::new(
                    3.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Microcrystalline Component"))),
                        IsItem::new(ItemAmount::new(2, String::from("Circuit Board")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Processor")))])]);
        res_hash.insert(String::from("Processor"), processor);
        let engine: Item =
            Item::new(
                "Engine",
                vec![ManFac::Assembler],
                vec![Recipe::new(
                    3.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Copper Ingot"))),
                        IsItem::new(ItemAmount::new(1, String::from("Magnetic Coil")))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Engine")))])]);
        res_hash.insert(String::from("Engine"), engine);
        let thruster: Item = 
            item!("Thruster",
                (Assembler),
                (recipe!(4.0, (recitem!(3, "Copper Ingot")), (recitem!(2, "Steel")))));
        res_hash.insert(String::from("Thruster"), thruster);
        tohash!(res_hash,
            reinforced_thruster,
            "Reinforced Thruster",
            item!("Reinforced Thruster",
                (Assembler),
                (recipe!(6.0,
                    (recitem!(5, "Electromagnetic Turbine"), recitem!(5, "Titanium Alloy")),
                    (recitem!(4, "Reinforced Thruster"))))));
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
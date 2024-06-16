pub mod itemsmod {
    // import hasmaps
    use std::collections::HashMap;
    // import the structs, emuns, etc.
    use crate::v0_10_30_22243::essentials::item_logic::{IsItem, Item, ItemAmount, ManFac, Recipe};
    use crate::v0_10_30_22243::essentials::item_logic::{
        ManFac::Assembler, ManFac::ChemicalPlant, ManFac::Furnace, ManFac::Lab,
        ManFac::MiniatureParticleCollider,
    };
    // import all macros
    use crate::{item, recipe, recitem, tohash};
    //use crate::v0_10_30_22243::essentials::{};
    /// creates a big HashMap with all the items and buildings in the game
    pub fn get_items<'a>(mut res_hash: HashMap<String, Item>) -> HashMap<String, Item> {
        // all ores/origin items
        let iron_ore: Item;
        let iron_ore_rec: Recipe = Recipe::new(
            0.0,
            vec![IsItem::new_nai()],
            vec![IsItem::new(ItemAmount::new(1, String::from("Iron Ore")))],
        );
        iron_ore = Item::new("Iron Ore", vec![ManFac::Origin], vec![iron_ore_rec]);
        res_hash.insert(String::from("Iron Ore"), iron_ore);
        let copper_ore: Item = Item::new(
            "Copper Ore",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(1, String::from("Copper Ore")))],
            )],
        );
        res_hash.insert(String::from("Copper Ore"), copper_ore);
        let stone: Item = Item::new(
            "Stone",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(1, String::from("Stone")))],
            )],
        );
        res_hash.insert(String::from("Stone"), stone);
        let coal: Item = Item::new(
            "Coal",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(1, String::from("Coal")))],
            )],
        );
        res_hash.insert(String::from("Coal"), coal);
        let silicon_ore: Item = Item::new(
            "Silicon Ore",
            // also minable
            vec![ManFac::Furnace],
            vec![Recipe::new(
                10.0,
                vec![IsItem::new(ItemAmount::new(10, String::from("Stone")))],
                vec![IsItem::new(ItemAmount::new(1, String::from("Silicon Ore")))],
            )],
        );
        res_hash.insert(String::from("Silicon Ore"), silicon_ore);
        let titanium_ore: Item = Item::new(
            "Titanium Ore",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Titanium Ore"),
                ))],
            )],
        );
        res_hash.insert(String::from("Titanium Ore"), titanium_ore);
        let water: Item = Item::new(
            "Water",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(1, String::from("Water")))],
            )],
        );
        res_hash.insert(String::from("Water"), water);
        let crude_oil: Item = Item::new(
            "Crude Oil",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(1, String::from("Crude Oil")))],
            )],
        );
        res_hash.insert(String::from("Crude Oil"), crude_oil);
        // hydrogen requires a lot of other items to be declared
        let refined_oil: Item;
        let graphene: Item;
        let fire_ice: Item;
        let energetic_graphite: Item;
        let antimatter: Item;
        let critical_photon: Item;
        let gear: Item;
        let hydrogen: Item = Item::new(
            "Hydrogen",
            vec![ManFac::ChemicalPlant, ManFac::OilRefinery],
            vec![
                Recipe::new(
                    4.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Crude Oil")))],
                    vec![
                        IsItem::new(ItemAmount::new(2, String::from("Refined Oil"))),
                        IsItem::new(ItemAmount::new(1, String::from("Hydrogen"))),
                    ],
                ),
                Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Fire Ice")))],
                    vec![
                        IsItem::new(ItemAmount::new(1, String::from("Hydrogen"))),
                        IsItem::new(ItemAmount::new(2, String::from("Graphene"))),
                    ],
                ),
                Recipe::new(
                    4.0,
                    vec![
                        IsItem::new(ItemAmount::new(2, String::from("Hydrogen"))),
                        IsItem::new(ItemAmount::new(1, String::from("Refined Oil"))),
                    ],
                    vec![
                        IsItem::new(ItemAmount::new(1, String::from("Energetic Graphite"))),
                        IsItem::new(ItemAmount::new(3, String::from("Hydrogen"))),
                    ],
                ),
                Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(
                        2,
                        String::from("Critical Photon"),
                    ))],
                    vec![
                        IsItem::new(ItemAmount::new(2, String::from("Hydrogen"))),
                        IsItem::new(ItemAmount::new(2, String::from("Anitmatter"))),
                    ],
                ),
            ],
        );
        res_hash.insert(String::from("Hydrogen"), hydrogen);
        let deuterium: Item = Item::new(
            "Deuterium",
            vec![ManFac::Origin, ManFac::MiniatureParticleCollider],
            vec![Recipe::new(
                2.5,
                vec![IsItem::new(ItemAmount::new(10, String::from("Hydrogen")))],
                vec![IsItem::new(ItemAmount::new(5, String::from("Deuterium")))],
            )],
        );
        res_hash.insert(String::from("Deuterium"), deuterium);
        antimatter = Item::new(
            "Antimatter",
            vec![ManFac::MiniatureParticleCollider],
            vec![Recipe::new(
                2.0,
                vec![IsItem::new(ItemAmount::new(
                    2,
                    String::from("Critical Photon"),
                ))],
                vec![
                    IsItem::new(ItemAmount::new(2, String::from("Hydrogen"))),
                    IsItem::new(ItemAmount::new(2, String::from("Antimatter"))),
                ],
            )],
        );
        res_hash.insert(String::from("Antimatter"), antimatter);
        let core_element: Item = Item::new(
            "Core Element",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Core Element"),
                ))],
            )],
        );
        res_hash.insert(String::from("Core Element"), core_element);
        critical_photon = Item::new(
            "Critical Photon",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Critical Photon"),
                ))],
            )],
        );
        res_hash.insert(String::from("Critical Photon"), critical_photon);
        let kimberlite_ore: Item = Item::new(
            "Kimberlite Ore",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Kimberlite Ore"),
                ))],
            )],
        );
        res_hash.insert(String::from("Kimberlite Ore"), kimberlite_ore);
        let fractal_silicon: Item = Item::new(
            "Fractal Silicon",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Fractal Silicon"),
                ))],
            )],
        );
        res_hash.insert(String::from("Fractal Silicon"), fractal_silicon);
        let grating_crystal: Item = Item::new(
            "Grating Crystal",
            vec![ManFac::Origin],
            vec![Recipe::new(
                1.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Grating Crystal"),
                ))],
            )],
        );
        res_hash.insert(String::from("Grating Crystal"), grating_crystal);
        let stalagmite_crystal: Item = Item::new(
            "Stalagmite Crystal",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Stalagmite Crystal"),
                ))],
            )],
        );
        res_hash.insert(String::from("Stalagmite Crystal"), stalagmite_crystal);
        let unipolar_magnet: Item = Item::new(
            "Unipolar Magnet",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Unipolar Magnet"),
                ))],
            )],
        );
        res_hash.insert(String::from("Unipolar Magnet"), unipolar_magnet);
        fire_ice = Item::new(
            "Fire Ice",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(1, String::from("Fire Ice")))],
            )],
        );
        res_hash.insert(String::from("Fire Ice"), fire_ice);
        let log: Item = Item::new(
            "Log",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(1, String::from("Log")))],
            )],
        );
        res_hash.insert(String::from("Log"), log);
        let plant_fuel: Item = Item::new(
            "Plant Fuel",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(1, String::from("Plant Fuel")))],
            )],
        );
        res_hash.insert(String::from("Plant Fuel"), plant_fuel);
        let dark_fog_matix: Item = Item::new(
            "Dark Fog Matrix",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Dark Fog Matrix"),
                ))],
            )],
        );
        res_hash.insert(String::from("Dark Fog Matrix"), dark_fog_matix);
        let energy_shard: Item = Item::new(
            "Energy Shard",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Energy Shard"),
                ))],
            )],
        );
        res_hash.insert(String::from("Energy Shard"), energy_shard);
        let silicon_based_neuron: Item = Item::new(
            "Silicon-based Neuron",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Silicon-based Neuron"),
                ))],
            )],
        );
        res_hash.insert(String::from("Silicon-based Neuron"), silicon_based_neuron);
        let negentropy_singularity: Item = Item::new(
            "Negentropy Singularity",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Negentropy Singularity"),
                ))],
            )],
        );
        res_hash.insert(
            String::from("Negentropy Singularity"),
            negentropy_singularity,
        );
        let matter_recombinator: Item = Item::new(
            "Matter Recombinator",
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Matter Recombinator"),
                ))],
            )],
        );
        res_hash.insert(String::from("Matter Recombinator"), matter_recombinator);
        // processed items
        let iron_ingot = Item::new(
            "Iron Ingot",
            vec![ManFac::Furnace],
            vec![Recipe::new(
                1.0,
                vec![IsItem::new(ItemAmount::new(1, String::from("Iron Ore")))],
                vec![IsItem::new(ItemAmount::new(1, String::from("Iron Ingot")))],
            )],
        );
        res_hash.insert(String::from("Iron Ingot"), iron_ingot);
        let copper_ingot: Item = Item::new(
            "Copper Ingot",
            vec![ManFac::Furnace],
            vec![Recipe::new(
                1.0,
                vec![IsItem::new(ItemAmount::new(1, String::from("Copper Ore")))],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Copper Ingot"),
                ))],
            )],
        );
        res_hash.insert(String::from("Copper Ingot"), copper_ingot);
        let stone_brick: Item = Item::new(
            "Stone Brick",
            vec![ManFac::Furnace],
            vec![Recipe::new(
                1.0,
                vec![IsItem::new(ItemAmount::new(1, String::from("Stone")))],
                vec![IsItem::new(ItemAmount::new(1, String::from("Stone Brick")))],
            )],
        );
        res_hash.insert(String::from("Stone Brick"), stone_brick);
        energetic_graphite = Item::new(
            "Energetic Graphite",
            vec![ManFac::Furnace],
            vec![
                Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Coal")))],
                    vec![IsItem::new(ItemAmount::new(
                        1,
                        String::from("Energetic Graphite"),
                    ))],
                ),
                Recipe::new(
                    4.0,
                    vec![
                        IsItem::new(ItemAmount::new(2, String::from("Hydrogen"))),
                        IsItem::new(ItemAmount::new(1, String::from("Refined Oil"))),
                    ],
                    vec![
                        IsItem::new(ItemAmount::new(1, String::from("Energetic Graphite"))),
                        IsItem::new(ItemAmount::new(3, String::from("Hydrogen"))),
                    ],
                ),
            ],
        );
        res_hash.insert(String::from("Energetic Graphite"), energetic_graphite);
        let high_purity_silicon: Item = Item::new(
            "High-purity Silicon",
            vec![ManFac::Furnace],
            vec![Recipe::new(
                2.0,
                vec![IsItem::new(ItemAmount::new(2, String::from("Silicon Ore")))],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("High-purity Silicon"),
                ))],
            )],
        );
        res_hash.insert(String::from("High-purity Silicon"), high_purity_silicon);
        let titanium_ingot: Item = Item::new(
            "Titanium Ingot",
            vec![ManFac::Furnace],
            vec![Recipe::new(
                2.0,
                vec![IsItem::new(ItemAmount::new(
                    2,
                    String::from("Titanium Ore"),
                ))],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Titanium Ingot"),
                ))],
            )],
        );
        res_hash.insert(String::from("Titanium Ingot"), titanium_ingot);
        let sulfuric_acid: Item = Item::new(
            "Sulfuric Acid",
            vec![ManFac::ChemicalPlant, ManFac::Origin],
            vec![Recipe::new(
                6.0,
                vec![
                    IsItem::new(ItemAmount::new(4, String::from("Water"))),
                    IsItem::new(ItemAmount::new(8, String::from("Stone"))),
                    IsItem::new(ItemAmount::new(6, String::from("Refined Oil"))),
                ],
                vec![IsItem::new(ItemAmount::new(
                    4,
                    String::from("Sulfuric Acid"),
                ))],
            )],
        );
        res_hash.insert(String::from("Sulfuric Acid"), sulfuric_acid);
        refined_oil = Item::new(
            "Refined Oil",
            vec![ManFac::OilRefinery],
            vec![
                Recipe::new(
                    4.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Crude Oil")))],
                    vec![
                        IsItem::new(ItemAmount::new(2, String::from("Refined Oil"))),
                        IsItem::new(ItemAmount::new(1, String::from("Hydrogen"))),
                    ],
                ),
                Recipe::new(
                    4.0,
                    vec![
                        IsItem::new(ItemAmount::new(1, String::from("Coal"))),
                        IsItem::new(ItemAmount::new(1, String::from("Hydrogen"))),
                        IsItem::new(ItemAmount::new(2, String::from("Refined Oil"))),
                    ],
                    vec![IsItem::new(ItemAmount::new(3, String::from("Refined Oil")))],
                ),
            ],
        );
        res_hash.insert(String::from("Refined Oil"), refined_oil);
        let magnet: Item = Item::new(
            "Magnet",
            vec![ManFac::Furnace],
            vec![Recipe::new(
                1.5,
                vec![IsItem::new(ItemAmount::new(1, String::from("Iron Ore")))],
                vec![IsItem::new(ItemAmount::new(1, String::from("Magnet")))],
            )],
        );
        res_hash.insert(String::from("Magnet"), magnet);
        let magentic_coil: Item = Item::new(
            "Magnetic Coil",
            vec![ManFac::Assembler],
            vec![Recipe::new(
                1.0,
                vec![
                    IsItem::new(ItemAmount::new(1, String::from("Copper Ingot"))),
                    IsItem::new(ItemAmount::new(2, String::from("Magnet"))),
                ],
                vec![IsItem::new(ItemAmount::new(
                    2,
                    String::from("Magnetic Coil"),
                ))],
            )],
        );
        res_hash.insert(String::from("Magnetic Coil"), magentic_coil);
        let glass: Item = Item::new(
            "Glass",
            vec![ManFac::Furnace],
            vec![Recipe::new(
                2.0,
                vec![IsItem::new(ItemAmount::new(2, String::from("Stone")))],
                vec![IsItem::new(ItemAmount::new(1, String::from("Glass")))],
            )],
        );
        res_hash.insert(String::from("Glass"), glass);
        let diamond: Item = Item::new(
            "Diamond",
            vec![ManFac::Furnace],
            vec![
                Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(
                        1,
                        String::from("Energetic Graphite"),
                    ))],
                    vec![IsItem::new(ItemAmount::new(1, String::from("Diamond")))],
                ),
                Recipe::new(
                    1.5,
                    vec![IsItem::new(ItemAmount::new(
                        1,
                        String::from("Kimberlite Ore"),
                    ))],
                    vec![IsItem::new(ItemAmount::new(2, String::from("Diamond")))],
                ),
            ],
        );
        res_hash.insert(String::from("Diamond"), diamond);
        let crystal_silicon: Item = Item::new(
            "Crystal Silicon",
            vec![ManFac::Furnace],
            vec![
                Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(
                        1,
                        String::from("High-purity Silicon"),
                    ))],
                    vec![IsItem::new(ItemAmount::new(
                        1,
                        String::from("Crystal Silicon"),
                    ))],
                ),
                Recipe::new(
                    1.5,
                    vec![IsItem::new(ItemAmount::new(
                        1,
                        String::from("Fractal Silicon"),
                    ))],
                    vec![IsItem::new(ItemAmount::new(
                        2,
                        String::from("Crystal Silicon"),
                    ))],
                ),
            ],
        );
        res_hash.insert(String::from("Crystal Silicon"), crystal_silicon);
        let steel: Item = Item::new(
            "Steel",
            vec![ManFac::Furnace],
            vec![Recipe::new(
                3.0,
                vec![IsItem::new(ItemAmount::new(3, String::from("Iron Ingot")))],
                vec![IsItem::new(ItemAmount::new(1, String::from("Steel")))],
            )],
        );
        res_hash.insert(String::from("Steel"), steel);
        tohash!(
            res_hash,
            titanium_alloy,
            "Titanium Alloy",
            item!(
                "Titanium Alloy",
                (Furnace),
                (recipe!(
                    12.0,
                    (
                        recitem!(8, "Sulfuric Acid"),
                        recitem!(4, "Steel"),
                        recitem!(4, "Titanium Ingot")
                    ),
                    (recitem!(4, "Titanium Alloy"))
                ))
            )
        );
        let circuit_board: Item = Item::new(
            "Circuit Board",
            vec![ManFac::Assembler],
            vec![Recipe::new(
                1.0,
                vec![
                    IsItem::new(ItemAmount::new(1, String::from("Copper Ingot"))),
                    IsItem::new(ItemAmount::new(2, String::from("Iron Ingot"))),
                ],
                vec![IsItem::new(ItemAmount::new(
                    2,
                    String::from("Circuit Board"),
                ))],
            )],
        );
        res_hash.insert(String::from("Circuit Board"), circuit_board);
        let prism: Item = Item::new(
            "Prism",
            vec![ManFac::Furnace],
            vec![Recipe::new(
                2.0,
                vec![IsItem::new(ItemAmount::new(3, String::from("Glass")))],
                vec![IsItem::new(ItemAmount::new(2, String::from("Prism")))],
            )],
        );
        res_hash.insert(String::from("Prism"), prism);
        let electric_motor: Item = Item::new(
            "Electric Motor",
            vec![ManFac::Assembler],
            vec![Recipe::new(
                2.0,
                vec![
                    IsItem::new(ItemAmount::new(1, String::from("Magnetic Coil"))),
                    IsItem::new(ItemAmount::new(1, String::from("Gear"))),
                    IsItem::new(ItemAmount::new(2, String::from("Iron Ingot"))),
                ],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Electric Motor"),
                ))],
            )],
        );
        res_hash.insert(String::from("Electric Motor"), electric_motor);
        let microcrystalline_component: Item = Item::new(
            "Microcrystalline Component",
            vec![ManFac::Furnace],
            vec![Recipe::new(
                2.0,
                vec![
                    IsItem::new(ItemAmount::new(1, String::from("Copper Ingot"))),
                    IsItem::new(ItemAmount::new(2, String::from("High-purity Silicon"))),
                ],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Microcrystalline Component"),
                ))],
            )],
        );
        res_hash.insert(
            String::from("Microcrystalline Component"),
            microcrystalline_component,
        );
        gear = Item::new(
            "Gear",
            vec![ManFac::Assembler],
            vec![Recipe::new(
                1.0,
                vec![IsItem::new(ItemAmount::new(1, String::from("Iron Ingot")))],
                vec![IsItem::new(ItemAmount::new(1, String::from("Gear")))],
            )],
        );
        res_hash.insert(String::from("Gear"), gear);
        let plasma_exciter: Item = Item::new(
            "Plasma Exciter",
            vec![ManFac::Assembler],
            vec![Recipe::new(
                2.0,
                vec![
                    IsItem::new(ItemAmount::new(2, String::from("Prism"))),
                    IsItem::new(ItemAmount::new(4, String::from("Magnetic Coil"))),
                ],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Plasma Exciter"),
                ))],
            )],
        );
        res_hash.insert(String::from("Plasma Exciter"), plasma_exciter);
        let photon_combiner: Item = Item::new(
            "Photon Combiner",
            vec![ManFac::Assembler],
            vec![
                Recipe::new(
                    3.0,
                    vec![
                        IsItem::new(ItemAmount::new(1, String::from("Circuit Board"))),
                        IsItem::new(ItemAmount::new(2, String::from("Prism"))),
                    ],
                    vec![IsItem::new(ItemAmount::new(
                        1,
                        String::from("Photon Combiner"),
                    ))],
                ),
                Recipe::new(
                    3.0,
                    vec![
                        IsItem::new(ItemAmount::new(1, String::from("Circuit Board"))),
                        IsItem::new(ItemAmount::new(2, String::from("Grating Crystal"))),
                    ],
                    vec![IsItem::new(ItemAmount::new(
                        1,
                        String::from("Photon Combiner"),
                    ))],
                ),
            ],
        );
        res_hash.insert(String::from("Photon Combiner"), photon_combiner);
        let electromagentic_turbine: Item = Item::new(
            "Electromagnetic Turbine",
            vec![ManFac::Assembler],
            vec![Recipe::new(
                2.0,
                vec![
                    IsItem::new(ItemAmount::new(2, String::from("Magnetic Coil"))),
                    IsItem::new(ItemAmount::new(2, String::from("Electric Motor"))),
                ],
                vec![IsItem::new(ItemAmount::new(
                    1,
                    String::from("Electromagnetic Turbine"),
                ))],
            )],
        );
        res_hash.insert(
            String::from("Electromagnetic Turbine"),
            electromagentic_turbine,
        );
        let processor: Item = Item::new(
            "Processor",
            vec![ManFac::Assembler],
            vec![Recipe::new(
                3.0,
                vec![
                    IsItem::new(ItemAmount::new(
                        2,
                        String::from("Microcrystalline Component"),
                    )),
                    IsItem::new(ItemAmount::new(2, String::from("Circuit Board"))),
                ],
                vec![IsItem::new(ItemAmount::new(1, String::from("Processor")))],
            )],
        );
        res_hash.insert(String::from("Processor"), processor);
        let engine: Item = Item::new(
            "Engine",
            vec![ManFac::Assembler],
            vec![Recipe::new(
                3.0,
                vec![
                    IsItem::new(ItemAmount::new(2, String::from("Copper Ingot"))),
                    IsItem::new(ItemAmount::new(1, String::from("Magnetic Coil"))),
                ],
                vec![IsItem::new(ItemAmount::new(1, String::from("Engine")))],
            )],
        );
        res_hash.insert(String::from("Engine"), engine);
        let thruster: Item = item!(
            "Thruster",
            (Assembler),
            (recipe!(4.0, (recitem!(3, "Copper Ingot")), (recitem!(2, "Steel"))))
        );
        res_hash.insert(String::from("Thruster"), thruster);
        /*
        macro template
        tohash!(res_hash,
            ,
            ,
            item!(,
                (),
                (recipe!(,
                    (recitem!()),
                    (recitem!())))));
        */
        tohash!(
            res_hash,
            reinforced_thruster,
            "Reinforced Thruster",
            item!(
                "Reinforced Thruster",
                (Assembler),
                (recipe!(
                    6.0,
                    (
                        recitem!(5, "Electromagnetic Turbine"),
                        recitem!(5, "Titanium Alloy")
                    ),
                    (recitem!(4, "Reinforced Thruster"))
                ))
            )
        );
        tohash!(
            res_hash,
            super_magentic_ring,
            "Super-magnetic Ring",
            item!(
                "Super-magnetic Ring",
                (Assembler),
                (recipe!(
                    3.0,
                    (
                        recitem!(1, "Energetic Graphite"),
                        recitem!(3, "Magnet"),
                        recitem!(2, "Electromagnetic Turbine")
                    ),
                    (recitem!(1, "Super-magnetic Ring"))
                ))
            )
        );
        tohash!(
            res_hash,
            particle_container,
            "Particle Container",
            item!(
                "Particle Container",
                (Assembler),
                (
                    recipe!(
                        4.0,
                        (
                            recitem!(2, "Graphene"),
                            recitem!(2, "Copper Ingot"),
                            recitem!(2, "Electromagnetic Turbine")
                        ),
                        (recitem!(1, "Particle Container"))
                    ),
                    recipe!(
                        4.0,
                        (recitem!(2, "Copper Ingot")),
                        (recitem!(10, "Unipolar Magnet"))
                    )
                )
            )
        );
        tohash!(
            res_hash,
            plastic,
            "Plastic",
            item!(
                "Plastic",
                (ChemicalPlant),
                (recipe!(
                    3.0,
                    (
                        recitem!(1, "Energetic Graphite"),
                        recitem!(2, "Refined Oil")
                    ),
                    (recitem!(1, "Plastic"))
                ))
            )
        );
        tohash!(
            res_hash,
            organic_crystal,
            "Organic Crystal",
            item!(
                "Organic Crystal",
                (ChemicalPlant),
                (
                    recipe!(
                        6.0,
                        (
                            recitem!(1, "Water"),
                            recitem!(1, "Refined Oil"),
                            recitem!(2, "Plastic")
                        ),
                        (recitem!(1, "Organic Crystal"))
                    ),
                    recipe!(
                        6.0,
                        (
                            recitem!(10, "Water"),
                            recitem!(30, "Plant Fuel"),
                            recitem!(20, "Log"),
                        ),
                        (recitem!(1, "Organic Crystal"))
                    )
                )
            )
        );
        // can't use macros because of the declaration earlier
        graphene = Item::new(
            "Graphene",
            vec![ManFac::ChemicalPlant],
            vec![
                Recipe::new(
                    3.0,
                    vec![
                        IsItem::new(ItemAmount::new(1, String::from("Sulfuric Acid"))),
                        IsItem::new(ItemAmount::new(3, String::from("Energetic Graphite"))),
                    ],
                    vec![IsItem::new(ItemAmount::new(2, String::from("Graphene")))],
                ),
                Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(2, String::from("Fire Ice")))],
                    vec![
                        IsItem::new(ItemAmount::new(1, String::from("Hydrogen"))),
                        IsItem::new(ItemAmount::new(2, String::from("Graphene"))),
                    ],
                ),
            ],
        );
        res_hash.insert(String::from("Graphene"), graphene);
        tohash!(
            res_hash,
            annihilation_constraint_sphere,
            "Annihilation Constraint Sphere",
            item!(
                "Annihilation Constraint Sphere",
                (Assembler),
                (recipe!(
                    20.0,
                    (recitem!(1, "Processor"), recitem!(1, "Particle Container")),
                    (recitem!(1, "Annihilation Constraint Sphere"))
                ))
            )
        );
        tohash!(
            res_hash,
            strange_matter,
            "Strange Matter",
            item!(
                "Strange Matter",
                (MiniatureParticleCollider),
                (recipe!(
                    8.0,
                    (
                        recitem!(10, "Deuterium"),
                        recitem!(2, "Iron Ingot"),
                        recitem!(1, "Particle Container")
                    ),
                    (recitem!(1, "Strange Matter"))
                ))
            )
        );
        tohash!(
            res_hash,
            titanium_crystal,
            "Titanium Crystal",
            item!(
                "Titanium Crystal",
                (Assembler),
                (recipe!(
                    4.0,
                    (
                        recitem!(3, "Titanium Ingot"),
                        recitem!(1, "Organic Crystal")
                    ),
                    (recitem!(1, "Titanium Crystal"))
                ))
            )
        );
        tohash!(
            res_hash,
            carbon_nanotube,
            "Carbon Nanotube",
            item!(
                "Carbon Nanotube",
                (ChemicalPlant),
                (
                    recipe!(
                        4.0,
                        (recitem!(1, "Titanium Ingot"), recitem!(3, "Graphene")),
                        (recitem!(2, "Carbon Nanotube"))
                    ),
                    recipe!(
                        4.0,
                        (recitem!(6, "Stalagmite Crystal")),
                        (recitem!(2, "Carbon Nanotube"))
                    )
                )
            )
        );
        tohash!(
            res_hash,
            particle_broadband,
            "Particle Broadband",
            item!(
                "Particle Broadband",
                (Assembler),
                (recipe!(
                    8.0,
                    (
                        recitem!(1, "Plastic"),
                        recitem!(2, "Crystal Silicon"),
                        recitem!(2, "Carbon Nanotube")
                    ),
                    (recitem!(1, "Particle Broadband"))
                ))
            )
        );
        tohash!(
            res_hash,
            casimir_crystal,
            "Casimir Crystal",
            item!(
                "Casimir Crystal",
                (Assembler),
                (
                    recipe!(
                        4.0,
                        (
                            recitem!(12, "Hydrogen"),
                            recitem!(2, "Graphene"),
                            recitem!(1, "Titanium Crystal")
                        ),
                        (recitem!(1, "Casimir Crystal"))
                    ),
                    recipe!(
                        4.0,
                        (
                            recitem!(12, "Hydrogen"),
                            recitem!(2, "Graphene"),
                            recitem!(8, "Grating Crystal")
                        ),
                        (recitem!(1, "Casimir Crystal"))
                    )
                )
            )
        );
        tohash!(
            res_hash,
            titanium_glass,
            "Titanium Glass",
            item!(
                "Titanium Glass",
                (Assembler),
                (recipe!(
                    5.0,
                    (
                        recitem!(2, "Water"),
                        recitem!(2, "Titanium Ingot"),
                        recitem!(2, "Glass")
                    ),
                    (recitem!(2, "Titanium Glass"))
                ))
            )
        );
        tohash!(
            res_hash,
            plane_filter,
            "Plane Filter",
            item!(
                "Plane Filter",
                (Assembler),
                (recipe!(
                    12.0,
                    (
                        recitem!(2, "Titanium Glass"),
                        recitem!(1, "Casimir Crystal")
                    ),
                    (recitem!(1, "Plane Filter"))
                ))
            )
        );
        tohash!(
            res_hash,
            quantum_chip,
            "Quantum Chip",
            item!(
                "Quantum Chip",
                (Assembler),
                (recipe!(
                    6.0,
                    (recitem!(2, "Plane Filter"), recitem!(2, "Processor")),
                    (recitem!(1, "Quantum Chip"))
                ))
            )
        );

        tohash!(
            res_hash,
            combustible_unit,
            "Combustible Unit",
            item!(
                "Combustible Unit",
                (Assembler),
                (recipe!(
                    3.0,
                    (recitem!(3, "Coal")),
                    (recitem!(1, "Combustible Unit"))
                ))
            )
        );
        // logistics
        tohash!(
            res_hash,
            logisitcs_bot,
            "Logistics Bot",
            item!(
                "Logistics Bot",
                (Assembler),
                (recipe!(
                    2.0,
                    (
                        recitem!(1, "Processor"),
                        recitem!(1, "Engine"),
                        recitem!(2, "Iron Ingot")
                    ),
                    (recitem!(1, "Logistics Bot"))
                ))
            )
        );
        tohash!(
            res_hash,
            logisitcs_drone,
            "Logistics Drone",
            item!(
                "Logistics Drone",
                (Assembler),
                (recipe!(
                    4.0,
                    (
                        recitem!(2, "Thruster"),
                        recitem!(2, "Processor"),
                        recitem!(5, "Iron Ingot")
                    ),
                    (recitem!(1, "Logistics Drone"))
                ))
            )
        );
        tohash!(
            res_hash,
            interstellar_logistics_vessel,
            "Interstellar Logistics Vessel",
            item!(
                "Interstellar Logistics Vessel",
                (Assembler),
                (recipe!(
                    6.0,
                    (
                        recitem!(2, "Reinforced Thruster"),
                        recitem!(10, "Processor"),
                        recitem!(10, "Titanium Alloy")
                    ),
                    (recitem!(1, "Interstellar Logistics Vessel"))
                ))
            )
        );
        tohash!(
            res_hash,
            graviton_lens,
            "Graviton Lens",
            item!(
                "Graviton Lens",
                (Assembler),
                (recipe!(
                    6.0,
                    (recitem!(1, "Strange Matter"), recitem!(4, "Diamond")),
                    (recitem!(1, "Graviton Lens"))
                ))
            )
        );
        tohash!(
            res_hash,
            space_warper,
            "Space Warper",
            item!(
                "Space Warper",
                (Assembler),
                (
                    recipe!(
                        10.0,
                        (recitem!(1, "Graviton Lens")),
                        (recitem!(1, "Space Warper"))
                    ),
                    recipe!(
                        10.0,
                        (recitem!(1, "Gravity Matrix")),
                        (recitem!(8, "Space Warper"))
                    )
                )
            )
        );
        tohash!(
            res_hash,
            foundation,
            "Foundation",
            item!(
                "Foundation",
                (Assembler),
                (recipe!(
                    1.0,
                    (recitem!(1, "Steel"), recitem!(3, "Stone Brick")),
                    (recitem!(1, "Foundation"))
                ))
            )
        );
        // proliferator
        tohash!(
            res_hash,
            proliferator_mki,
            "Proliferator Mk.I",
            item!(
                "Proliferator Mk.I",
                (Assembler),
                (recipe!(
                    0.5,
                    (recitem!(1, "Coal")),
                    (recitem!(1, "Proliferator Mk.I"))
                ))
            )
        );
        tohash!(
            res_hash,
            proliferator_mkii,
            "Proliferator MK.II",
            item!(
                "Proliferator MK.II",
                (Assembler),
                (recipe!(
                    1.0,
                    (recitem!(1, "Diamond"), recitem!(2, "Proliferator Mk.I")),
                    (recitem!(1, "Proliferator Mk.II"))
                ))
            )
        );
        tohash!(
            res_hash,
            proliferator_mkiii,
            "Proliferator Mk.III",
            item!(
                "Proliferator Mk.III",
                (Assembler),
                (recipe!(
                    1.0,
                    (recitem!(1, "Diamond"), recitem!(2, "Proliferator Mk.II")),
                    (recitem!(1, "Proliferator Mk.III"))
                ))
            )
        );
        // fuel rods
        tohash!(
            res_hash,
            hydrogen_fuel_rod,
            "Hydrogen Fuel Rod",
            item!(
                "Hydrogen Fuel Rod",
                (Assembler),
                (recipe!(
                    6.0,
                    (recitem!(10, "Hydrogen"), recitem!(1, "Titanium Ingot")),
                    (recitem!(2, "Hydrogen Fuel Rod"))
                ))
            )
        );
        tohash!(
            res_hash,
            deuterium_fuel_rod,
            "Deuterium Fuel Rod",
            item!(
                "Deuterium Fuel Rod",
                (Assembler),
                (recipe!(
                    12.0,
                    (
                        recitem!(1, "Super-magnetic Ring"),
                        recitem!(20, "Deuterium"),
                        recitem!(1, "Titanium Alloy")
                    ),
                    (recitem!(2, "Deuterium Fuel Rod"))
                ))
            )
        );
        tohash!(
            res_hash,
            antimatter_fuel_rod,
            "Antimatter Fuel Rod",
            item!(
                "Antimatter Fuel Rod",
                (Assembler),
                (recipe!(
                    24.0,
                    (
                        recitem!(1, "Titanium Alloy"),
                        recitem!(1, "Annihilation Constraint Sphere"),
                        recitem!(12, "Hydrogen"),
                        recitem!(12, "Antimatter")
                    ),
                    (recitem!(2, "Antimatter Fuel Rod"))
                ))
            )
        );
        tohash!(
            res_hash,
            strange_anihilation_fuel_rod,
            "Strange Annihilation Fuel Rod",
            item!(
                "Strange Annihilation Fuel Rod",
                (Assembler),
                (recipe!(
                    32.0,
                    (
                        recitem!(1, "Frame Material"),
                        recitem!(2, "Strange Matter"),
                        recitem!(1, "Core Element"),
                        recitem!(8, "Antimatter Fuel Rod")
                    ),
                    (recitem!(1, "Strange Annihilation Fuel Rod"))
                ))
            )
        );
        // dyson shpere building
        tohash!(
            res_hash,
            solar_sail,
            "Solar Sail",
            item!(
                "Solar Sail",
                (Assembler),
                (recipe!(
                    4.0,
                    (recitem!(1, "Photon Combiner"), recitem!(1, "Graphene")),
                    (recitem!(2, "Solar Sail"))
                ))
            )
        );
        tohash!(
            res_hash,
            frame_material,
            "Frame Material",
            item!(
                "Frame Material",
                (Assembler),
                (recipe!(
                    6.0,
                    (
                        recitem!(1, "High-purity Silicon"),
                        recitem!(1, "Titanium Alloy"),
                        recitem!(4, "Carbon Nanotube")
                    ),
                    (recitem!(1, "Frame Material"))
                ))
            )
        );
        tohash!(
            res_hash,
            dyson_sphere_component,
            "Dyson Sphere Component",
            item!(
                "Dyson Sphere Component",
                (Assembler),
                (recipe!(
                    8.0,
                    (
                        recitem!(3, "Processor"),
                        recitem!(3, "Solar Sail"),
                        recitem!(3, "Frame Material")
                    ),
                    (recitem!(1, "Dyson Sphere Component"))
                ))
            )
        );
        tohash!(
            res_hash,
            small_carrier_rocket,
            "Small Carrier Rocket",
            item!(
                "Small Carrier Rocket",
                (Assembler),
                (recipe!(
                    6.0,
                    (
                        recitem!(2, "Quantum Chip"),
                        recitem!(4, "Deuterium Fuel Rod"),
                        recitem!(2, "Dyson Sphere Component")
                    ),
                    (recitem!(1, "Small Carrier Rocket"))
                ))
            )
        );
        // Matrices
        tohash!(
            res_hash,
            electromagnetic_matrix,
            "Electromagnetic Matrix",
            item!(
                "Electromagnetic Matrix",
                (Lab),
                (recipe!(
                    3.0,
                    (recitem!(1, "Circuit Board"), recitem!(1, "Magnetic Coil")),
                    (recitem!(1, "Electromagnetic Matrix"))
                ))
            )
        );
        tohash!(
            res_hash,
            energy_matrix,
            "Energy Matrix",
            item!(
                "Energy Matrix",
                (Lab),
                (recipe!(
                    6.0,
                    (recitem!(2, "Hydrogen"), recitem!(2, "Energetic Graphite")),
                    (recitem!(1, "Energy Matrix"))
                ))
            )
        );
        tohash!(
            res_hash,
            structure_matrix,
            "Structure Matrix",
            item!(
                "Structure Matrix",
                (Lab),
                (recipe!(
                    8.0,
                    (recitem!(1, "Titanium Crystal"), recitem!(1, "Diamond")),
                    (recitem!(1, "Structure Matrix"))
                ))
            )
        );
        tohash!(
            res_hash,
            information_matrix,
            "Information Matrix",
            item!(
                "Information Matrix",
                (Lab),
                (recipe!(
                    10.0,
                    (recitem!(1, "Particle Broadband"), recitem!(2, "Processor")),
                    (recitem!(1, "Information Matrix"))
                ))
            )
        );
        tohash!(
            res_hash,
            gravity_matrix,
            "Gravity Matrix",
            item!(
                "Gravity Matrix",
                (Lab),
                (recipe!(
                    24.0,
                    (recitem!(1, "Quantum Chip"), recitem!(1, "Graviton Lens")),
                    (recitem!(2, "Gravity Matrix"))
                ))
            )
        );
        tohash!(
            res_hash,
            universe_matrix,
            "Universe Matrix",
            item!(
                "Universe Matrix",
                (Lab),
                (recipe!(
                    15.0,
                    (
                        recitem!(1, "Electromagnetic Matrix"),
                        recitem!(1, "Energy Matrix"),
                        recitem!(1, "Structure Matrix"),
                        recitem!(1, "Information Matrix"),
                        recitem!(1, "Gravity Matrix"),
                        recitem!(1, "Antimatter")
                    ),
                    (recitem!(1, "Universe Matrix"))
                ))
            )
        );
        // ammunition
        tohash!(
            res_hash,
            magnum_ammo_box,
            "Magnum Ammo Box",
            item!(
                "Magnum Ammo Box",
                (Assembler),
                (recipe!(
                    1.0,
                    (recitem!(1, "Copper Ingot")),
                    (recitem!(1, "Magnum Ammo Box"))
                ))
            )
        );
        tohash!(
            res_hash,
            titanium_ammo_box,
            "Titanium Ammo Box",
            item!(
                "Titanium Ammo Box",
                (Assembler),
                (recipe!(
                    2.0,
                    (
                        recitem!(2, "Titanium Ingot"),
                        recitem!(1, "Magnum Ammo Box")
                    ),
                    (recitem!(1, "Titanium Ammo Box"))
                ))
            )
        );
        tohash!(
            res_hash,
            superalloy_ammo_box,
            "Superalloy Ammo Box",
            item!(
                "Superalloy Ammo Box",
                (Assembler),
                (recipe!(
                    3.0,
                    (
                        recitem!(1, "Titanium Alloy"),
                        recitem!(1, "Titanium Ammo Box")
                    ),
                    (recitem!(1, "Superalloy Ammo Box"))
                ))
            )
        );
        tohash!(
            res_hash,
            explosive_unit,
            "Explosive Unit",
            item!(
                "Explosive Unit",
                (ChemicalPlant),
                (recipe!(
                    6.0,
                    (
                        recitem!(1, "Sulfuric Acid"),
                        recitem!(2, "Plastic"),
                        recitem!(2, "Combustible Unit")
                    ),
                    (recitem!(2, "Explosive Unit"))
                ))
            )
        );
        tohash!(
            res_hash,
            crystal_explosive_unit,
            "Crystal Explosive Unit",
            item!(
                "Crystal Explosive Unit",
                (ChemicalPlant),
                (recipe!(
                    24.0,
                    (
                        recitem!(8, "Crystal Silicon"),
                        recitem!(1, "Casimir Crystal"),
                        recitem!(8, "Explosive Unit")
                    ),
                    (recitem!(8, "Crystal Explosive Unit"))
                ))
            )
        );
        tohash!(
            res_hash,
            missile_set,
            "Missile Set",
            item!(
                "Missile Set",
                (Assembler),
                (recipe!(
                    2.0,
                    (
                        recitem!(1, "Engine"),
                        recitem!(2, "Combustible Unit"),
                        recitem!(3, "Circuit Board"),
                        recitem!(6, "Copper Ingot")
                    ),
                    (recitem!(1, "Missile Set"))
                ))
            )
        );
        tohash!(
            res_hash,
            supersonic_missile_set,
            "Supersonic Missile Set",
            item!(
                "Supersonic Missile Set",
                (Assembler),
                (recipe!(
                    4.0,
                    (
                        recitem!(2, "Thruster"),
                        recitem!(4, "Explosive Unit"),
                        recitem!(4, "Processor"),
                        recitem!(2, "Missile Set")
                    ),
                    (recitem!(2, "Supersonic Missile Set"))
                ))
            )
        );
        tohash!(
            res_hash,
            gravity_missile_set,
            "Gravity Missile Set",
            item!(
                "Gravity Missile Set",
                (Assembler),
                (recipe!(
                    6.0,
                    (
                        recitem!(3, "Strange Matter"),
                        recitem!(6, "Crystal Explosive Unit"),
                        recitem!(3, "Supersonic Missile Set")
                    ),
                    (recitem!(3, "Gravity Missile Set"))
                ))
            )
        );
        tohash!(
            res_hash,
            shell_set,
            "Shell Set",
            item!(
                "Shell Set",
                (Assembler),
                (recipe!(
                    1.5,
                    (recitem!(2, "Explosive Unit"), recitem!(9, "Copper Ingot")),
                    (recitem!(1, "Shell Set"))
                ))
            )
        );
        tohash!(
            res_hash,
            high_explosive_shell_set,
            "High-Explosive Shell Set",
            item!(
                "High-Explosive Shell Set",
                (Assembler),
                (recipe!(
                    3.0,
                    (
                        recitem!(2, "Explosive Unit"),
                        recitem!(6, "Titanium Ingot"),
                        recitem!(1, "Shell Set")
                    ),
                    (recitem!(1, "High-Explosive Shell Set"))
                ))
            )
        );
        tohash!(
            res_hash,
            crystal_shell_set,
            "Crystal Shell Set",
            item!(
                "Crystal Shell Set",
                (Assembler),
                (recipe!(
                    6.0,
                    (
                        recitem!(2, "Crystal Explosive Unit"),
                        recitem!(3, "Titanium Ingot"),
                        recitem!(1, "High-Explosive Shell Set")
                    ),
                    (recitem!(1, "Crystal Shell Set"))
                ))
            )
        );
        tohash!(
            res_hash,
            plasma_capsule,
            "Plasma Capsule",
            item!(
                "Plasma Capsule",
                (Assembler),
                (recipe!(
                    2.0,
                    (
                        recitem!(10, "Deuterium"),
                        recitem!(2, "Magnet"),
                        recitem!(1, "Graphene")
                    ),
                    (recitem!(1, "Plasma Capsule"))
                ))
            )
        );
        tohash!(
            res_hash,
            anitmatter_capsule,
            "Antimatter Capsule",
            item!(
                "Antimatter Capsule",
                (Assembler),
                (recipe!(
                    2.0,
                    (
                        recitem!(10, "Antimatter"),
                        recitem!(10, "Hydrogen"),
                        recitem!(1, "Particle Container"),
                        recitem!(1, "Plasma Capsule")
                    ),
                    (recitem!(1, "Antimatter Capsule"))
                ))
            )
        );
        tohash!(
            res_hash,
            jamming_capsule,
            "Jamming Capsule",
            item!(
                "Jamming Capsule",
                (Assembler),
                (recipe!(
                    2.0,
                    (
                        recitem!(3, "Hydrogen"),
                        recitem!(1, "Plasma Exciter"),
                        recitem!(1, "Electromagnetic Turbine")
                    ),
                    (recitem!(1, "Jamming Capsule"))
                ))
            )
        );
        tohash!(
            res_hash,
            suppressing_capsule,
            "Suppressing Capsule",
            item!(
                "Suppressing Capsule",
                (Assembler),
                (recipe!(
                    8.0,
                    (
                        recitem!(2, "Glass"),
                        recitem!(1, "Super-magnetic Ring"),
                        recitem!(2, "Jamming Capsule")
                    ),
                    (recitem!(2, "Suppressing Capsule"))
                ))
            )
        );
        // drones and ships
        tohash!(
            res_hash,
            prototype,
            "Prototype",
            item!(
                "Prototype",
                (Assembler),
                (recipe!(
                    2.0,
                    (
                        recitem!(1, "Plasma Exciter"),
                        recitem!(2, "Circuit Board"),
                        recitem!(1, "Engine"),
                        recitem!(3, "Iron Ingot")
                    ),
                    (recitem!(1, "Prototype"))
                ))
            )
        );
        tohash!(
            res_hash,
            precision_drone,
            "Precision Drone",
            item!(
                "Precision Drone",
                (Assembler),
                (recipe!(
                    4.0,
                    (
                        recitem!(2, "Photon Combiner"),
                        recitem!(2, "Circuit Board"),
                        recitem!(1, "Electromagnetic Turbine"),
                        recitem!(1, "Prototype")
                    ),
                    (recitem!(1, "Precision Drone"))
                ))
            )
        );
        tohash!(
            res_hash,
            attack_drone,
            "Attack Drone",
            item!(
                "Attack Drone",
                (Assembler),
                (recipe!(
                    4.0,
                    (
                        recitem!(1, "Particle Container"),
                        recitem!(1, "Processor"),
                        recitem!(1, "Electromagnetic Turbine"),
                        recitem!(1, "Prototype")
                    ),
                    (recitem!(1, "Attack Drone"))
                ))
            )
        );
        tohash!(
            res_hash,
            corvette,
            "Corvette",
            item!(
                "Corvette",
                (Assembler),
                (recipe!(
                    5.0,
                    (
                        recitem!(3, "Particle Container"),
                        recitem!(2, "Processor"),
                        recitem!(1, "Reinforced Thruster"),
                        recitem!(5, "Titanium Alloy")
                    ),
                    (recitem!(1, "Corvette"))
                ))
            )
        );
        tohash!(
            res_hash,
            destroyer,
            "Destroyer",
            item!(
                "Destroyer",
                (Assembler),
                (recipe!(
                    8.0,
                    (
                        recitem!(1, "Strange Matter"),
                        recitem!(4, "Processor"),
                        recitem!(4, "Reinforced Thruster"),
                        recitem!(20, "Frame Material")
                    ),
                    (recitem!(1, "Destroyer"))
                ))
            )
        );
        // return the hashmap with all the items
        res_hash
    }
}

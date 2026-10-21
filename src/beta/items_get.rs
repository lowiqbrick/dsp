// all crafting recipe numbers are from the [wiki](https://dsp-wiki.com/Items) are

// import hashmaps
use std::collections::HashMap;
// import the structs, emuns, etc.
use crate::beta::essentials::{IsItem, Item, ItemAmount, ManFac, Recipe};
// import all macros
use crate::{item, recipe, recitem, to_hash};

/// creates a big HashMap with all the items and buildings in the game
pub fn get_items() -> HashMap<String, Item<'static>> {
    let mut res_hash = HashMap::new();
    /*
    macro template explanation
    to_hash!(
       res_hash,
        <variable name>,
        <item name string>,
        item!(
            <item name string>,
            (recipe!(
                <seconds to craft>,
                <Manufacturing facility>,
                    (recitem!(<Amount Item>, <ingredient item name string>)),
                    (recitem!(<Amount Item>, <product item name string>))
            ))
        )
    );
    copy paste reference
    to_hash!(
       res_hash,
        ,
        ,
        item!(
            ,
            (recipe!(
                ,
                ,
                    (recitem!(,)),
                    (recitem!(,))
            ))
        )
    );
    */
    // all ores/origin items
    to_hash!(
        res_hash,
        iron_ore,
        "Iron Ore",
        item!(
            "Iron Ore",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Iron Ore"))
            ))
        )
    );
    to_hash!(
        res_hash,
        copper_ore,
        "Copper Ore",
        item!(
            "Copper Ore",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Copper Ore"))
            ))
        )
    );
    to_hash!(
        res_hash,
        stone,
        "Stone",
        item!(
            "Stone",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Stone"))
            ))
        )
    );
    to_hash!(
        res_hash,
        coal,
        "Coal",
        item!(
            "Coal",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Coal"))
            ))
        )
    );
    to_hash!(
        res_hash,
        silicon_ore,
        "Silicon Ore",
        item!(
            "Silicon Ore",
            (
                recipe!(
                    0.0,
                    ManFac::Origin,
                    (IsItem::new_nai()),
                    (recitem!(1.0, "Silicon Ore"))
                ),
                recipe!(
                    10.0,
                    ManFac::Furnace,
                    (recitem!(10.0, "Stone")),
                    (recitem!(1.0, "Silicon Ore"))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        titanium_ore,
        "Titanium Ore",
        item!(
            "Titanium Ore",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Titanium Ore"))
            ))
        )
    );
    to_hash!(
        res_hash,
        water,
        "Water",
        item!(
            "Water",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Water"))
            ))
        )
    );
    to_hash!(
        res_hash,
        crude_oil,
        "Crude Oil",
        item!(
            "Crude Oil",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Crude Oil"))
            ))
        )
    );
    // hydrogen requires a lot of other items to be declared
    to_hash!(
        res_hash,
        hydrogen,
        "Hydrogen",
        item!(
            "Hydrogen",
            (
                recipe!(
                    4.0,
                    ManFac::OilRefinery,
                    (recitem!(2.0, "Crude Oil")),
                    (recitem!(2.0, "Refined Oil"), recitem!(1.0, "Hydrogen"))
                ),
                recipe!(
                    2.0,
                    ManFac::ChemicalPlant,
                    (recitem!(2.0, "Fire Ice")),
                    (recitem!(1.0, "Hydrogen"), recitem!(2.0, "Graphene"))
                ),
                recipe!(
                    4.0,
                    ManFac::OilRefinery,
                    (recitem!(2.0, "Hydrogen"), recitem!(1.0, "Refined Oil")),
                    (
                        recitem!(1.0, "Energetic Graphite"),
                        recitem!(3.0, "Hydrogen")
                    )
                ),
                recipe!(
                    2.0,
                    ManFac::MiniatureParticleCollider,
                    (recitem!(2.0, "Critical Photon")),
                    (recitem!(2.0, "Hydrogen"), recitem!(2.0, "Antimatter"))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        deuterium,
        "Deuterium",
        item!(
            "Deuterium",
            (
                recipe!(
                    0.0,
                    ManFac::Origin,
                    (IsItem::new_nai()),
                    (recitem!(1.0, "Deuterium"))
                ),
                recipe!(
                    2.5,
                    ManFac::MiniatureParticleCollider,
                    (recitem!(10.0, "Hydrogen")),
                    (recitem!(5.0, "Deuterium"))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        antimatter,
        "Antimatter",
        item!(
            "Antimatter",
            (recipe!(
                2.0,
                ManFac::MiniatureParticleCollider,
                (recitem!(2.0, "Critical Photon")),
                (recitem!(2.0, "Hydrogen"), recitem!(2.0, "Antimatter"))
            ))
        )
    );
    to_hash!(
        res_hash,
        core_element,
        "Core Element",
        item!(
            "Core Element",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Core Element"))
            ))
        )
    );
    to_hash!(
        res_hash,
        critical_photon,
        "Critical Photon",
        item!(
            "Critical Photon",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Critical Photon"))
            ))
        )
    );
    to_hash!(
        res_hash,
        kimberlite_ore,
        "Kimberlite Ore",
        item!(
            "Kimberlite Ore",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Kimberlite Ore"))
            ))
        )
    );
    to_hash!(
        res_hash,
        fractal_silicon,
        "Fractal Silicon",
        item!(
            "Fractal Silicon",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Fractal Silicon"))
            ))
        )
    );
    to_hash!(
        res_hash,
        grating_crystal,
        "Grating Crystal",
        item!(
            "Grating Crystal",
            (recipe!(
                1.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Grating Crystal"))
            ))
        )
    );
    to_hash!(
        res_hash,
        stalagmite_crystal,
        "Stalagmite Crystal",
        item!(
            "Stalagmite Crystal",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Stalagmite Crystal"))
            ))
        )
    );
    to_hash!(
        res_hash,
        unipolar_magnet,
        "Unipolar Magnet",
        item!(
            "Unipolar Magnet",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Unipolar Magnet"))
            ))
        )
    );
    to_hash!(
        res_hash,
        fire_ice,
        "Fire Ice",
        item!(
            "Fire Ice",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Fire Ice"))
            ))
        )
    );
    to_hash!(
        res_hash,
        log,
        "Log",
        item!(
            "Log",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Log"))
            ))
        )
    );
    to_hash!(
        res_hash,
        plant_fuel,
        "Plant Fuel",
        item!(
            "Plant Fuel",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Plant Fuel"))
            ))
        )
    );
    to_hash!(
        res_hash,
        dark_fog_matrix,
        "Dark Fog Matrix",
        item!(
            "Dark Fog Matrix",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Dark Fog Matrix"))
            ))
        )
    );
    to_hash!(
        res_hash,
        energy_shard,
        "Energy Shard",
        item!(
            "Energy Shard",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Energy Shard"))
            ))
        )
    );
    to_hash!(
        res_hash,
        silicon_based_neuron,
        "Silicon-based Neuron",
        item!(
            "Silicon-based Neuron",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Silicon-based Neuron"))
            ))
        )
    );
    to_hash!(
        res_hash,
        negentropy_singularity,
        "Negentropy Singularity",
        item!(
            "Negentropy Singularity",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Negentropy Singularity"))
            ))
        )
    );
    to_hash!(
        res_hash,
        matter_recombinator,
        "Matter Recombinator",
        item!(
            "Matter Recombinator",
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, "Matter Recombinator"))
            ))
        )
    );
    // processed items
    to_hash!(
        res_hash,
        iron_ingot,
        "Iron Ingot",
        item!(
            "Iron Ingot",
            (recipe!(
                1.0,
                ManFac::Furnace,
                (recitem!(1.0, "Iron Ore")),
                (recitem!(1.0, "Iron Ingot"))
            ))
        )
    );
    to_hash!(
        res_hash,
        copper_ingot,
        "Copper Ingot",
        item!(
            "Copper Ingot",
            (recipe!(
                1.0,
                ManFac::Furnace,
                (recitem!(1.0, "Copper Ore")),
                (recitem!(1.0, "Copper Ingot"))
            ))
        )
    );
    to_hash!(
        res_hash,
        stone_brick,
        "Stone Brick",
        item!(
            "Stone Brick",
            (recipe!(
                1.0,
                ManFac::Furnace,
                (recitem!(1.0, "Stone")),
                (recitem!(1.0, "Stone Brick"))
            ))
        )
    );
    to_hash!(
        res_hash,
        energetic_graphite,
        "Energetic Graphite",
        item!(
            "Energetic Graphite",
            (
                recipe!(
                    2.0,
                    ManFac::Furnace,
                    (recitem!(2.0, "Coal")),
                    (recitem!(1.0, "Energetic Graphite"))
                ),
                recipe!(
                    4.0,
                    ManFac::OilRefinery,
                    (recitem!(2.0, "Hydrogen"), recitem!(1.0, "Refined Oil")),
                    (
                        recitem!(1.0, "Energetic Graphite"),
                        recitem!(3.0, "Hydrogen")
                    )
                )
            )
        )
    );
    to_hash!(
        res_hash,
        high_purity_silicon,
        "High-purity Silicon",
        item!(
            "High-purity Silicon",
            (recipe!(
                2.0,
                ManFac::Furnace,
                (recitem!(2.0, "Silicon Ore")),
                (recitem!(1.0, "High-purity Silicon"))
            ))
        )
    );
    to_hash!(
        res_hash,
        titanium_ingot,
        "Titanium Ingot",
        item!(
            "Titanium Ingot",
            (recipe!(
                2.0,
                ManFac::Furnace,
                (recitem!(2.0, "Titanium Ore")),
                (recitem!(1.0, "Titanium Ingot"))
            ))
        )
    );
    to_hash!(
        res_hash,
        sulfuric_acid,
        "Sulfuric Acid",
        item!(
            "Sulfuric Acid",
            (
                recipe!(
                    6.0,
                    ManFac::ChemicalPlant,
                    (
                        recitem!(4.0, "Water"),
                        recitem!(8.0, "Stone"),
                        recitem!(6.0, "Refined Oil")
                    ),
                    (recitem!(4.0, "Sulfuric Acid"))
                ),
                recipe!(
                    1.2,
                    ManFac::Origin,
                    (IsItem::new_nai()),
                    (recitem!(1.0, "Sulfuric Acid"))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        refined_oil,
        "Refined Oil",
        item!(
            "Refined Oil",
            (
                recipe!(
                    4.0,
                    ManFac::OilRefinery,
                    (recitem!(2.0, "Crude Oil")),
                    (recitem!(2.0, "Refined Oil"), recitem!(1.0, "Hydrogen"))
                ),
                recipe!(
                    4.0,
                    ManFac::OilRefinery,
                    (
                        recitem!(1.0, "Coal"),
                        recitem!(1.0, "Hydrogen"),
                        recitem!(2.0, "Refined Oil")
                    ),
                    (recitem!(3.0, "Refined Oil"))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        magnet,
        "Magnet",
        item!(
            "Magnet",
            (recipe!(
                1.5,
                ManFac::Furnace,
                (recitem!(1.0, "Iron Ore")),
                (recitem!(1.0, "Magnet"))
            ))
        )
    );
    to_hash!(
        res_hash,
        magnetic_coil,
        "Magnetic Coil",
        item!(
            "Magnetic Coil",
            (recipe!(
                1.0,
                ManFac::Assembler,
                (recitem!(1.0, "Copper Ingot"), recitem!(2.0, "Magnet")),
                (recitem!(2.0, "Magnetic Coil"))
            ))
        )
    );
    to_hash!(
        res_hash,
        glass,
        "Glass",
        item!(
            "Glass",
            (recipe!(
                2.0,
                ManFac::Furnace,
                (recitem!(2.0, "Stone")),
                (recitem!(1.0, "Glass"))
            ))
        )
    );
    to_hash!(
        res_hash,
        diamond,
        "Diamond",
        item!(
            "Diamond",
            (
                recipe!(
                    2.0,
                    ManFac::Furnace,
                    (recitem!(1.0, "Energetic Graphite")),
                    (recitem!(1.0, "Diamond"))
                ),
                recipe!(
                    1.5,
                    ManFac::Furnace,
                    (recitem!(1.0, "Kimberlite Ore")),
                    (recitem!(2.0, "Diamond"))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        crystal_silicon,
        "Crystal Silicon",
        item!(
            "Crystal Silicon",
            (
                recipe!(
                    2.0,
                    ManFac::Furnace,
                    (recitem!(1.0, "High-purity Silicon")),
                    (recitem!(1.0, "Crystal Silicon"))
                ),
                recipe!(
                    1.5,
                    ManFac::Furnace,
                    (recitem!(1.0, "Fractal Silicon")),
                    (recitem!(2.0, "Crystal Silicon"))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        steel,
        "Steel",
        item!(
            "Steel",
            (recipe!(
                3.0,
                ManFac::Furnace,
                (recitem!(3.0, "Iron Ingot")),
                (recitem!(1.0, "Steel"))
            ))
        )
    );
    to_hash!(
        res_hash,
        titanium_alloy,
        "Titanium Alloy",
        item!(
            "Titanium Alloy",
            (recipe!(
                12.0,
                ManFac::Furnace,
                (
                    recitem!(8.0, "Sulfuric Acid"),
                    recitem!(4.0, "Steel"),
                    recitem!(4.0, "Titanium Ingot")
                ),
                (recitem!(4.0, "Titanium Alloy"))
            ))
        )
    );
    to_hash!(
        res_hash,
        circuit_board,
        "Circuit Board",
        item!(
            "Circuit Board",
            (recipe!(
                1.0,
                ManFac::Assembler,
                (recitem!(1.0, "Copper Ingot"), recitem!(2.0, "Iron Ingot")),
                (recitem!(2.0, "Circuit Board"))
            ))
        )
    );
    to_hash!(
        res_hash,
        prism,
        "Prism",
        item!(
            "Prism",
            (recipe!(
                2.0,
                ManFac::Furnace,
                (recitem!(3.0, "Glass")),
                (recitem!(2.0, "Prism"))
            ))
        )
    );
    to_hash!(
        res_hash,
        electric_motor,
        "Electric Motor",
        item!(
            "Electric Motor",
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, "Magnetic Coil"),
                    recitem!(1.0, "Gear"),
                    recitem!(2.0, "Iron Ingot")
                ),
                (recitem!(1.0, "Electric Motor"))
            ))
        )
    );
    to_hash!(
        res_hash,
        microcrystalline_component,
        "Microcrystalline Component",
        item!(
            "Microcrystalline Component",
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, "Copper Ingot"),
                    recitem!(2.0, "High-purity Silicon")
                ),
                (recitem!(1.0, "Microcrystalline Component"))
            ))
        )
    );
    to_hash!(
        res_hash,
        gear,
        "Gear",
        item!(
            "Gear",
            (recipe!(
                1.0,
                ManFac::Assembler,
                (recitem!(1.0, "Iron Ingot")),
                (recitem!(1.0, "Gear"))
            ))
        )
    );
    to_hash!(
        res_hash,
        plasma_exciter,
        "Plasma Exciter",
        item!(
            "Plasma Exciter",
            (recipe!(
                2.0,
                ManFac::Assembler,
                (recitem!(2.0, "Prism"), recitem!(4.0, "Magnetic Coil")),
                (recitem!(1.0, "Plasma Exciter"))
            ))
        )
    );
    to_hash!(
        res_hash,
        photon_combiner,
        "Photon Combiner",
        item!(
            "Photon Combiner",
            (
                recipe!(
                    3.0,
                    ManFac::Assembler,
                    (recitem!(1.0, "Circuit Board"), recitem!(2.0, "Prism")),
                    (recitem!(1.0, "Photon Combiner"))
                ),
                recipe!(
                    3.0,
                    ManFac::Assembler,
                    (
                        recitem!(1.0, "Circuit Board"),
                        recitem!(1.0, "Grating Crystal")
                    ),
                    (recitem!(1.0, "Photon Combiner"))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        electromagnetic_turbine,
        "Electromagnetic Turbine",
        item!(
            "Electromagnetic Turbine",
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, "Magnetic Coil"),
                    recitem!(2.0, "Electric Motor")
                ),
                (recitem!(1.0, "Electromagnetic Turbine"))
            ))
        )
    );
    to_hash!(
        res_hash,
        processor,
        "Processor",
        item!(
            "Processor",
            (recipe!(
                3.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, "Microcrystalline Component"),
                    recitem!(2.0, "Circuit Board")
                ),
                (recitem!(1.0, "Processor"))
            ))
        )
    );
    to_hash!(
        res_hash,
        engine,
        "Engine",
        item!(
            "Engine",
            (recipe!(
                3.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, "Copper Ingot"),
                    recitem!(1.0, "Magnetic Coil")
                ),
                (recitem!(1.0, "Engine"))
            ))
        )
    );
    to_hash!(
        res_hash,
        thruster,
        "Thruster",
        item!(
            "Thruster",
            (recipe!(
                4.0,
                ManFac::Assembler,
                (recitem!(3.0, "Copper Ingot")),
                (recitem!(2.0, "Steel"))
            ))
        )
    );
    to_hash!(
        res_hash,
        reinforced_thruster,
        "Reinforced Thruster",
        item!(
            "Reinforced Thruster",
            (recipe!(
                6.0,
                ManFac::Assembler,
                (
                    recitem!(5.0, "Electromagnetic Turbine"),
                    recitem!(5.0, "Titanium Alloy")
                ),
                (recitem!(1.0, "Reinforced Thruster"))
            ))
        )
    );
    to_hash!(
        res_hash,
        super_magnetic_ring,
        "Super-magnetic Ring",
        item!(
            "Super-magnetic Ring",
            (recipe!(
                3.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, "Energetic Graphite"),
                    recitem!(3.0, "Magnet"),
                    recitem!(2.0, "Electromagnetic Turbine")
                ),
                (recitem!(1.0, "Super-magnetic Ring"))
            ))
        )
    );
    to_hash!(
        res_hash,
        particle_container,
        "Particle Container",
        item!(
            "Particle Container",
            (
                recipe!(
                    4.0,
                    ManFac::Assembler,
                    (
                        recitem!(2.0, "Graphene"),
                        recitem!(2.0, "Copper Ingot"),
                        recitem!(2.0, "Electromagnetic Turbine")
                    ),
                    (recitem!(1.0, "Particle Container"))
                ),
                recipe!(
                    4.0,
                    ManFac::Assembler,
                    (
                        recitem!(2.0, "Copper Ingot"),
                        recitem!(10.0, "Unipolar Magnet")
                    ),
                    (recitem!(1.0, "Particle Container"))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        plastic,
        "Plastic",
        item!(
            "Plastic",
            (recipe!(
                3.0,
                ManFac::ChemicalPlant,
                (
                    recitem!(1.0, "Energetic Graphite"),
                    recitem!(2.0, "Refined Oil")
                ),
                (recitem!(1.0, "Plastic"))
            ))
        )
    );
    to_hash!(
        res_hash,
        organic_crystal,
        "Organic Crystal",
        item!(
            "Organic Crystal",
            (
                recipe!(
                    6.0,
                    ManFac::ChemicalPlant,
                    (
                        recitem!(1.0, "Water"),
                        recitem!(1.0, "Refined Oil"),
                        recitem!(2.0, "Plastic")
                    ),
                    (recitem!(1.0, "Organic Crystal"))
                ),
                recipe!(
                    6.0,
                    ManFac::ChemicalPlant,
                    (
                        recitem!(10.0, "Water"),
                        recitem!(30.0, "Plant Fuel"),
                        recitem!(20.0, "Log"),
                    ),
                    (recitem!(1.0, "Organic Crystal"))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        graphene,
        "Graphene",
        item!(
            "Graphene",
            (
                recipe!(
                    3.0,
                    ManFac::ChemicalPlant,
                    (
                        recitem!(1.0, "Sulfuric Acid"),
                        recitem!(3.0, "Energetic Graphite")
                    ),
                    (recitem!(2.0, "Graphene"))
                ),
                recipe!(
                    2.0,
                    ManFac::ChemicalPlant,
                    (recitem!(2.0, "Fire Ice")),
                    (recitem!(1.0, "Hydrogen"), recitem!(2.0, "Graphene"))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        annihilation_constraint_sphere,
        "Annihilation Constraint Sphere",
        item!(
            "Annihilation Constraint Sphere",
            (recipe!(
                20.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, "Processor"),
                    recitem!(1.0, "Particle Container")
                ),
                (recitem!(1.0, "Annihilation Constraint Sphere"))
            ))
        )
    );
    to_hash!(
        res_hash,
        strange_matter,
        "Strange Matter",
        item!(
            "Strange Matter",
            (recipe!(
                8.0,
                ManFac::MiniatureParticleCollider,
                (
                    recitem!(10.0, "Deuterium"),
                    recitem!(2.0, "Iron Ingot"),
                    recitem!(2.0, "Particle Container")
                ),
                (recitem!(1.0, "Strange Matter"))
            ))
        )
    );
    to_hash!(
        res_hash,
        titanium_crystal,
        "Titanium Crystal",
        item!(
            "Titanium Crystal",
            (recipe!(
                4.0,
                ManFac::Assembler,
                (
                    recitem!(3.0, "Titanium Ingot"),
                    recitem!(1.0, "Organic Crystal")
                ),
                (recitem!(1.0, "Titanium Crystal"))
            ))
        )
    );
    to_hash!(
        res_hash,
        carbon_nanotube,
        "Carbon Nanotube",
        item!(
            "Carbon Nanotube",
            (
                recipe!(
                    4.0,
                    ManFac::ChemicalPlant,
                    (recitem!(1.0, "Titanium Ingot"), recitem!(3.0, "Graphene")),
                    (recitem!(2.0, "Carbon Nanotube"))
                ),
                recipe!(
                    4.0,
                    ManFac::ChemicalPlant,
                    (recitem!(6.0, "Stalagmite Crystal")),
                    (recitem!(2.0, "Carbon Nanotube"))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        particle_broadband,
        "Particle Broadband",
        item!(
            "Particle Broadband",
            (recipe!(
                8.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, "Plastic"),
                    recitem!(2.0, "Crystal Silicon"),
                    recitem!(2.0, "Carbon Nanotube")
                ),
                (recitem!(1.0, "Particle Broadband"))
            ))
        )
    );
    to_hash!(
        res_hash,
        casimir_crystal,
        "Casimir Crystal",
        item!(
            "Casimir Crystal",
            (
                recipe!(
                    4.0,
                    ManFac::Assembler,
                    (
                        recitem!(12.0, "Hydrogen"),
                        recitem!(2.0, "Graphene"),
                        recitem!(1.0, "Titanium Crystal")
                    ),
                    (recitem!(1.0, "Casimir Crystal"))
                ),
                recipe!(
                    4.0,
                    ManFac::Assembler,
                    (
                        recitem!(12.0, "Hydrogen"),
                        recitem!(2.0, "Graphene"),
                        recitem!(8.0, "Grating Crystal")
                    ),
                    (recitem!(1.0, "Casimir Crystal"))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        titanium_glass,
        "Titanium Glass",
        item!(
            "Titanium Glass",
            (recipe!(
                5.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, "Water"),
                    recitem!(2.0, "Titanium Ingot"),
                    recitem!(2.0, "Glass")
                ),
                (recitem!(2.0, "Titanium Glass"))
            ))
        )
    );
    to_hash!(
        res_hash,
        plane_filter,
        "Plane Filter",
        item!(
            "Plane Filter",
            (recipe!(
                12.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, "Titanium Glass"),
                    recitem!(1.0, "Casimir Crystal")
                ),
                (recitem!(1.0, "Plane Filter"))
            ))
        )
    );
    to_hash!(
        res_hash,
        quantum_chip,
        "Quantum Chip",
        item!(
            "Quantum Chip",
            (recipe!(
                6.0,
                ManFac::Assembler,
                (recitem!(2.0, "Plane Filter"), recitem!(2.0, "Processor")),
                (recitem!(1.0, "Quantum Chip"))
            ))
        )
    );

    to_hash!(
        res_hash,
        combustible_unit,
        "Combustible Unit",
        item!(
            "Combustible Unit",
            (recipe!(
                3.0,
                ManFac::Assembler,
                (recitem!(3.0, "Coal")),
                (recitem!(1.0, "Combustible Unit"))
            ))
        )
    );
    // logistics
    to_hash!(
        res_hash,
        logistics_bot,
        "Logistics Bot",
        item!(
            "Logistics Bot",
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, "Processor"),
                    recitem!(1.0, "Engine"),
                    recitem!(2.0, "Iron Ingot")
                ),
                (recitem!(1.0, "Logistics Bot"))
            ))
        )
    );
    to_hash!(
        res_hash,
        logistics_drone,
        "Logistics Drone",
        item!(
            "Logistics Drone",
            (recipe!(
                4.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, "Thruster"),
                    recitem!(2.0, "Processor"),
                    recitem!(5.0, "Iron Ingot")
                ),
                (recitem!(1.0, "Logistics Drone"))
            ))
        )
    );
    to_hash!(
        res_hash,
        interstellar_logistics_vessel,
        "Interstellar Logistics Vessel",
        item!(
            "Interstellar Logistics Vessel",
            (recipe!(
                6.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, "Reinforced Thruster"),
                    recitem!(10.0, "Processor"),
                    recitem!(10.0, "Titanium Alloy")
                ),
                (recitem!(1.0, "Interstellar Logistics Vessel"))
            ))
        )
    );
    to_hash!(
        res_hash,
        graviton_lens,
        "Graviton Lens",
        item!(
            "Graviton Lens",
            (recipe!(
                6.0,
                ManFac::Assembler,
                (recitem!(1.0, "Strange Matter"), recitem!(4.0, "Diamond")),
                (recitem!(1.0, "Graviton Lens"))
            ))
        )
    );
    to_hash!(
        res_hash,
        space_warper,
        "Space Warper",
        item!(
            "Space Warper",
            (
                recipe!(
                    10.0,
                    ManFac::Assembler,
                    (recitem!(1.0, "Graviton Lens")),
                    (recitem!(1.0, "Space Warper"))
                ),
                recipe!(
                    10.0,
                    ManFac::Assembler,
                    (recitem!(1.0, "Gravity Matrix")),
                    (recitem!(8.0, "Space Warper"))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        foundation,
        "Foundation",
        item!(
            "Foundation",
            (recipe!(
                1.0,
                ManFac::Assembler,
                (recitem!(1.0, "Steel"), recitem!(3.0, "Stone Brick")),
                (recitem!(1.0, "Foundation"))
            ))
        )
    );
    // proliferator
    to_hash!(
        res_hash,
        proliferator_mki,
        "Proliferator Mk.I",
        item!(
            "Proliferator Mk.I",
            (recipe!(
                0.5,
                ManFac::Assembler,
                (recitem!(1.0, "Coal")),
                (recitem!(1.0, "Proliferator Mk.I"))
            ))
        )
    );
    to_hash!(
        res_hash,
        proliferator_mkii,
        "Proliferator Mk.II",
        item!(
            "Proliferator MK.II",
            (recipe!(
                1.0,
                ManFac::Assembler,
                (recitem!(1.0, "Diamond"), recitem!(2.0, "Proliferator Mk.I")),
                (recitem!(1.0, "Proliferator Mk.II"))
            ))
        )
    );
    to_hash!(
        res_hash,
        proliferator_mkiii,
        "Proliferator Mk.III",
        item!(
            "Proliferator Mk.III",
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, "Carbon Nanotube"),
                    recitem!(2.0, "Proliferator Mk.II")
                ),
                (recitem!(1.0, "Proliferator Mk.III"))
            ))
        )
    );
    // fuel rods
    to_hash!(
        res_hash,
        hydrogen_fuel_rod,
        "Hydrogen Fuel Rod",
        item!(
            "Hydrogen Fuel Rod",
            (recipe!(
                6.0,
                ManFac::Assembler,
                (recitem!(10.0, "Hydrogen"), recitem!(1.0, "Titanium Ingot")),
                (recitem!(2.0, "Hydrogen Fuel Rod"))
            ))
        )
    );
    to_hash!(
        res_hash,
        deuterium_fuel_rod,
        "Deuterium Fuel Rod",
        item!(
            "Deuterium Fuel Rod",
            (recipe!(
                12.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, "Super-magnetic Ring"),
                    recitem!(20.0, "Deuterium"),
                    recitem!(1.0, "Titanium Alloy")
                ),
                (recitem!(2.0, "Deuterium Fuel Rod"))
            ))
        )
    );
    to_hash!(
        res_hash,
        antimatter_fuel_rod,
        "Antimatter Fuel Rod",
        item!(
            "Antimatter Fuel Rod",
            (recipe!(
                24.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, "Titanium Alloy"),
                    recitem!(1.0, "Annihilation Constraint Sphere"),
                    recitem!(12.0, "Hydrogen"),
                    recitem!(12.0, "Antimatter")
                ),
                (recitem!(2.0, "Antimatter Fuel Rod"))
            ))
        )
    );
    to_hash!(
        res_hash,
        strange_annihilation_fuel_rod,
        "Strange Annihilation Fuel Rod",
        item!(
            "Strange Annihilation Fuel Rod",
            (recipe!(
                32.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, "Frame Material"),
                    recitem!(2.0, "Strange Matter"),
                    recitem!(1.0, "Core Element"),
                    recitem!(8.0, "Antimatter Fuel Rod")
                ),
                (recitem!(1.0, "Strange Annihilation Fuel Rod"))
            ))
        )
    );
    // dyson sphere building
    to_hash!(
        res_hash,
        solar_sail,
        "Solar Sail",
        item!(
            "Solar Sail",
            (recipe!(
                4.0,
                ManFac::Assembler,
                (recitem!(1.0, "Photon Combiner"), recitem!(1.0, "Graphene")),
                (recitem!(2.0, "Solar Sail"))
            ))
        )
    );
    to_hash!(
        res_hash,
        frame_material,
        "Frame Material",
        item!(
            "Frame Material",
            (recipe!(
                6.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, "High-purity Silicon"),
                    recitem!(1.0, "Titanium Alloy"),
                    recitem!(4.0, "Carbon Nanotube")
                ),
                (recitem!(1.0, "Frame Material"))
            ))
        )
    );
    to_hash!(
        res_hash,
        dyson_sphere_component,
        "Dyson Sphere Component",
        item!(
            "Dyson Sphere Component",
            (recipe!(
                8.0,
                ManFac::Assembler,
                (
                    recitem!(3.0, "Processor"),
                    recitem!(3.0, "Solar Sail"),
                    recitem!(3.0, "Frame Material")
                ),
                (recitem!(1.0, "Dyson Sphere Component"))
            ))
        )
    );
    to_hash!(
        res_hash,
        small_carrier_rocket,
        "Small Carrier Rocket",
        item!(
            "Small Carrier Rocket",
            (recipe!(
                6.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, "Quantum Chip"),
                    recitem!(4.0, "Deuterium Fuel Rod"),
                    recitem!(2.0, "Dyson Sphere Component")
                ),
                (recitem!(1.0, "Small Carrier Rocket"))
            ))
        )
    );
    // Matrices
    to_hash!(
        res_hash,
        electromagnetic_matrix,
        "Electromagnetic Matrix",
        item!(
            "Electromagnetic Matrix",
            (recipe!(
                3.0,
                ManFac::Lab,
                (
                    recitem!(1.0, "Circuit Board"),
                    recitem!(1.0, "Magnetic Coil")
                ),
                (recitem!(1.0, "Electromagnetic Matrix"))
            ))
        )
    );
    to_hash!(
        res_hash,
        energy_matrix,
        "Energy Matrix",
        item!(
            "Energy Matrix",
            (recipe!(
                6.0,
                ManFac::Lab,
                (
                    recitem!(2.0, "Hydrogen"),
                    recitem!(2.0, "Energetic Graphite")
                ),
                (recitem!(1.0, "Energy Matrix"))
            ))
        )
    );
    to_hash!(
        res_hash,
        structure_matrix,
        "Structure Matrix",
        item!(
            "Structure Matrix",
            (recipe!(
                8.0,
                ManFac::Lab,
                (recitem!(1.0, "Titanium Crystal"), recitem!(1.0, "Diamond")),
                (recitem!(1.0, "Structure Matrix"))
            ))
        )
    );
    to_hash!(
        res_hash,
        information_matrix,
        "Information Matrix",
        item!(
            "Information Matrix",
            (recipe!(
                10.0,
                ManFac::Lab,
                (
                    recitem!(1.0, "Particle Broadband"),
                    recitem!(2.0, "Processor")
                ),
                (recitem!(1.0, "Information Matrix"))
            ))
        )
    );
    to_hash!(
        res_hash,
        gravity_matrix,
        "Gravity Matrix",
        item!(
            "Gravity Matrix",
            (recipe!(
                24.0,
                ManFac::Lab,
                (
                    recitem!(1.0, "Quantum Chip"),
                    recitem!(1.0, "Graviton Lens")
                ),
                (recitem!(2.0, "Gravity Matrix"))
            ))
        )
    );
    to_hash!(
        res_hash,
        universe_matrix,
        "Universe Matrix",
        item!(
            "Universe Matrix",
            (recipe!(
                15.0,
                ManFac::Lab,
                (
                    recitem!(1.0, "Electromagnetic Matrix"),
                    recitem!(1.0, "Energy Matrix"),
                    recitem!(1.0, "Structure Matrix"),
                    recitem!(1.0, "Information Matrix"),
                    recitem!(1.0, "Gravity Matrix"),
                    recitem!(1.0, "Antimatter")
                ),
                (recitem!(1.0, "Universe Matrix"))
            ))
        )
    );
    // ammunition
    to_hash!(
        res_hash,
        magnum_ammo_box,
        "Magnum Ammo Box",
        item!(
            "Magnum Ammo Box",
            (recipe!(
                1.0,
                ManFac::Assembler,
                (recitem!(3.0, "Copper Ingot")),
                (recitem!(1.0, "Magnum Ammo Box"))
            ))
        )
    );
    to_hash!(
        res_hash,
        titanium_ammo_box,
        "Titanium Ammo Box",
        item!(
            "Titanium Ammo Box",
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, "Titanium Ingot"),
                    recitem!(1.0, "Magnum Ammo Box")
                ),
                (recitem!(1.0, "Titanium Ammo Box"))
            ))
        )
    );
    to_hash!(
        res_hash,
        superalloy_ammo_box,
        "Superalloy Ammo Box",
        item!(
            "Superalloy Ammo Box",
            (recipe!(
                3.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, "Titanium Alloy"),
                    recitem!(1.0, "Titanium Ammo Box")
                ),
                (recitem!(1.0, "Superalloy Ammo Box"))
            ))
        )
    );
    to_hash!(
        res_hash,
        explosive_unit,
        "Explosive Unit",
        item!(
            "Explosive Unit",
            (recipe!(
                6.0,
                ManFac::ChemicalPlant,
                (
                    recitem!(1.0, "Sulfuric Acid"),
                    recitem!(2.0, "Plastic"),
                    recitem!(2.0, "Combustible Unit")
                ),
                (recitem!(2.0, "Explosive Unit"))
            ))
        )
    );
    to_hash!(
        res_hash,
        crystal_explosive_unit,
        "Crystal Explosive Unit",
        item!(
            "Crystal Explosive Unit",
            (recipe!(
                24.0,
                ManFac::ChemicalPlant,
                (
                    recitem!(8.0, "Crystal Silicon"),
                    recitem!(1.0, "Casimir Crystal"),
                    recitem!(8.0, "Explosive Unit")
                ),
                (recitem!(8.0, "Crystal Explosive Unit"))
            ))
        )
    );
    to_hash!(
        res_hash,
        missile_set,
        "Missile Set",
        item!(
            "Missile Set",
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, "Engine"),
                    recitem!(2.0, "Combustible Unit"),
                    recitem!(3.0, "Circuit Board"),
                    recitem!(6.0, "Copper Ingot")
                ),
                (recitem!(1.0, "Missile Set"))
            ))
        )
    );
    to_hash!(
        res_hash,
        supersonic_missile_set,
        "Supersonic Missile Set",
        item!(
            "Supersonic Missile Set",
            (recipe!(
                4.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, "Thruster"),
                    recitem!(2.0, "Explosive Unit"),
                    recitem!(4.0, "Processor"),
                    recitem!(2.0, "Missile Set")
                ),
                (recitem!(2.0, "Supersonic Missile Set"))
            ))
        )
    );
    to_hash!(
        res_hash,
        gravity_missile_set,
        "Gravity Missile Set",
        item!(
            "Gravity Missile Set",
            (recipe!(
                6.0,
                ManFac::Assembler,
                (
                    recitem!(3.0, "Strange Matter"),
                    recitem!(6.0, "Crystal Explosive Unit"),
                    recitem!(3.0, "Supersonic Missile Set")
                ),
                (recitem!(3.0, "Gravity Missile Set"))
            ))
        )
    );
    to_hash!(
        res_hash,
        shell_set,
        "Shell Set",
        item!(
            "Shell Set",
            (recipe!(
                1.5,
                ManFac::Assembler,
                (
                    recitem!(2.0, "Explosive Unit"),
                    recitem!(9.0, "Copper Ingot")
                ),
                (recitem!(1.0, "Shell Set"))
            ))
        )
    );
    to_hash!(
        res_hash,
        high_explosive_shell_set,
        "High-Explosive Shell Set",
        item!(
            "High-Explosive Shell Set",
            (recipe!(
                3.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, "Explosive Unit"),
                    recitem!(6.0, "Titanium Ingot"),
                    recitem!(1.0, "Shell Set")
                ),
                (recitem!(1.0, "High-Explosive Shell Set"))
            ))
        )
    );
    to_hash!(
        res_hash,
        crystal_shell_set,
        "Crystal Shell Set",
        item!(
            "Crystal Shell Set",
            (recipe!(
                6.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, "Crystal Explosive Unit"),
                    recitem!(3.0, "Titanium Ingot"),
                    recitem!(1.0, "High-Explosive Shell Set")
                ),
                (recitem!(1.0, "Crystal Shell Set"))
            ))
        )
    );
    to_hash!(
        res_hash,
        plasma_capsule,
        "Plasma Capsule",
        item!(
            "Plasma Capsule",
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(10.0, "Deuterium"),
                    recitem!(2.0, "Magnet"),
                    recitem!(1.0, "Graphene")
                ),
                (recitem!(1.0, "Plasma Capsule"))
            ))
        )
    );
    to_hash!(
        res_hash,
        antimatter_capsule,
        "Antimatter Capsule",
        item!(
            "Antimatter Capsule",
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(10.0, "Antimatter"),
                    recitem!(10.0, "Hydrogen"),
                    recitem!(1.0, "Particle Container"),
                    recitem!(1.0, "Plasma Capsule")
                ),
                (recitem!(1.0, "Antimatter Capsule"))
            ))
        )
    );
    to_hash!(
        res_hash,
        jamming_capsule,
        "Jamming Capsule",
        item!(
            "Jamming Capsule",
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(3.0, "Hydrogen"),
                    recitem!(1.0, "Plasma Exciter"),
                    recitem!(1.0, "Electromagnetic Turbine")
                ),
                (recitem!(1.0, "Jamming Capsule"))
            ))
        )
    );
    to_hash!(
        res_hash,
        suppressing_capsule,
        "Suppressing Capsule",
        item!(
            "Suppressing Capsule",
            (recipe!(
                8.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, "Titanium Glass"),
                    recitem!(1.0, "Super-magnetic Ring"),
                    recitem!(2.0, "Jamming Capsule")
                ),
                (recitem!(2.0, "Suppressing Capsule"))
            ))
        )
    );
    // drones and ships
    to_hash!(
        res_hash,
        prototype,
        "Prototype",
        item!(
            "Prototype",
            (recipe!(
                3.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, "Plasma Exciter"),
                    recitem!(2.0, "Circuit Board"),
                    recitem!(1.0, "Engine"),
                    recitem!(3.0, "Iron Ingot")
                ),
                (recitem!(1.0, "Prototype"))
            ))
        )
    );
    to_hash!(
        res_hash,
        precision_drone,
        "Precision Drone",
        item!(
            "Precision Drone",
            (recipe!(
                4.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, "Photon Combiner"),
                    recitem!(2.0, "Circuit Board"),
                    recitem!(1.0, "Electromagnetic Turbine"),
                    recitem!(1.0, "Prototype")
                ),
                (recitem!(1.0, "Precision Drone"))
            ))
        )
    );
    to_hash!(
        res_hash,
        attack_drone,
        "Attack Drone",
        item!(
            "Attack Drone",
            (recipe!(
                4.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, "Particle Container"),
                    recitem!(1.0, "Processor"),
                    recitem!(1.0, "Electromagnetic Turbine"),
                    recitem!(1.0, "Prototype")
                ),
                (recitem!(1.0, "Attack Drone"))
            ))
        )
    );
    to_hash!(
        res_hash,
        corvette,
        "Corvette",
        item!(
            "Corvette",
            (recipe!(
                5.0,
                ManFac::Assembler,
                (
                    recitem!(3.0, "Particle Container"),
                    recitem!(2.0, "Processor"),
                    recitem!(1.0, "Reinforced Thruster"),
                    recitem!(5.0, "Titanium Alloy")
                ),
                (recitem!(1.0, "Corvette"))
            ))
        )
    );
    to_hash!(
        res_hash,
        destroyer,
        "Destroyer",
        item!(
            "Destroyer",
            (recipe!(
                8.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, "Strange Matter"),
                    recitem!(4.0, "Processor"),
                    recitem!(4.0, "Reinforced Thruster"),
                    recitem!(20.0, "Frame Material")
                ),
                (recitem!(1.0, "Destroyer"))
            ))
        )
    );
    // return the hashmap with all the items
    res_hash
}

#[cfg(test)]
mod test_items {
    // import HashMaps
    use crate::beta::essentials::*;

    #[test]
    fn test_hashmap() {
        // get list of all items
        let item_map = super::get_items();
        // get all keys
        let keys = item_map.keys();
        // access all items in HashMap
        for key in keys {
            // save current item from HashMap
            let current_item = match item_map.get_key_value(key) {
                Some(value_key) => value_key.1,
                None => panic!(),
            };
            let all_recipes = current_item.recipes.clone();
            // go through all recipes
            for recipe in all_recipes.iter() {
                // ingredients
                let ingredients = recipe.ingredients.clone();
                for ingredient in ingredients {
                    let is_item = ingredient;
                    let item = match is_item {
                        IsItem::Item(item_amount) => item_amount.item,
                        IsItem::Nai => continue,
                    };
                    // check whether or not the string can be found as key in the
                    // item hashmap
                    let item_result = item_map.get_key_value(&item);
                    // process the result
                    match item_result {
                        // Item exists String was typed correctly
                        Some(_result) => continue,
                        // string doesn't exist in hashmap
                        None => panic!(
                            "ingredient '{}' of item '{:?}' doesn't exist in HashMap",
                            item, recipe
                        ),
                    }
                }
                // results
                let products = recipe.products.clone();
                for product in products {
                    let is_item = product;
                    let item = match is_item {
                        IsItem::Item(item_amount) => {
                            println!("{:?}", item_amount);
                            item_amount.item
                        }
                        IsItem::Nai => continue,
                    };
                    // check whether or not the string can be found as key in the
                    // item hashmap
                    let item_result = item_map.get_key_value(&item);
                    // process the result
                    match item_result {
                        // Item exists String was typed correctly
                        Some(_result) => continue,
                        // string doesn't exist in hashmap
                        None => panic!(
                            "product '{}' of item '{:?}' doesn't exist in HashMap",
                            item, recipe
                        ),
                    }
                }
            }
        }
    }
}

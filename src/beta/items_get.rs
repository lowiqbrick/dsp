// all crafting recipe numbers are from the [wiki](https://dsp-wiki.com/Items) are

// import hashmaps
use std::collections::HashMap;
// import the structs, emuns, etc.
use crate::beta::essentials::{IsItem, Item, ItemAmount, ManFac, Recipe};
use crate::beta::items::ItemType;
// import all macros
use crate::{item, recipe, recitem, to_hash};

/// creates a big HashMap with all the items and buildings in the game
pub fn get_items() -> HashMap<ItemType, Item> {
    let mut res_hash = HashMap::new();
    /*
    macro template explanation
    to_hash!(
       res_hash,
        <variable name>,
        <item type>,
        item!(
            <item type>,
            (recipe!(
                <seconds to craft>,
                <Manufacturing facility>,
                    (recitem!(<Amount Item>, <ingredient item type>)),
                    (recitem!(<Amount Item>, <product item type>))
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
        ItemType::IronOre,
        item!(
            ItemType::IronOre,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::IronOre))
            ))
        )
    );
    to_hash!(
        res_hash,
        copper_ore,
        ItemType::CopperOre,
        item!(
            ItemType::CopperOre,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::CopperOre))
            ))
        )
    );
    to_hash!(
        res_hash,
        stone,
        ItemType::Stone,
        item!(
            ItemType::Stone,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::Stone))
            ))
        )
    );
    to_hash!(
        res_hash,
        coal,
        ItemType::Coal,
        item!(
            ItemType::Coal,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::Coal))
            ))
        )
    );
    to_hash!(
        res_hash,
        silicon_ore,
        ItemType::SiliconOre,
        item!(
            ItemType::SiliconOre,
            (
                recipe!(
                    0.0,
                    ManFac::Origin,
                    (IsItem::new_nai()),
                    (recitem!(1.0, ItemType::SiliconOre))
                ),
                recipe!(
                    10.0,
                    ManFac::Furnace,
                    (recitem!(10.0, ItemType::Stone)),
                    (recitem!(1.0, ItemType::SiliconOre))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        titanium_ore,
        ItemType::TitaniumOre,
        item!(
            ItemType::TitaniumOre,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::TitaniumOre))
            ))
        )
    );
    to_hash!(
        res_hash,
        water,
        ItemType::Water,
        item!(
            ItemType::Water,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::Water))
            ))
        )
    );
    to_hash!(
        res_hash,
        crude_oil,
        ItemType::CrudeOil,
        item!(
            ItemType::CrudeOil,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::CrudeOil))
            ))
        )
    );
    // hydrogen requires a lot of other items to be declared
    to_hash!(
        res_hash,
        hydrogen,
        ItemType::Hydrogen,
        item!(
            ItemType::Hydrogen,
            (
                recipe!(
                    4.0,
                    ManFac::OilRefinery,
                    (recitem!(2.0, ItemType::CrudeOil)),
                    (
                        recitem!(2.0, ItemType::RefinedOil),
                        recitem!(1.0, ItemType::Hydrogen)
                    )
                ),
                recipe!(
                    2.0,
                    ManFac::ChemicalPlant,
                    (recitem!(2.0, ItemType::FireIce)),
                    (
                        recitem!(1.0, ItemType::Hydrogen),
                        recitem!(2.0, ItemType::Graphene)
                    )
                ),
                recipe!(
                    4.0,
                    ManFac::OilRefinery,
                    (
                        recitem!(2.0, ItemType::Hydrogen),
                        recitem!(1.0, ItemType::RefinedOil)
                    ),
                    (
                        recitem!(1.0, ItemType::EnergeticGraphite),
                        recitem!(3.0, ItemType::Hydrogen)
                    )
                ),
                recipe!(
                    2.0,
                    ManFac::MiniatureParticleCollider,
                    (recitem!(2.0, ItemType::CriticalPhoton)),
                    (
                        recitem!(2.0, ItemType::Hydrogen),
                        recitem!(2.0, ItemType::Antimatter)
                    )
                )
            )
        )
    );
    to_hash!(
        res_hash,
        deuterium,
        ItemType::Deuterium,
        item!(
            ItemType::Deuterium,
            (
                recipe!(
                    0.0,
                    ManFac::Origin,
                    (IsItem::new_nai()),
                    (recitem!(1.0, ItemType::Deuterium))
                ),
                recipe!(
                    2.5,
                    ManFac::MiniatureParticleCollider,
                    (recitem!(10.0, ItemType::Hydrogen)),
                    (recitem!(5.0, ItemType::Deuterium))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        antimatter,
        ItemType::Antimatter,
        item!(
            ItemType::Antimatter,
            (recipe!(
                2.0,
                ManFac::MiniatureParticleCollider,
                (recitem!(2.0, ItemType::CriticalPhoton)),
                (
                    recitem!(2.0, ItemType::Hydrogen),
                    recitem!(2.0, ItemType::Antimatter)
                )
            ))
        )
    );
    to_hash!(
        res_hash,
        core_element,
        ItemType::CoreElement,
        item!(
            ItemType::CoreElement,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::CoreElement))
            ))
        )
    );
    to_hash!(
        res_hash,
        critical_photon,
        ItemType::CriticalPhoton,
        item!(
            ItemType::CriticalPhoton,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::CriticalPhoton))
            ))
        )
    );
    to_hash!(
        res_hash,
        kimberlite_ore,
        ItemType::KimberliteOre,
        item!(
            ItemType::KimberliteOre,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::KimberliteOre))
            ))
        )
    );
    to_hash!(
        res_hash,
        fractal_silicon,
        ItemType::FractalSilicon,
        item!(
            ItemType::FractalSilicon,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::FractalSilicon))
            ))
        )
    );
    to_hash!(
        res_hash,
        grating_crystal,
        ItemType::GratingCrystal,
        item!(
            ItemType::GratingCrystal,
            (recipe!(
                1.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::GratingCrystal))
            ))
        )
    );
    to_hash!(
        res_hash,
        stalagmite_crystal,
        ItemType::StalagmiteCrystal,
        item!(
            ItemType::StalagmiteCrystal,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::StalagmiteCrystal))
            ))
        )
    );
    to_hash!(
        res_hash,
        unipolar_magnet,
        ItemType::UnipolarMagnet,
        item!(
            ItemType::UnipolarMagnet,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::UnipolarMagnet))
            ))
        )
    );
    to_hash!(
        res_hash,
        fire_ice,
        ItemType::FireIce,
        item!(
            ItemType::FireIce,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::FireIce))
            ))
        )
    );
    to_hash!(
        res_hash,
        log,
        ItemType::Log,
        item!(
            ItemType::Log,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::Log))
            ))
        )
    );
    to_hash!(
        res_hash,
        plant_fuel,
        ItemType::PlantFuel,
        item!(
            ItemType::PlantFuel,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::PlantFuel))
            ))
        )
    );
    to_hash!(
        res_hash,
        dark_fog_matrix,
        ItemType::DarkFogMatrix,
        item!(
            ItemType::DarkFogMatrix,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::DarkFogMatrix))
            ))
        )
    );
    to_hash!(
        res_hash,
        energy_shard,
        ItemType::EnergyShard,
        item!(
            ItemType::EnergyShard,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::EnergyShard))
            ))
        )
    );
    to_hash!(
        res_hash,
        silicon_based_neuron,
        ItemType::SiliconBasedNeuron,
        item!(
            ItemType::SiliconBasedNeuron,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::SiliconBasedNeuron))
            ))
        )
    );
    to_hash!(
        res_hash,
        negentropy_singularity,
        ItemType::NegentropySingularity,
        item!(
            ItemType::NegentropySingularity,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::NegentropySingularity))
            ))
        )
    );
    to_hash!(
        res_hash,
        matter_recombinator,
        ItemType::MatterRecombinator,
        item!(
            ItemType::MatterRecombinator,
            (recipe!(
                0.0,
                ManFac::Origin,
                (IsItem::new_nai()),
                (recitem!(1.0, ItemType::MatterRecombinator))
            ))
        )
    );
    // processed items
    to_hash!(
        res_hash,
        iron_ingot,
        ItemType::IronIngot,
        item!(
            ItemType::IronIngot,
            (recipe!(
                1.0,
                ManFac::Furnace,
                (recitem!(1.0, ItemType::IronOre)),
                (recitem!(1.0, ItemType::IronIngot))
            ))
        )
    );
    to_hash!(
        res_hash,
        copper_ingot,
        ItemType::CopperIngot,
        item!(
            ItemType::CopperIngot,
            (recipe!(
                1.0,
                ManFac::Furnace,
                (recitem!(1.0, ItemType::CopperOre)),
                (recitem!(1.0, ItemType::CopperIngot))
            ))
        )
    );
    to_hash!(
        res_hash,
        stone_brick,
        ItemType::StoneBrick,
        item!(
            ItemType::StoneBrick,
            (recipe!(
                1.0,
                ManFac::Furnace,
                (recitem!(1.0, ItemType::Stone)),
                (recitem!(1.0, ItemType::StoneBrick))
            ))
        )
    );
    to_hash!(
        res_hash,
        energetic_graphite,
        ItemType::EnergeticGraphite,
        item!(
            ItemType::EnergeticGraphite,
            (
                recipe!(
                    2.0,
                    ManFac::Furnace,
                    (recitem!(2.0, ItemType::Coal)),
                    (recitem!(1.0, ItemType::EnergeticGraphite))
                ),
                recipe!(
                    4.0,
                    ManFac::OilRefinery,
                    (
                        recitem!(2.0, ItemType::Hydrogen),
                        recitem!(1.0, ItemType::RefinedOil)
                    ),
                    (
                        recitem!(1.0, ItemType::EnergeticGraphite),
                        recitem!(3.0, ItemType::Hydrogen)
                    )
                )
            )
        )
    );
    to_hash!(
        res_hash,
        high_purity_silicon,
        ItemType::HighPuritySilicon,
        item!(
            ItemType::HighPuritySilicon,
            (recipe!(
                2.0,
                ManFac::Furnace,
                (recitem!(2.0, ItemType::SiliconOre)),
                (recitem!(1.0, ItemType::HighPuritySilicon))
            ))
        )
    );
    to_hash!(
        res_hash,
        titanium_ingot,
        ItemType::TitaniumIngot,
        item!(
            ItemType::TitaniumIngot,
            (recipe!(
                2.0,
                ManFac::Furnace,
                (recitem!(2.0, ItemType::TitaniumOre)),
                (recitem!(1.0, ItemType::TitaniumIngot))
            ))
        )
    );
    to_hash!(
        res_hash,
        sulfuric_acid,
        ItemType::SulfuricAcid,
        item!(
            ItemType::SulfuricAcid,
            (
                recipe!(
                    6.0,
                    ManFac::ChemicalPlant,
                    (
                        recitem!(4.0, ItemType::Water),
                        recitem!(8.0, ItemType::Stone),
                        recitem!(6.0, ItemType::RefinedOil)
                    ),
                    (recitem!(4.0, ItemType::SulfuricAcid))
                ),
                recipe!(
                    1.2,
                    ManFac::Origin,
                    (IsItem::new_nai()),
                    (recitem!(1.0, ItemType::SulfuricAcid))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        refined_oil,
        ItemType::RefinedOil,
        item!(
            ItemType::RefinedOil,
            (
                recipe!(
                    4.0,
                    ManFac::OilRefinery,
                    (recitem!(2.0, ItemType::CrudeOil)),
                    (
                        recitem!(2.0, ItemType::RefinedOil),
                        recitem!(1.0, ItemType::Hydrogen)
                    )
                ),
                recipe!(
                    4.0,
                    ManFac::OilRefinery,
                    (
                        recitem!(1.0, ItemType::Coal),
                        recitem!(1.0, ItemType::Hydrogen),
                        recitem!(2.0, ItemType::RefinedOil)
                    ),
                    (recitem!(3.0, ItemType::RefinedOil))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        magnet,
        ItemType::Magnet,
        item!(
            ItemType::Magnet,
            (recipe!(
                1.5,
                ManFac::Furnace,
                (recitem!(1.0, ItemType::IronOre)),
                (recitem!(1.0, ItemType::Magnet))
            ))
        )
    );
    to_hash!(
        res_hash,
        magnetic_coil,
        ItemType::MagneticCoil,
        item!(
            ItemType::MagneticCoil,
            (recipe!(
                1.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::CopperIngot),
                    recitem!(2.0, ItemType::Magnet)
                ),
                (recitem!(2.0, ItemType::MagneticCoil))
            ))
        )
    );
    to_hash!(
        res_hash,
        glass,
        ItemType::Glass,
        item!(
            ItemType::Glass,
            (recipe!(
                2.0,
                ManFac::Furnace,
                (recitem!(2.0, ItemType::Stone)),
                (recitem!(1.0, ItemType::Glass))
            ))
        )
    );
    to_hash!(
        res_hash,
        diamond,
        ItemType::Diamond,
        item!(
            ItemType::Diamond,
            (
                recipe!(
                    2.0,
                    ManFac::Furnace,
                    (recitem!(1.0, ItemType::EnergeticGraphite)),
                    (recitem!(1.0, ItemType::Diamond))
                ),
                recipe!(
                    1.5,
                    ManFac::Furnace,
                    (recitem!(1.0, ItemType::KimberliteOre)),
                    (recitem!(2.0, ItemType::Diamond))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        crystal_silicon,
        ItemType::CrystalSilicon,
        item!(
            ItemType::CrystalSilicon,
            (
                recipe!(
                    2.0,
                    ManFac::Furnace,
                    (recitem!(1.0, ItemType::HighPuritySilicon)),
                    (recitem!(1.0, ItemType::CrystalSilicon))
                ),
                recipe!(
                    1.5,
                    ManFac::Furnace,
                    (recitem!(1.0, ItemType::FractalSilicon)),
                    (recitem!(2.0, ItemType::CrystalSilicon))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        steel,
        ItemType::Steel,
        item!(
            ItemType::Steel,
            (recipe!(
                3.0,
                ManFac::Furnace,
                (recitem!(3.0, ItemType::IronIngot)),
                (recitem!(1.0, ItemType::Steel))
            ))
        )
    );
    to_hash!(
        res_hash,
        titanium_alloy,
        ItemType::TitaniumAlloy,
        item!(
            ItemType::TitaniumAlloy,
            (recipe!(
                12.0,
                ManFac::Furnace,
                (
                    recitem!(8.0, ItemType::SulfuricAcid),
                    recitem!(4.0, ItemType::Steel),
                    recitem!(4.0, ItemType::TitaniumIngot)
                ),
                (recitem!(4.0, ItemType::TitaniumAlloy))
            ))
        )
    );
    to_hash!(
        res_hash,
        circuit_board,
        ItemType::CircuitBoard,
        item!(
            ItemType::CircuitBoard,
            (recipe!(
                1.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::CopperIngot),
                    recitem!(2.0, ItemType::IronIngot)
                ),
                (recitem!(2.0, ItemType::CircuitBoard))
            ))
        )
    );
    to_hash!(
        res_hash,
        prism,
        ItemType::Prism,
        item!(
            ItemType::Prism,
            (recipe!(
                2.0,
                ManFac::Furnace,
                (recitem!(3.0, ItemType::Glass)),
                (recitem!(2.0, ItemType::Prism))
            ))
        )
    );
    to_hash!(
        res_hash,
        electric_motor,
        ItemType::ElectricMotor,
        item!(
            ItemType::ElectricMotor,
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::MagneticCoil),
                    recitem!(1.0, ItemType::Gear),
                    recitem!(2.0, ItemType::IronIngot)
                ),
                (recitem!(1.0, ItemType::ElectricMotor))
            ))
        )
    );
    to_hash!(
        res_hash,
        microcrystalline_component,
        ItemType::MicrocrystallineComponent,
        item!(
            ItemType::MicrocrystallineComponent,
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::CopperIngot),
                    recitem!(2.0, ItemType::HighPuritySilicon)
                ),
                (recitem!(1.0, ItemType::MicrocrystallineComponent))
            ))
        )
    );
    to_hash!(
        res_hash,
        gear,
        ItemType::Gear,
        item!(
            ItemType::Gear,
            (recipe!(
                1.0,
                ManFac::Assembler,
                (recitem!(1.0, ItemType::IronIngot)),
                (recitem!(1.0, ItemType::Gear))
            ))
        )
    );
    to_hash!(
        res_hash,
        plasma_exciter,
        ItemType::PlasmaExciter,
        item!(
            ItemType::PlasmaExciter,
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, ItemType::Prism),
                    recitem!(4.0, ItemType::MagneticCoil)
                ),
                (recitem!(1.0, ItemType::PlasmaExciter))
            ))
        )
    );
    to_hash!(
        res_hash,
        photon_combiner,
        ItemType::PhotonCombiner,
        item!(
            ItemType::PhotonCombiner,
            (
                recipe!(
                    3.0,
                    ManFac::Assembler,
                    (
                        recitem!(1.0, ItemType::CircuitBoard),
                        recitem!(2.0, ItemType::Prism)
                    ),
                    (recitem!(1.0, ItemType::PhotonCombiner))
                ),
                recipe!(
                    3.0,
                    ManFac::Assembler,
                    (
                        recitem!(1.0, ItemType::CircuitBoard),
                        recitem!(1.0, ItemType::GratingCrystal)
                    ),
                    (recitem!(1.0, ItemType::PhotonCombiner))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        electromagnetic_turbine,
        ItemType::ElectromagneticTurbine,
        item!(
            ItemType::ElectromagneticTurbine,
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, ItemType::MagneticCoil),
                    recitem!(2.0, ItemType::ElectricMotor)
                ),
                (recitem!(1.0, ItemType::ElectromagneticTurbine))
            ))
        )
    );
    to_hash!(
        res_hash,
        processor,
        ItemType::Processor,
        item!(
            ItemType::Processor,
            (recipe!(
                3.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, ItemType::MicrocrystallineComponent),
                    recitem!(2.0, ItemType::CircuitBoard)
                ),
                (recitem!(1.0, ItemType::Processor))
            ))
        )
    );
    to_hash!(
        res_hash,
        engine,
        ItemType::Engine,
        item!(
            ItemType::Engine,
            (recipe!(
                3.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, ItemType::CopperIngot),
                    recitem!(1.0, ItemType::MagneticCoil)
                ),
                (recitem!(1.0, ItemType::Engine))
            ))
        )
    );
    to_hash!(
        res_hash,
        thruster,
        ItemType::Thruster,
        item!(
            ItemType::Thruster,
            (recipe!(
                4.0,
                ManFac::Assembler,
                (recitem!(3.0, ItemType::CopperIngot)),
                (recitem!(2.0, ItemType::Steel))
            ))
        )
    );
    to_hash!(
        res_hash,
        reinforced_thruster,
        ItemType::ReinforcedThruster,
        item!(
            ItemType::ReinforcedThruster,
            (recipe!(
                6.0,
                ManFac::Assembler,
                (
                    recitem!(5.0, ItemType::ElectromagneticTurbine),
                    recitem!(5.0, ItemType::TitaniumAlloy)
                ),
                (recitem!(1.0, ItemType::ReinforcedThruster))
            ))
        )
    );
    to_hash!(
        res_hash,
        super_magnetic_ring,
        ItemType::SuperMagneticRing,
        item!(
            ItemType::SuperMagneticRing,
            (recipe!(
                3.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::EnergeticGraphite),
                    recitem!(3.0, ItemType::Magnet),
                    recitem!(2.0, ItemType::ElectromagneticTurbine)
                ),
                (recitem!(1.0, ItemType::SuperMagneticRing))
            ))
        )
    );
    to_hash!(
        res_hash,
        particle_container,
        ItemType::ParticleContainer,
        item!(
            ItemType::ParticleContainer,
            (
                recipe!(
                    4.0,
                    ManFac::Assembler,
                    (
                        recitem!(2.0, ItemType::Graphene),
                        recitem!(2.0, ItemType::CopperIngot),
                        recitem!(2.0, ItemType::ElectromagneticTurbine)
                    ),
                    (recitem!(1.0, ItemType::ParticleContainer))
                ),
                recipe!(
                    4.0,
                    ManFac::Assembler,
                    (
                        recitem!(2.0, ItemType::CopperIngot),
                        recitem!(10.0, ItemType::UnipolarMagnet)
                    ),
                    (recitem!(1.0, ItemType::ParticleContainer))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        plastic,
        ItemType::Plastic,
        item!(
            ItemType::Plastic,
            (recipe!(
                3.0,
                ManFac::ChemicalPlant,
                (
                    recitem!(1.0, ItemType::EnergeticGraphite),
                    recitem!(2.0, ItemType::RefinedOil)
                ),
                (recitem!(1.0, ItemType::Plastic))
            ))
        )
    );
    to_hash!(
        res_hash,
        organic_crystal,
        ItemType::OrganicCrystal,
        item!(
            ItemType::OrganicCrystal,
            (
                recipe!(
                    6.0,
                    ManFac::ChemicalPlant,
                    (
                        recitem!(1.0, ItemType::Water),
                        recitem!(1.0, ItemType::RefinedOil),
                        recitem!(2.0, ItemType::Plastic)
                    ),
                    (recitem!(1.0, ItemType::OrganicCrystal))
                ),
                recipe!(
                    6.0,
                    ManFac::ChemicalPlant,
                    (
                        recitem!(10.0, ItemType::Water),
                        recitem!(30.0, ItemType::PlantFuel),
                        recitem!(20.0, ItemType::Log),
                    ),
                    (recitem!(1.0, ItemType::OrganicCrystal))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        graphene,
        ItemType::Graphene,
        item!(
            ItemType::Graphene,
            (
                recipe!(
                    3.0,
                    ManFac::ChemicalPlant,
                    (
                        recitem!(1.0, ItemType::SulfuricAcid),
                        recitem!(3.0, ItemType::EnergeticGraphite)
                    ),
                    (recitem!(2.0, ItemType::Graphene))
                ),
                recipe!(
                    2.0,
                    ManFac::ChemicalPlant,
                    (recitem!(2.0, ItemType::FireIce)),
                    (
                        recitem!(1.0, ItemType::Hydrogen),
                        recitem!(2.0, ItemType::Graphene)
                    )
                )
            )
        )
    );
    to_hash!(
        res_hash,
        annihilation_constraint_sphere,
        ItemType::AnnihilationConstraintSphere,
        item!(
            ItemType::AnnihilationConstraintSphere,
            (recipe!(
                20.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::Processor),
                    recitem!(1.0, ItemType::ParticleContainer)
                ),
                (recitem!(1.0, ItemType::AnnihilationConstraintSphere))
            ))
        )
    );
    to_hash!(
        res_hash,
        strange_matter,
        ItemType::StrangeMatter,
        item!(
            ItemType::StrangeMatter,
            (recipe!(
                8.0,
                ManFac::MiniatureParticleCollider,
                (
                    recitem!(10.0, ItemType::Deuterium),
                    recitem!(2.0, ItemType::IronIngot),
                    recitem!(2.0, ItemType::ParticleContainer)
                ),
                (recitem!(1.0, ItemType::StrangeMatter))
            ))
        )
    );
    to_hash!(
        res_hash,
        titanium_crystal,
        ItemType::TitaniumCrystal,
        item!(
            ItemType::TitaniumCrystal,
            (recipe!(
                4.0,
                ManFac::Assembler,
                (
                    recitem!(3.0, ItemType::TitaniumIngot),
                    recitem!(1.0, ItemType::OrganicCrystal)
                ),
                (recitem!(1.0, ItemType::TitaniumCrystal))
            ))
        )
    );
    to_hash!(
        res_hash,
        carbon_nanotube,
        ItemType::CarbonNanotube,
        item!(
            ItemType::CarbonNanotube,
            (
                recipe!(
                    4.0,
                    ManFac::ChemicalPlant,
                    (
                        recitem!(1.0, ItemType::TitaniumIngot),
                        recitem!(3.0, ItemType::Graphene)
                    ),
                    (recitem!(2.0, ItemType::CarbonNanotube))
                ),
                recipe!(
                    4.0,
                    ManFac::ChemicalPlant,
                    (recitem!(6.0, ItemType::StalagmiteCrystal)),
                    (recitem!(2.0, ItemType::CarbonNanotube))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        particle_broadband,
        ItemType::ParticleBroadband,
        item!(
            ItemType::ParticleBroadband,
            (recipe!(
                8.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::Plastic),
                    recitem!(2.0, ItemType::CrystalSilicon),
                    recitem!(2.0, ItemType::CarbonNanotube)
                ),
                (recitem!(1.0, ItemType::ParticleBroadband))
            ))
        )
    );
    to_hash!(
        res_hash,
        casimir_crystal,
        ItemType::CasimirCrystal,
        item!(
            ItemType::CasimirCrystal,
            (
                recipe!(
                    4.0,
                    ManFac::Assembler,
                    (
                        recitem!(12.0, ItemType::Hydrogen),
                        recitem!(2.0, ItemType::Graphene),
                        recitem!(1.0, ItemType::TitaniumCrystal)
                    ),
                    (recitem!(1.0, ItemType::CasimirCrystal))
                ),
                recipe!(
                    4.0,
                    ManFac::Assembler,
                    (
                        recitem!(12.0, ItemType::Hydrogen),
                        recitem!(2.0, ItemType::Graphene),
                        recitem!(8.0, ItemType::GratingCrystal)
                    ),
                    (recitem!(1.0, ItemType::CasimirCrystal))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        titanium_glass,
        ItemType::TitaniumGlass,
        item!(
            ItemType::TitaniumGlass,
            (recipe!(
                5.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, ItemType::Water),
                    recitem!(2.0, ItemType::TitaniumIngot),
                    recitem!(2.0, ItemType::Glass)
                ),
                (recitem!(2.0, ItemType::TitaniumGlass))
            ))
        )
    );
    to_hash!(
        res_hash,
        plane_filter,
        ItemType::PlaneFilter,
        item!(
            ItemType::PlaneFilter,
            (recipe!(
                12.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, ItemType::TitaniumGlass),
                    recitem!(1.0, ItemType::CasimirCrystal)
                ),
                (recitem!(1.0, ItemType::PlaneFilter))
            ))
        )
    );
    to_hash!(
        res_hash,
        quantum_chip,
        ItemType::QuantumChip,
        item!(
            ItemType::QuantumChip,
            (recipe!(
                6.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, ItemType::PlaneFilter),
                    recitem!(2.0, ItemType::Processor)
                ),
                (recitem!(1.0, ItemType::QuantumChip))
            ))
        )
    );

    to_hash!(
        res_hash,
        combustible_unit,
        ItemType::CombustibleUnit,
        item!(
            ItemType::CombustibleUnit,
            (recipe!(
                3.0,
                ManFac::Assembler,
                (recitem!(3.0, ItemType::Coal)),
                (recitem!(1.0, ItemType::CombustibleUnit))
            ))
        )
    );
    // logistics
    to_hash!(
        res_hash,
        logistics_bot,
        ItemType::LogisticsBot,
        item!(
            ItemType::LogisticsBot,
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::Processor),
                    recitem!(1.0, ItemType::Engine),
                    recitem!(2.0, ItemType::IronIngot)
                ),
                (recitem!(1.0, ItemType::LogisticsBot))
            ))
        )
    );
    to_hash!(
        res_hash,
        logistics_drone,
        ItemType::LogisticsDrone,
        item!(
            ItemType::LogisticsDrone,
            (recipe!(
                4.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, ItemType::Thruster),
                    recitem!(2.0, ItemType::Processor),
                    recitem!(5.0, ItemType::IronIngot)
                ),
                (recitem!(1.0, ItemType::LogisticsDrone))
            ))
        )
    );
    to_hash!(
        res_hash,
        interstellar_logistics_vessel,
        ItemType::InterstellarLogisticsVessel,
        item!(
            ItemType::InterstellarLogisticsVessel,
            (recipe!(
                6.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, ItemType::ReinforcedThruster),
                    recitem!(10.0, ItemType::Processor),
                    recitem!(10.0, ItemType::TitaniumAlloy)
                ),
                (recitem!(1.0, ItemType::InterstellarLogisticsVessel))
            ))
        )
    );
    to_hash!(
        res_hash,
        graviton_lens,
        ItemType::GravitonLens,
        item!(
            ItemType::GravitonLens,
            (recipe!(
                6.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::StrangeMatter),
                    recitem!(4.0, ItemType::Diamond)
                ),
                (recitem!(1.0, ItemType::GravitonLens))
            ))
        )
    );
    to_hash!(
        res_hash,
        space_warper,
        ItemType::SpaceWarper,
        item!(
            ItemType::SpaceWarper,
            (
                recipe!(
                    10.0,
                    ManFac::Assembler,
                    (recitem!(1.0, ItemType::GravitonLens)),
                    (recitem!(1.0, ItemType::SpaceWarper))
                ),
                recipe!(
                    10.0,
                    ManFac::Assembler,
                    (recitem!(1.0, ItemType::GravityMatrix)),
                    (recitem!(8.0, ItemType::SpaceWarper))
                )
            )
        )
    );
    to_hash!(
        res_hash,
        foundation,
        ItemType::Foundation,
        item!(
            ItemType::Foundation,
            (recipe!(
                1.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::Steel),
                    recitem!(3.0, ItemType::StoneBrick)
                ),
                (recitem!(1.0, ItemType::Foundation))
            ))
        )
    );
    // proliferator
    to_hash!(
        res_hash,
        proliferator_mki,
        ItemType::ProliferatorMkI,
        item!(
            ItemType::ProliferatorMkI,
            (recipe!(
                0.5,
                ManFac::Assembler,
                (recitem!(1.0, ItemType::Coal)),
                (recitem!(1.0, ItemType::ProliferatorMkI))
            ))
        )
    );
    to_hash!(
        res_hash,
        proliferator_mkii,
        ItemType::ProliferatorMkII,
        item!(
            ItemType::ProliferatorMkII,
            (recipe!(
                1.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::Diamond),
                    recitem!(2.0, ItemType::ProliferatorMkI)
                ),
                (recitem!(1.0, ItemType::ProliferatorMkII))
            ))
        )
    );
    to_hash!(
        res_hash,
        proliferator_mkiii,
        ItemType::ProliferatorMkIII,
        item!(
            ItemType::ProliferatorMkIII,
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::CarbonNanotube),
                    recitem!(2.0, ItemType::ProliferatorMkII)
                ),
                (recitem!(1.0, ItemType::ProliferatorMkIII))
            ))
        )
    );
    // fuel rods
    to_hash!(
        res_hash,
        hydrogen_fuel_rod,
        ItemType::HydrogenFuelRod,
        item!(
            ItemType::HydrogenFuelRod,
            (recipe!(
                6.0,
                ManFac::Assembler,
                (
                    recitem!(10.0, ItemType::Hydrogen),
                    recitem!(1.0, ItemType::TitaniumIngot)
                ),
                (recitem!(2.0, ItemType::HydrogenFuelRod))
            ))
        )
    );
    to_hash!(
        res_hash,
        deuterium_fuel_rod,
        ItemType::DeuteriumFuelRod,
        item!(
            ItemType::DeuteriumFuelRod,
            (recipe!(
                12.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::SuperMagneticRing),
                    recitem!(20.0, ItemType::Deuterium),
                    recitem!(1.0, ItemType::TitaniumAlloy)
                ),
                (recitem!(2.0, ItemType::DeuteriumFuelRod))
            ))
        )
    );
    to_hash!(
        res_hash,
        antimatter_fuel_rod,
        ItemType::AntimatterFuelRod,
        item!(
            ItemType::AntimatterFuelRod,
            (recipe!(
                24.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::TitaniumAlloy),
                    recitem!(1.0, ItemType::AnnihilationConstraintSphere),
                    recitem!(12.0, ItemType::Hydrogen),
                    recitem!(12.0, ItemType::Antimatter)
                ),
                (recitem!(2.0, ItemType::AntimatterFuelRod))
            ))
        )
    );
    to_hash!(
        res_hash,
        strange_annihilation_fuel_rod,
        ItemType::StrangeAnnihilationFuelRod,
        item!(
            ItemType::StrangeAnnihilationFuelRod,
            (recipe!(
                32.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::FrameMaterial),
                    recitem!(2.0, ItemType::StrangeMatter),
                    recitem!(1.0, ItemType::CoreElement),
                    recitem!(8.0, ItemType::AntimatterFuelRod)
                ),
                (recitem!(1.0, ItemType::StrangeAnnihilationFuelRod))
            ))
        )
    );
    // dyson sphere building
    to_hash!(
        res_hash,
        solar_sail,
        ItemType::SolarSail,
        item!(
            ItemType::SolarSail,
            (recipe!(
                4.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::PhotonCombiner),
                    recitem!(1.0, ItemType::Graphene)
                ),
                (recitem!(2.0, ItemType::SolarSail))
            ))
        )
    );
    to_hash!(
        res_hash,
        frame_material,
        ItemType::FrameMaterial,
        item!(
            ItemType::FrameMaterial,
            (recipe!(
                6.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::HighPuritySilicon),
                    recitem!(1.0, ItemType::TitaniumAlloy),
                    recitem!(4.0, ItemType::CarbonNanotube)
                ),
                (recitem!(1.0, ItemType::FrameMaterial))
            ))
        )
    );
    to_hash!(
        res_hash,
        dyson_sphere_component,
        ItemType::DysonSphereComponent,
        item!(
            ItemType::DysonSphereComponent,
            (recipe!(
                8.0,
                ManFac::Assembler,
                (
                    recitem!(3.0, ItemType::Processor),
                    recitem!(3.0, ItemType::SolarSail),
                    recitem!(3.0, ItemType::FrameMaterial)
                ),
                (recitem!(1.0, ItemType::DysonSphereComponent))
            ))
        )
    );
    to_hash!(
        res_hash,
        small_carrier_rocket,
        ItemType::SmallCarrierRocket,
        item!(
            ItemType::SmallCarrierRocket,
            (recipe!(
                6.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, ItemType::QuantumChip),
                    recitem!(4.0, ItemType::DeuteriumFuelRod),
                    recitem!(2.0, ItemType::DysonSphereComponent)
                ),
                (recitem!(1.0, ItemType::SmallCarrierRocket))
            ))
        )
    );
    // Matrices
    to_hash!(
        res_hash,
        electromagnetic_matrix,
        ItemType::ElectromagneticMatrix,
        item!(
            ItemType::ElectromagneticMatrix,
            (recipe!(
                3.0,
                ManFac::Lab,
                (
                    recitem!(1.0, ItemType::CircuitBoard),
                    recitem!(1.0, ItemType::MagneticCoil)
                ),
                (recitem!(1.0, ItemType::ElectromagneticMatrix))
            ))
        )
    );
    to_hash!(
        res_hash,
        energy_matrix,
        ItemType::EnergyMatrix,
        item!(
            ItemType::EnergyMatrix,
            (recipe!(
                6.0,
                ManFac::Lab,
                (
                    recitem!(2.0, ItemType::Hydrogen),
                    recitem!(2.0, ItemType::EnergeticGraphite)
                ),
                (recitem!(1.0, ItemType::EnergyMatrix))
            ))
        )
    );
    to_hash!(
        res_hash,
        structure_matrix,
        ItemType::StructureMatrix,
        item!(
            ItemType::StructureMatrix,
            (recipe!(
                8.0,
                ManFac::Lab,
                (
                    recitem!(1.0, ItemType::TitaniumCrystal),
                    recitem!(1.0, ItemType::Diamond)
                ),
                (recitem!(1.0, ItemType::StructureMatrix))
            ))
        )
    );
    to_hash!(
        res_hash,
        information_matrix,
        ItemType::InformationMatrix,
        item!(
            ItemType::InformationMatrix,
            (recipe!(
                10.0,
                ManFac::Lab,
                (
                    recitem!(1.0, ItemType::ParticleBroadband),
                    recitem!(2.0, ItemType::Processor)
                ),
                (recitem!(1.0, ItemType::InformationMatrix))
            ))
        )
    );
    to_hash!(
        res_hash,
        gravity_matrix,
        ItemType::GravityMatrix,
        item!(
            ItemType::GravityMatrix,
            (recipe!(
                24.0,
                ManFac::Lab,
                (
                    recitem!(1.0, ItemType::QuantumChip),
                    recitem!(1.0, ItemType::GravitonLens)
                ),
                (recitem!(2.0, ItemType::GravityMatrix))
            ))
        )
    );
    to_hash!(
        res_hash,
        universe_matrix,
        ItemType::UniverseMatrix,
        item!(
            ItemType::UniverseMatrix,
            (recipe!(
                15.0,
                ManFac::Lab,
                (
                    recitem!(1.0, ItemType::ElectromagneticMatrix),
                    recitem!(1.0, ItemType::EnergyMatrix),
                    recitem!(1.0, ItemType::StructureMatrix),
                    recitem!(1.0, ItemType::InformationMatrix),
                    recitem!(1.0, ItemType::GravityMatrix),
                    recitem!(1.0, ItemType::Antimatter)
                ),
                (recitem!(1.0, ItemType::UniverseMatrix))
            ))
        )
    );
    // ammunition
    to_hash!(
        res_hash,
        magnum_ammo_box,
        ItemType::MagnumAmmoBox,
        item!(
            ItemType::MagnumAmmoBox,
            (recipe!(
                1.0,
                ManFac::Assembler,
                (recitem!(3.0, ItemType::CopperIngot)),
                (recitem!(1.0, ItemType::MagnumAmmoBox))
            ))
        )
    );
    to_hash!(
        res_hash,
        titanium_ammo_box,
        ItemType::TitaniumAmmoBox,
        item!(
            ItemType::TitaniumAmmoBox,
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, ItemType::TitaniumIngot),
                    recitem!(1.0, ItemType::MagnumAmmoBox)
                ),
                (recitem!(1.0, ItemType::TitaniumAmmoBox))
            ))
        )
    );
    to_hash!(
        res_hash,
        superalloy_ammo_box,
        ItemType::SuperalloyAmmoBox,
        item!(
            ItemType::SuperalloyAmmoBox,
            (recipe!(
                3.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::TitaniumAlloy),
                    recitem!(1.0, ItemType::TitaniumAmmoBox)
                ),
                (recitem!(1.0, ItemType::SuperalloyAmmoBox))
            ))
        )
    );
    to_hash!(
        res_hash,
        explosive_unit,
        ItemType::ExplosiveUnit,
        item!(
            ItemType::ExplosiveUnit,
            (recipe!(
                6.0,
                ManFac::ChemicalPlant,
                (
                    recitem!(1.0, ItemType::SulfuricAcid),
                    recitem!(2.0, ItemType::Plastic),
                    recitem!(2.0, ItemType::CombustibleUnit)
                ),
                (recitem!(2.0, ItemType::ExplosiveUnit))
            ))
        )
    );
    to_hash!(
        res_hash,
        crystal_explosive_unit,
        ItemType::CrystalExplosiveUnit,
        item!(
            ItemType::CrystalExplosiveUnit,
            (recipe!(
                24.0,
                ManFac::ChemicalPlant,
                (
                    recitem!(8.0, ItemType::CrystalSilicon),
                    recitem!(1.0, ItemType::CasimirCrystal),
                    recitem!(8.0, ItemType::ExplosiveUnit)
                ),
                (recitem!(8.0, ItemType::CrystalExplosiveUnit))
            ))
        )
    );
    to_hash!(
        res_hash,
        missile_set,
        ItemType::MissileSet,
        item!(
            ItemType::MissileSet,
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::Engine),
                    recitem!(2.0, ItemType::CombustibleUnit),
                    recitem!(3.0, ItemType::CircuitBoard),
                    recitem!(6.0, ItemType::CopperIngot)
                ),
                (recitem!(1.0, ItemType::MissileSet))
            ))
        )
    );
    to_hash!(
        res_hash,
        supersonic_missile_set,
        ItemType::SupersonicMissileSet,
        item!(
            ItemType::SupersonicMissileSet,
            (recipe!(
                4.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, ItemType::Thruster),
                    recitem!(2.0, ItemType::ExplosiveUnit),
                    recitem!(4.0, ItemType::Processor),
                    recitem!(2.0, ItemType::MissileSet)
                ),
                (recitem!(2.0, ItemType::SupersonicMissileSet))
            ))
        )
    );
    to_hash!(
        res_hash,
        gravity_missile_set,
        ItemType::GravityMissileSet,
        item!(
            ItemType::GravityMissileSet,
            (recipe!(
                6.0,
                ManFac::Assembler,
                (
                    recitem!(3.0, ItemType::StrangeMatter),
                    recitem!(6.0, ItemType::CrystalExplosiveUnit),
                    recitem!(3.0, ItemType::SupersonicMissileSet)
                ),
                (recitem!(3.0, ItemType::GravityMissileSet))
            ))
        )
    );
    to_hash!(
        res_hash,
        shell_set,
        ItemType::ShellSet,
        item!(
            ItemType::ShellSet,
            (recipe!(
                1.5,
                ManFac::Assembler,
                (
                    recitem!(2.0, ItemType::ExplosiveUnit),
                    recitem!(9.0, ItemType::CopperIngot)
                ),
                (recitem!(1.0, ItemType::ShellSet))
            ))
        )
    );
    to_hash!(
        res_hash,
        high_explosive_shell_set,
        ItemType::HighExplosiveShellSet,
        item!(
            ItemType::HighExplosiveShellSet,
            (recipe!(
                3.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, ItemType::ExplosiveUnit),
                    recitem!(6.0, ItemType::TitaniumIngot),
                    recitem!(1.0, ItemType::ShellSet)
                ),
                (recitem!(1.0, ItemType::HighExplosiveShellSet))
            ))
        )
    );
    to_hash!(
        res_hash,
        crystal_shell_set,
        ItemType::CrystalShellSet,
        item!(
            ItemType::CrystalShellSet,
            (recipe!(
                6.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, ItemType::CrystalExplosiveUnit),
                    recitem!(3.0, ItemType::TitaniumIngot),
                    recitem!(1.0, ItemType::HighExplosiveShellSet)
                ),
                (recitem!(1.0, ItemType::CrystalShellSet))
            ))
        )
    );
    to_hash!(
        res_hash,
        plasma_capsule,
        ItemType::PlasmaCapsule,
        item!(
            ItemType::PlasmaCapsule,
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(10.0, ItemType::Deuterium),
                    recitem!(2.0, ItemType::Magnet),
                    recitem!(1.0, ItemType::Graphene)
                ),
                (recitem!(1.0, ItemType::PlasmaCapsule))
            ))
        )
    );
    to_hash!(
        res_hash,
        antimatter_capsule,
        ItemType::AntimatterCapsule,
        item!(
            ItemType::AntimatterCapsule,
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(10.0, ItemType::Antimatter),
                    recitem!(10.0, ItemType::Hydrogen),
                    recitem!(1.0, ItemType::ParticleContainer),
                    recitem!(1.0, ItemType::PlasmaCapsule)
                ),
                (recitem!(1.0, ItemType::AntimatterCapsule))
            ))
        )
    );
    to_hash!(
        res_hash,
        jamming_capsule,
        ItemType::JammingCapsule,
        item!(
            ItemType::JammingCapsule,
            (recipe!(
                2.0,
                ManFac::Assembler,
                (
                    recitem!(3.0, ItemType::Hydrogen),
                    recitem!(1.0, ItemType::PlasmaExciter),
                    recitem!(1.0, ItemType::ElectromagneticTurbine)
                ),
                (recitem!(1.0, ItemType::JammingCapsule))
            ))
        )
    );
    to_hash!(
        res_hash,
        suppressing_capsule,
        ItemType::SuppressingCapsule,
        item!(
            ItemType::SuppressingCapsule,
            (recipe!(
                8.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, ItemType::TitaniumGlass),
                    recitem!(1.0, ItemType::SuperMagneticRing),
                    recitem!(2.0, ItemType::JammingCapsule)
                ),
                (recitem!(2.0, ItemType::SuppressingCapsule))
            ))
        )
    );
    // drones and ships
    to_hash!(
        res_hash,
        prototype,
        ItemType::Prototype,
        item!(
            ItemType::Prototype,
            (recipe!(
                3.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::PlasmaExciter),
                    recitem!(2.0, ItemType::CircuitBoard),
                    recitem!(1.0, ItemType::Engine),
                    recitem!(3.0, ItemType::IronIngot)
                ),
                (recitem!(1.0, ItemType::Prototype))
            ))
        )
    );
    to_hash!(
        res_hash,
        precision_drone,
        ItemType::PrecisionDrone,
        item!(
            ItemType::PrecisionDrone,
            (recipe!(
                4.0,
                ManFac::Assembler,
                (
                    recitem!(2.0, ItemType::PhotonCombiner),
                    recitem!(2.0, ItemType::CircuitBoard),
                    recitem!(1.0, ItemType::ElectromagneticTurbine),
                    recitem!(1.0, ItemType::Prototype)
                ),
                (recitem!(1.0, ItemType::PrecisionDrone))
            ))
        )
    );
    to_hash!(
        res_hash,
        attack_drone,
        ItemType::AttackDrone,
        item!(
            ItemType::AttackDrone,
            (recipe!(
                4.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::ParticleContainer),
                    recitem!(1.0, ItemType::Processor),
                    recitem!(1.0, ItemType::ElectromagneticTurbine),
                    recitem!(1.0, ItemType::Prototype)
                ),
                (recitem!(1.0, ItemType::AttackDrone))
            ))
        )
    );
    to_hash!(
        res_hash,
        corvette,
        ItemType::Corvette,
        item!(
            ItemType::Corvette,
            (recipe!(
                5.0,
                ManFac::Assembler,
                (
                    recitem!(3.0, ItemType::ParticleContainer),
                    recitem!(2.0, ItemType::Processor),
                    recitem!(1.0, ItemType::ReinforcedThruster),
                    recitem!(5.0, ItemType::TitaniumAlloy)
                ),
                (recitem!(1.0, ItemType::Corvette))
            ))
        )
    );
    to_hash!(
        res_hash,
        destroyer,
        ItemType::Destroyer,
        item!(
            ItemType::Destroyer,
            (recipe!(
                8.0,
                ManFac::Assembler,
                (
                    recitem!(1.0, ItemType::StrangeMatter),
                    recitem!(4.0, ItemType::Processor),
                    recitem!(4.0, ItemType::ReinforcedThruster),
                    recitem!(20.0, ItemType::FrameMaterial)
                ),
                (recitem!(1.0, ItemType::Destroyer))
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

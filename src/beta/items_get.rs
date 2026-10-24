// all crafting recipe numbers are from the [wiki](https://dsp-wiki.com/Items) are

// import hashmaps
use std::collections::HashMap;
// import the structs, emuns, etc.
use crate::beta::essentials::{IsItem, Item, ItemAmount, ManFac, Recipe};
use crate::beta::items::ItemType;
// import all macros
use crate::{new_item, recipe, recitem};

/// creates a big HashMap with all the items and buildings in the game
pub fn get_items() -> HashMap<ItemType, Item> {
    let mut res_hash = HashMap::new();
    /*
    macro template explanation
    new_item!(
        res_hash,
        <item type>,
        (recipe!(
            <seconds to craft>,
            <Manufacturing facility>,
                (recitem!(<Amount Item>, <ingredient item type>)),
                (recitem!(<Amount Item>, <product item type>))
        ))
    );
    copy paste reference
    new_item!(
        res_hash,
        ,
        (recipe!(
            ,
            ,
                (recitem!(,)),
                (recitem!(,))
        ))
    );
    */
    // all ores/origin items
    new_item!(
        res_hash,
        ItemType::IronOre,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::IronOre))
        ))
    );
    new_item!(
        res_hash,
        ItemType::CopperOre,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::CopperOre))
        ))
    );
    new_item!(
        res_hash,
        ItemType::Stone,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::Stone))
        ))
    );
    new_item!(
        res_hash,
        ItemType::Coal,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::Coal))
        ))
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
        ItemType::TitaniumOre,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::TitaniumOre))
        ))
    );
    new_item!(
        res_hash,
        ItemType::Water,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::Water))
        ))
    );
    new_item!(
        res_hash,
        ItemType::CrudeOil,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::CrudeOil))
        ))
    );
    // hydrogen requires a lot of other items to be declared
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
        ItemType::CoreElement,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::CoreElement))
        ))
    );
    new_item!(
        res_hash,
        ItemType::CriticalPhoton,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::CriticalPhoton))
        ))
    );
    new_item!(
        res_hash,
        ItemType::KimberliteOre,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::KimberliteOre))
        ))
    );
    new_item!(
        res_hash,
        ItemType::FractalSilicon,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::FractalSilicon))
        ))
    );
    new_item!(
        res_hash,
        ItemType::GratingCrystal,
        (recipe!(
            1.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::GratingCrystal))
        ))
    );
    new_item!(
        res_hash,
        ItemType::StalagmiteCrystal,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::StalagmiteCrystal))
        ))
    );
    new_item!(
        res_hash,
        ItemType::UnipolarMagnet,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::UnipolarMagnet))
        ))
    );
    new_item!(
        res_hash,
        ItemType::FireIce,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::FireIce))
        ))
    );
    new_item!(
        res_hash,
        ItemType::Log,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::Log))
        ))
    );
    new_item!(
        res_hash,
        ItemType::PlantFuel,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::PlantFuel))
        ))
    );
    new_item!(
        res_hash,
        ItemType::DarkFogMatrix,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::DarkFogMatrix))
        ))
    );
    new_item!(
        res_hash,
        ItemType::EnergyShard,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::EnergyShard))
        ))
    );
    new_item!(
        res_hash,
        ItemType::SiliconBasedNeuron,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::SiliconBasedNeuron))
        ))
    );
    new_item!(
        res_hash,
        ItemType::NegentropySingularity,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::NegentropySingularity))
        ))
    );
    new_item!(
        res_hash,
        ItemType::MatterRecombinator,
        (recipe!(
            0.0,
            ManFac::Origin,
            (IsItem::new_nai()),
            (recitem!(1.0, ItemType::MatterRecombinator))
        ))
    );
    // processed items
    new_item!(
        res_hash,
        ItemType::IronIngot,
        (recipe!(
            1.0,
            ManFac::Furnace,
            (recitem!(1.0, ItemType::IronOre)),
            (recitem!(1.0, ItemType::IronIngot))
        ))
    );
    new_item!(
        res_hash,
        ItemType::CopperIngot,
        (recipe!(
            1.0,
            ManFac::Furnace,
            (recitem!(1.0, ItemType::CopperOre)),
            (recitem!(1.0, ItemType::CopperIngot))
        ))
    );
    new_item!(
        res_hash,
        ItemType::StoneBrick,
        (recipe!(
            1.0,
            ManFac::Furnace,
            (recitem!(1.0, ItemType::Stone)),
            (recitem!(1.0, ItemType::StoneBrick))
        ))
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
        ItemType::HighPuritySilicon,
        (recipe!(
            2.0,
            ManFac::Furnace,
            (recitem!(2.0, ItemType::SiliconOre)),
            (recitem!(1.0, ItemType::HighPuritySilicon))
        ))
    );
    new_item!(
        res_hash,
        ItemType::TitaniumIngot,
        (recipe!(
            2.0,
            ManFac::Furnace,
            (recitem!(2.0, ItemType::TitaniumOre)),
            (recitem!(1.0, ItemType::TitaniumIngot))
        ))
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
        ItemType::Magnet,
        (recipe!(
            1.5,
            ManFac::Furnace,
            (recitem!(1.0, ItemType::IronOre)),
            (recitem!(1.0, ItemType::Magnet))
        ))
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
        ItemType::Glass,
        (recipe!(
            2.0,
            ManFac::Furnace,
            (recitem!(2.0, ItemType::Stone)),
            (recitem!(1.0, ItemType::Glass))
        ))
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
        ItemType::Steel,
        (recipe!(
            3.0,
            ManFac::Furnace,
            (recitem!(3.0, ItemType::IronIngot)),
            (recitem!(1.0, ItemType::Steel))
        ))
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
        ItemType::Prism,
        (recipe!(
            2.0,
            ManFac::Furnace,
            (recitem!(3.0, ItemType::Glass)),
            (recitem!(2.0, ItemType::Prism))
        ))
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
        ItemType::Gear,
        (recipe!(
            1.0,
            ManFac::Assembler,
            (recitem!(1.0, ItemType::IronIngot)),
            (recitem!(1.0, ItemType::Gear))
        ))
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
        ItemType::Thruster,
        (recipe!(
            4.0,
            ManFac::Assembler,
            (recitem!(3.0, ItemType::CopperIngot)),
            (recitem!(2.0, ItemType::Steel))
        ))
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );

    new_item!(
        res_hash,
        ItemType::CombustibleUnit,
        (recipe!(
            3.0,
            ManFac::Assembler,
            (recitem!(3.0, ItemType::Coal)),
            (recitem!(1.0, ItemType::CombustibleUnit))
        ))
    );
    // logistics
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    // proliferator
    new_item!(
        res_hash,
        ItemType::ProliferatorMkI,
        (recipe!(
            0.5,
            ManFac::Assembler,
            (recitem!(1.0, ItemType::Coal)),
            (recitem!(1.0, ItemType::ProliferatorMkI))
        ))
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    // fuel rods
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    // dyson sphere building
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    // Matrices
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    // ammunition
    new_item!(
        res_hash,
        ItemType::MagnumAmmoBox,
        (recipe!(
            1.0,
            ManFac::Assembler,
            (recitem!(3.0, ItemType::CopperIngot)),
            (recitem!(1.0, ItemType::MagnumAmmoBox))
        ))
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    // drones and ships
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
    );
    new_item!(
        res_hash,
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
                        IsItem::Item(item_amount) => item_amount.item_type,
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
                            item_amount.item_type
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

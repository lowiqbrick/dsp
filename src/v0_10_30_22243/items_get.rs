pub mod itemsmod {
    // import hasmaps
    use std::collections::HashMap;
    // import the structs, emuns, etc.
    use crate::v0_10_30_22243::enum_item::item_enum_support::ItemEnum::{self, *};
    use crate::v0_10_30_22243::essentials::item_logic::{IsItem, Item, ItemAmount, ManFac, Recipe};
    use crate::v0_10_30_22243::essentials::item_logic::{
        ManFac::Assembler, ManFac::ChemicalPlant, ManFac::Furnace, ManFac::Lab,
        ManFac::MiniatureParticleCollider,
    };
    // import all macros
    use crate::{item, recipe, recitem, tohash};
    /// creates a big HashMap with all the items and buildings in the game
    pub fn get_items<'a>(mut res_hash: HashMap<ItemEnum, Item>) -> HashMap<ItemEnum, Item> {
        // all ores/origin items
        let iron_ore: Item;
        let iron_ore_rec: Recipe = Recipe::new(
            0.0,
            vec![IsItem::new_nai()],
            vec![IsItem::new(ItemAmount::new(1.0, String::from(IronOre.to_string())))],
        );
        iron_ore = Item::new(&IronOre.to_string(), vec![ManFac::Origin], vec![iron_ore_rec]);
        res_hash.insert(IronOre, iron_ore);
        let copper_ore: Item = Item::new(
            &CopperOre.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    CopperOre.to_string(),
                ))],
            )],
        );
        res_hash.insert(CopperOre, copper_ore);
        let stone: Item = Item::new(
            &Stone.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(1.0, Stone.to_string()))],
            )],
        );
        res_hash.insert(Stone, stone);
        let coal: Item = Item::new(
            &Coal.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(1.0, Coal.to_string()))],
            )],
        );
        res_hash.insert(Coal, coal);
        let silicon_ore: Item = Item::new(
            &SiliconOre.to_string(),
            // also minable
            vec![ManFac::Furnace, ManFac::Origin],
            vec![Recipe::new(
                10.0,
                vec![IsItem::new(ItemAmount::new(10.0, SiliconOre.to_string()))],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    SiliconOre.to_string(),
                ))],
            )],
        );
        res_hash.insert(SiliconOre, silicon_ore);
        let titanium_ore: Item = Item::new(
            &TitaniumOre.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    TitaniumOre.to_string(),
                ))],
            )],
        );
        res_hash.insert(TitaniumOre, titanium_ore);
        let water: Item = Item::new(
            &Water.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(1.0, Water.to_string()))],
            )],
        );
        res_hash.insert(Water, water);
        let crude_oil: Item = Item::new(
            &CrudeOil.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(1.0, CrudeOil.to_string()))],
            )],
        );
        res_hash.insert(CrudeOil, crude_oil);
        // hydrogen requires a lot of other items to be declared
        let refined_oil: Item;
        let graphene: Item;
        let fire_ice: Item;
        let energetic_graphite: Item;
        let antimatter: Item;
        let critical_photon: Item;
        let gear: Item;
        let hydrogen: Item = Item::new(
            &Hydrogen.to_string(),
            vec![ManFac::ChemicalPlant, ManFac::OilRefinery, ManFac::Origin],
            vec![
                Recipe::new(
                    4.0,
                    vec![IsItem::new(ItemAmount::new(2.0, CrudeOil.to_string()))],
                    vec![
                        IsItem::new(ItemAmount::new(2.0, RefinedOil.to_string())),
                        IsItem::new(ItemAmount::new(1.0, Hydrogen.to_string())),
                    ],
                ),
                Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(2.0, FireIce.to_string()))],
                    vec![
                        IsItem::new(ItemAmount::new(1.0, Hydrogen.to_string())),
                        IsItem::new(ItemAmount::new(2.0, Graphene.to_string())),
                    ],
                ),
                Recipe::new(
                    4.0,
                    vec![
                        IsItem::new(ItemAmount::new(2.0, Hydrogen.to_string())),
                        IsItem::new(ItemAmount::new(1.0, RefinedOil.to_string())),
                    ],
                    vec![
                        IsItem::new(ItemAmount::new(1.0, EnergeticGraphite.to_string())),
                        IsItem::new(ItemAmount::new(3.0, Hydrogen.to_string())),
                    ],
                ),
                Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(
                        2.0,
                        CriticalPhoton.to_string(),
                    ))],
                    vec![
                        IsItem::new(ItemAmount::new(2.0, Hydrogen.to_string())),
                        IsItem::new(ItemAmount::new(2.0, AntiMatter.to_string())),
                    ],
                ),
            ],
        );
        res_hash.insert(Hydrogen, hydrogen);
        let deuterium: Item = Item::new(
            &Deuterium.to_string(),
            vec![ManFac::Origin, ManFac::MiniatureParticleCollider],
            vec![Recipe::new(
                2.5,
                vec![IsItem::new(ItemAmount::new(10.0, Hydrogen.to_string()))],
                vec![IsItem::new(ItemAmount::new(5.0, Deuterium.to_string()))],
            )],
        );
        res_hash.insert(Deuterium, deuterium);
        antimatter = Item::new(
            &AntiMatter.to_string(),
            vec![ManFac::MiniatureParticleCollider],
            vec![Recipe::new(
                2.0,
                vec![IsItem::new(ItemAmount::new(
                    2.0,
                    CriticalPhoton.to_string(),
                ))],
                vec![
                    IsItem::new(ItemAmount::new(2.0, Hydrogen.to_string())),
                    IsItem::new(ItemAmount::new(2.0, AntiMatter.to_string())),
                ],
            )],
        );
        res_hash.insert(AntiMatter, antimatter);
        let core_element: Item = Item::new(
            &CoreElement.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    CoreElement.to_string(),
                ))],
            )],
        );
        res_hash.insert(CoreElement, core_element);
        critical_photon = Item::new(
            &CriticalPhoton.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    CriticalPhoton.to_string(),
                ))],
            )],
        );
        res_hash.insert(CriticalPhoton, critical_photon);
        let kimberlite_ore: Item = Item::new(
            &KimberliteOre.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    KimberliteOre.to_string(),
                ))],
            )],
        );
        res_hash.insert(KimberliteOre, kimberlite_ore);
        let fractal_silicon: Item = Item::new(
            &FractalSilicon.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    FractalSilicon.to_string(),
                ))],
            )],
        );
        res_hash.insert(FractalSilicon, fractal_silicon);
        let grating_crystal: Item = Item::new(
            &GratingCrystal.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                1.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    GratingCrystal.to_string(),
                ))],
            )],
        );
        res_hash.insert(GratingCrystal, grating_crystal);
        let stalagmite_crystal: Item = Item::new(
            &StalagmiteOre.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    StalagmiteOre.to_string(),
                ))],
            )],
        );
        res_hash.insert(StalagmiteOre, stalagmite_crystal);
        let unipolar_magnet: Item = Item::new(
            &UnipolarMagnet.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    UnipolarMagnet.to_string(),
                ))],
            )],
        );
        res_hash.insert(UnipolarMagnet, unipolar_magnet);
        fire_ice = Item::new(
            &FireIce.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(1.0, FireIce.to_string()))],
            )],
        );
        res_hash.insert(FireIce, fire_ice);
        let log: Item = Item::new(
            &Log.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(1.0, Log.to_string()))],
            )],
        );
        res_hash.insert(Log, log);
        let plant_fuel: Item = Item::new(
            &PlantFuel.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    PlantFuel.to_string(),
                ))],
            )],
        );
        res_hash.insert(PlantFuel, plant_fuel);
        let dark_fog_matix: Item = Item::new(
            &DarkFogMatrix.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    DarkFogMatrix.to_string(),
                ))],
            )],
        );
        res_hash.insert(DarkFogMatrix, dark_fog_matix);
        let energy_shard: Item = Item::new(
            &EnergyShard.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    EnergyShard.to_string(),
                ))],
            )],
        );
        res_hash.insert(EnergyShard, energy_shard);
        let silicon_based_neuron: Item = Item::new(
            &SiliconBasedNeuron.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    SiliconBasedNeuron.to_string(),
                ))],
            )],
        );
        res_hash.insert(SiliconBasedNeuron, silicon_based_neuron);
        let negentropy_singularity: Item = Item::new(
            &NegentropySingularity.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    NegentropySingularity.to_string(),
                ))],
            )],
        );
        res_hash.insert(
            NegentropySingularity,
            negentropy_singularity,
        );
        let matter_recombinator: Item = Item::new(
            &MatterRecombinator.to_string(),
            vec![ManFac::Origin],
            vec![Recipe::new(
                0.0,
                vec![IsItem::new_nai()],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    MatterRecombinator.to_string(),
                ))],
            )],
        );
        res_hash.insert(MatterRecombinator, matter_recombinator);
        // processed items
        let iron_ingot = Item::new(
            &IronIngot.to_string(),
            vec![ManFac::Furnace],
            vec![Recipe::new(
                1.0,
                vec![IsItem::new(ItemAmount::new(1.0, IronOre.to_string()))],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    IronIngot.to_string(),
                ))],
            )],
        );
        res_hash.insert(IronIngot, iron_ingot);
        let copper_ingot: Item = Item::new(
            &CopperIngot.to_string(),
            vec![ManFac::Furnace],
            vec![Recipe::new(
                1.0,
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    CopperOre.to_string(),
                ))],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    CopperIngot.to_string(),
                ))],
            )],
        );
        res_hash.insert(CopperIngot, copper_ingot);
        let stone_brick: Item = Item::new(
            &StoneBrick.to_string(),
            vec![ManFac::Furnace],
            vec![Recipe::new(
                1.0,
                vec![IsItem::new(ItemAmount::new(1.0, Stone.to_string()))],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    StoneBrick.to_string(),
                ))],
            )],
        );
        res_hash.insert(StoneBrick, stone_brick);
        energetic_graphite = Item::new(
            &EnergeticGraphite.to_string(),
            vec![ManFac::Furnace],
            vec![
                Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(2.0, Coal.to_string()))],
                    vec![IsItem::new(ItemAmount::new(
                        1.0,
                        EnergeticGraphite.to_string(),
                    ))],
                ),
                Recipe::new(
                    4.0,
                    vec![
                        IsItem::new(ItemAmount::new(2.0, Hydrogen.to_string())),
                        IsItem::new(ItemAmount::new(1.0, RefinedOil.to_string())),
                    ],
                    vec![
                        IsItem::new(ItemAmount::new(1.0, EnergeticGraphite.to_string())),
                        IsItem::new(ItemAmount::new(3.0, Hydrogen.to_string())),
                    ],
                ),
            ],
        );
        res_hash.insert(EnergeticGraphite, energetic_graphite);
        let high_purity_silicon: Item = Item::new(
            &HighPuritySilicon.to_string(),
            vec![ManFac::Furnace],
            vec![Recipe::new(
                2.0,
                vec![IsItem::new(ItemAmount::new(
                    2.0,
                    SiliconOre.to_string(),
                ))],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    HighPuritySilicon.to_string(),
                ))],
            )],
        );
        res_hash.insert(HighPuritySilicon, high_purity_silicon);
        let titanium_ingot: Item = Item::new(
            &TitaniumIngot.to_string(),
            vec![ManFac::Furnace],
            vec![Recipe::new(
                2.0,
                vec![IsItem::new(ItemAmount::new(
                    2.0,
                    TitaniumOre.to_string(),
                ))],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    TitaniumIngot.to_string(),
                ))],
            )],
        );
        res_hash.insert(TitaniumIngot, titanium_ingot);
        let sulfuric_acid: Item = Item::new(
            &SulfuricAcid.to_string(),
            vec![ManFac::ChemicalPlant, ManFac::Origin],
            vec![Recipe::new(
                6.0,
                vec![
                    IsItem::new(ItemAmount::new(4.0, Water.to_string())),
                    IsItem::new(ItemAmount::new(8.0, Stone.to_string())),
                    IsItem::new(ItemAmount::new(6.0, RefinedOil.to_string())),
                ],
                vec![IsItem::new(ItemAmount::new(
                    4.0,
                    SulfuricAcid.to_string(),
                ))],
            )],
        );
        res_hash.insert(SulfuricAcid, sulfuric_acid);
        refined_oil = Item::new(
            &RefinedOil.to_string(),
            vec![ManFac::OilRefinery],
            vec![
                Recipe::new(
                    4.0,
                    vec![IsItem::new(ItemAmount::new(2.0, CrudeOil.to_string()))],
                    vec![
                        IsItem::new(ItemAmount::new(2.0, RefinedOil.to_string())),
                        IsItem::new(ItemAmount::new(1.0, Hydrogen.to_string())),
                    ],
                ),
                Recipe::new(
                    4.0,
                    vec![
                        IsItem::new(ItemAmount::new(1.0, Coal.to_string())),
                        IsItem::new(ItemAmount::new(1.0, Hydrogen.to_string())),
                        IsItem::new(ItemAmount::new(2.0, RefinedOil.to_string())),
                    ],
                    vec![IsItem::new(ItemAmount::new(
                        3.0,
                        RefinedOil.to_string(),
                    ))],
                ),
            ],
        );
        res_hash.insert(RefinedOil, refined_oil);
        let magnet: Item = Item::new(
            &Magnet.to_string(),
            vec![ManFac::Furnace],
            vec![Recipe::new(
                1.5,
                vec![IsItem::new(ItemAmount::new(1.0, IronOre.to_string()))],
                vec![IsItem::new(ItemAmount::new(1.0, Magnet.to_string()))],
            )],
        );
        res_hash.insert(Magnet, magnet);
        let magentic_coil: Item = Item::new(
            &MagneticCoil.to_string(),
            vec![ManFac::Assembler],
            vec![Recipe::new(
                1.0,
                vec![
                    IsItem::new(ItemAmount::new(1.0, CopperIngot.to_string())),
                    IsItem::new(ItemAmount::new(2.0, Magnet.to_string())),
                ],
                vec![IsItem::new(ItemAmount::new(
                    2.0,
                    MagneticCoil.to_string(),
                ))],
            )],
        );
        res_hash.insert(MagneticCoil, magentic_coil);
        let glass: Item = Item::new(
            &Glass.to_string(),
            vec![ManFac::Furnace],
            vec![Recipe::new(
                2.0,
                vec![IsItem::new(ItemAmount::new(2.0, Stone.to_string()))],
                vec![IsItem::new(ItemAmount::new(1.0, Glass.to_string()))],
            )],
        );
        res_hash.insert(Glass, glass);
        let diamond: Item = Item::new(
            &Diamond.to_string(),
            vec![ManFac::Furnace],
            vec![
                Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(
                        1.0,
                        EnergeticGraphite.to_string(),
                    ))],
                    vec![IsItem::new(ItemAmount::new(1.0, Diamond.to_string()))],
                ),
                Recipe::new(
                    1.5,
                    vec![IsItem::new(ItemAmount::new(
                        1.0,
                        KimberliteOre.to_string(),
                    ))],
                    vec![IsItem::new(ItemAmount::new(2.0, Diamond.to_string()))],
                ),
            ],
        );
        res_hash.insert(Diamond, diamond);
        let crystal_silicon: Item = Item::new(
            &CrystalSilicon.to_string(),
            vec![ManFac::Furnace],
            vec![
                Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(
                        1.0,
                        CrystalSilicon.to_string(),
                    ))],
                    vec![IsItem::new(ItemAmount::new(
                        1.0,
                        CrystalSilicon.to_string(),
                    ))],
                ),
                Recipe::new(
                    1.5,
                    vec![IsItem::new(ItemAmount::new(
                        1.0,
                        FractalSilicon.to_string(),
                    ))],
                    vec![IsItem::new(ItemAmount::new(
                        2.0,
                        CrystalSilicon.to_string(),
                    ))],
                ),
            ],
        );
        res_hash.insert(CrystalSilicon, crystal_silicon);
        let steel: Item = Item::new(
            &Steel.to_string(),
            vec![ManFac::Furnace],
            vec![Recipe::new(
                3.0,
                vec![IsItem::new(ItemAmount::new(
                    3.0,
                    IronIngot.to_string(),
                ))],
                vec![IsItem::new(ItemAmount::new(1.0, Steel.to_string()))],
            )],
        );
        res_hash.insert(Steel, steel);
        tohash!(
            res_hash,
            titanium_alloy,
            TitaniumAlloy,
            item!(
                TitaniumAlloy,
                (Furnace),
                (recipe!(
                    12.0,
                    (
                        recitem!(8.0, "Sulfuric Acid"),
                        recitem!(4.0, "Steel"),
                        recitem!(4.0, "Titanium Ingot")
                    ),
                    (recitem!(4.0, "Titanium Alloy"))
                ))
            )
        );
        let circuit_board: Item = Item::new(
            "Circuit Board",
            vec![ManFac::Assembler],
            vec![Recipe::new(
                1.0,
                vec![
                    IsItem::new(ItemAmount::new(1.0, String::from("Copper Ingot"))),
                    IsItem::new(ItemAmount::new(2.0, String::from("Iron Ingot"))),
                ],
                vec![IsItem::new(ItemAmount::new(
                    2.0,
                    String::from("Circuit Board"),
                ))],
            )],
        );
        res_hash.insert(CircuitBoard, circuit_board);
        let prism: Item = Item::new(
            "Prism",
            vec![ManFac::Furnace],
            vec![Recipe::new(
                2.0,
                vec![IsItem::new(ItemAmount::new(3.0, String::from("Glass")))],
                vec![IsItem::new(ItemAmount::new(2.0, String::from("Prism")))],
            )],
        );
        res_hash.insert(Prism, prism);
        let electric_motor: Item = Item::new(
            "Electric Motor",
            vec![ManFac::Assembler],
            vec![Recipe::new(
                2.0,
                vec![
                    IsItem::new(ItemAmount::new(1.0, String::from("Magnetic Coil"))),
                    IsItem::new(ItemAmount::new(1.0, String::from("Gear"))),
                    IsItem::new(ItemAmount::new(2.0, String::from("Iron Ingot"))),
                ],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    String::from("Electric Motor"),
                ))],
            )],
        );
        res_hash.insert(ElectricMotor, electric_motor);
        let microcrystalline_component: Item = Item::new(
            "Microcrystalline Component",
            vec![ManFac::Furnace],
            vec![Recipe::new(
                2.0,
                vec![
                    IsItem::new(ItemAmount::new(1.0, String::from("Copper Ingot"))),
                    IsItem::new(ItemAmount::new(2.0, String::from("High-purity Silicon"))),
                ],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    String::from("Microcrystalline Component"),
                ))],
            )],
        );
        res_hash.insert(
            MicrocrystallineComponent,
            microcrystalline_component,
        );
        gear = Item::new(
            "Gear",
            vec![ManFac::Assembler],
            vec![Recipe::new(
                1.0,
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    String::from("Iron Ingot"),
                ))],
                vec![IsItem::new(ItemAmount::new(1.0, String::from("Gear")))],
            )],
        );
        res_hash.insert(Gear, gear);
        let plasma_exciter: Item = Item::new(
            "Plasma Exciter",
            vec![ManFac::Assembler],
            vec![Recipe::new(
                2.0,
                vec![
                    IsItem::new(ItemAmount::new(2.0, String::from("Prism"))),
                    IsItem::new(ItemAmount::new(4.0, String::from("Magnetic Coil"))),
                ],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    String::from("Plasma Exciter"),
                ))],
            )],
        );
        res_hash.insert(PlasmaExciter, plasma_exciter);
        let photon_combiner: Item = Item::new(
            "Photon Combiner",
            vec![ManFac::Assembler],
            vec![
                Recipe::new(
                    3.0,
                    vec![
                        IsItem::new(ItemAmount::new(1.0, String::from("Circuit Board"))),
                        IsItem::new(ItemAmount::new(2.0, String::from("Prism"))),
                    ],
                    vec![IsItem::new(ItemAmount::new(
                        1.0,
                        String::from("Photon Combiner"),
                    ))],
                ),
                Recipe::new(
                    3.0,
                    vec![
                        IsItem::new(ItemAmount::new(1.0, String::from("Circuit Board"))),
                        IsItem::new(ItemAmount::new(2.0, String::from("Grating Crystal"))),
                    ],
                    vec![IsItem::new(ItemAmount::new(
                        1.0,
                        String::from("Photon Combiner"),
                    ))],
                ),
            ],
        );
        res_hash.insert(PhotonCombiner, photon_combiner);
        let electromagentic_turbine: Item = Item::new(
            "Electromagnetic Turbine",
            vec![ManFac::Assembler],
            vec![Recipe::new(
                2.0,
                vec![
                    IsItem::new(ItemAmount::new(2.0, String::from("Magnetic Coil"))),
                    IsItem::new(ItemAmount::new(2.0, String::from("Electric Motor"))),
                ],
                vec![IsItem::new(ItemAmount::new(
                    1.0,
                    String::from("Electromagnetic Turbine"),
                ))],
            )],
        );
        res_hash.insert(
            ElectromagenticTurbine,
            electromagentic_turbine,
        );
        let processor: Item = Item::new(
            "Processor",
            vec![ManFac::Assembler],
            vec![Recipe::new(
                3.0,
                vec![
                    IsItem::new(ItemAmount::new(
                        2.0,
                        String::from("Microcrystalline Component"),
                    )),
                    IsItem::new(ItemAmount::new(2.0, String::from("Circuit Board"))),
                ],
                vec![IsItem::new(ItemAmount::new(1.0, String::from("Processor")))],
            )],
        );
        res_hash.insert(Processor, processor);
        let engine: Item = Item::new(
            "Engine",
            vec![ManFac::Assembler],
            vec![Recipe::new(
                3.0,
                vec![
                    IsItem::new(ItemAmount::new(2.0, String::from("Copper Ingot"))),
                    IsItem::new(ItemAmount::new(1.0, String::from("Magnetic Coil"))),
                ],
                vec![IsItem::new(ItemAmount::new(1.0, String::from("Engine")))],
            )],
        );
        res_hash.insert(Engine, engine);
        let thruster: Item = item!(
            Thruster,
            (Assembler),
            (recipe!(
                4.0,
                (recitem!(3.0, "Copper Ingot")),
                (recitem!(2.0, "Steel"))
            ))
        );
        res_hash.insert(Thruster, thruster);
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
            ReinforcedThruster,
            item!(
                ReinforcedThruster,
                (Assembler),
                (recipe!(
                    6.0,
                    (
                        recitem!(5.0, "Electromagnetic Turbine"),
                        recitem!(5.0, "Titanium Alloy")
                    ),
                    (recitem!(4.0, "Reinforced Thruster"))
                ))
            )
        );
        tohash!(
            res_hash,
            super_magentic_ring,
            SuperMagenticRing,
            item!(
                SuperMagenticRing,
                (Assembler),
                (recipe!(
                    3.0,
                    (
                        recitem!(1.0, "Energetic Graphite"),
                        recitem!(3.0, "Magnet"),
                        recitem!(2.0, "Electromagnetic Turbine")
                    ),
                    (recitem!(1.0, "Super-magnetic Ring"))
                ))
            )
        );
        tohash!(
            res_hash,
            particle_container,
            ParticleContainer,
            item!(
                ParticleContainer,
                (Assembler),
                (
                    recipe!(
                        4.0,
                        (
                            recitem!(2.0, "Graphene"),
                            recitem!(2.0, "Copper Ingot"),
                            recitem!(2.0, "Electromagnetic Turbine")
                        ),
                        (recitem!(1.0, "Particle Container"))
                    ),
                    recipe!(
                        4.0,
                        (recitem!(2.0, "Copper Ingot")),
                        (recitem!(10.0, "Unipolar Magnet"))
                    )
                )
            )
        );
        tohash!(
            res_hash,
            plastic,
            Plastic,
            item!(
                Plastic,
                (ChemicalPlant),
                (recipe!(
                    3.0,
                    (
                        recitem!(1.0, "Energetic Graphite"),
                        recitem!(2.0, "Refined Oil")
                    ),
                    (recitem!(1.0, "Plastic"))
                ))
            )
        );
        tohash!(
            res_hash,
            organic_crystal,
            OrganicCrystal,
            item!(
                OrganicCrystal,
                (ChemicalPlant),
                (
                    recipe!(
                        6.0,
                        (
                            recitem!(1.0, "Water"),
                            recitem!(1.0, "Refined Oil"),
                            recitem!(2.0, "Plastic")
                        ),
                        (recitem!(1.0, "Organic Crystal"))
                    ),
                    recipe!(
                        6.0,
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
        // can't use macros because of the declaration earlier
        graphene = Item::new(
            "Graphene",
            vec![ManFac::ChemicalPlant],
            vec![
                Recipe::new(
                    3.0,
                    vec![
                        IsItem::new(ItemAmount::new(1.0, String::from("Sulfuric Acid"))),
                        IsItem::new(ItemAmount::new(3.0, String::from("Energetic Graphite"))),
                    ],
                    vec![IsItem::new(ItemAmount::new(2.0, String::from("Graphene")))],
                ),
                Recipe::new(
                    2.0,
                    vec![IsItem::new(ItemAmount::new(2.0, String::from("Fire Ice")))],
                    vec![
                        IsItem::new(ItemAmount::new(1.0, String::from("Hydrogen"))),
                        IsItem::new(ItemAmount::new(2.0, String::from("Graphene"))),
                    ],
                ),
            ],
        );
        res_hash.insert(Graphene, graphene);
        tohash!(
            res_hash,
            annihilation_constraint_sphere,
            AnnihilationConstraintSphere,
            item!(
                AnnihilationConstraintSphere,
                (Assembler),
                (recipe!(
                    20.0,
                    (
                        recitem!(1.0, "Processor"),
                        recitem!(1.0, "Particle Container")
                    ),
                    (recitem!(1.0, "Annihilation Constraint Sphere"))
                ))
            )
        );
        tohash!(
            res_hash,
            strange_matter,
            StrangeMatter,
            item!(
                StrangeMatter,
                (MiniatureParticleCollider),
                (recipe!(
                    8.0,
                    (
                        recitem!(10.0, "Deuterium"),
                        recitem!(2.0, "Iron Ingot"),
                        recitem!(1.0, "Particle Container")
                    ),
                    (recitem!(1.0, "Strange Matter"))
                ))
            )
        );
        tohash!(
            res_hash,
            titanium_crystal,
            TitaniumCrystal,
            item!(
                TitaniumCrystal,
                (Assembler),
                (recipe!(
                    4.0,
                    (
                        recitem!(3.0, "Titanium Ingot"),
                        recitem!(1.0, "Organic Crystal")
                    ),
                    (recitem!(1.0, "Titanium Crystal"))
                ))
            )
        );
        tohash!(
            res_hash,
            carbon_nanotube,
            CarbonNanotube,
            item!(
                CarbonNanotube,
                (ChemicalPlant),
                (
                    recipe!(
                        4.0,
                        (recitem!(1.0, "Titanium Ingot"), recitem!(3.0, "Graphene")),
                        (recitem!(2.0, "Carbon Nanotube"))
                    ),
                    recipe!(
                        4.0,
                        (recitem!(6.0, "Stalagmite Crystal")),
                        (recitem!(2.0, "Carbon Nanotube"))
                    )
                )
            )
        );
        tohash!(
            res_hash,
            particle_broadband,
            ParticleBroadband,
            item!(
                ParticleBroadband,
                (Assembler),
                (recipe!(
                    8.0,
                    (
                        recitem!(1.0, "Plastic"),
                        recitem!(2.0, "Crystal Silicon"),
                        recitem!(2.0, "Carbon Nanotube")
                    ),
                    (recitem!(1.0, "Particle Broadband"))
                ))
            )
        );
        tohash!(
            res_hash,
            casimir_crystal,
            CasimirCrystal,
            item!(
                CasimirCrystal,
                (Assembler),
                (
                    recipe!(
                        4.0,
                        (
                            recitem!(12.0, "Hydrogen"),
                            recitem!(2.0, "Graphene"),
                            recitem!(1.0, "Titanium Crystal")
                        ),
                        (recitem!(1.0, "Casimir Crystal"))
                    ),
                    recipe!(
                        4.0,
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
        tohash!(
            res_hash,
            titanium_glass,
            TitaniumGlass,
            item!(
                TitaniumGlass,
                (Assembler),
                (recipe!(
                    5.0,
                    (
                        recitem!(2.0, "Water"),
                        recitem!(2.0, "Titanium Ingot"),
                        recitem!(2.0, "Glass")
                    ),
                    (recitem!(2.0, "Titanium Glass"))
                ))
            )
        );
        tohash!(
            res_hash,
            plane_filter,
            PlaneFilter,
            item!(
                PlaneFilter,
                (Assembler),
                (recipe!(
                    12.0,
                    (
                        recitem!(2.0, "Titanium Glass"),
                        recitem!(1.0, "Casimir Crystal")
                    ),
                    (recitem!(1.0, "Plane Filter"))
                ))
            )
        );
        tohash!(
            res_hash,
            quantum_chip,
            QuantumChip,
            item!(
                QuantumChip,
                (Assembler),
                (recipe!(
                    6.0,
                    (recitem!(2.0, "Plane Filter"), recitem!(2.0, "Processor")),
                    (recitem!(1.0, "Quantum Chip"))
                ))
            )
        );

        tohash!(
            res_hash,
            combustible_unit,
            CombustibleUnit,
            item!(
                CombustibleUnit,
                (Assembler),
                (recipe!(
                    3.0,
                    (recitem!(3.0, "Coal")),
                    (recitem!(1.0, "Combustible Unit"))
                ))
            )
        );
        // logistics
        tohash!(
            res_hash,
            logisitcs_bot,
            LogisticsBot,
            item!(
                LogisticsBot,
                (Assembler),
                (recipe!(
                    2.0,
                    (
                        recitem!(1.0, "Processor"),
                        recitem!(1.0, "Engine"),
                        recitem!(2.0, "Iron Ingot")
                    ),
                    (recitem!(1.0, "Logistics Bot"))
                ))
            )
        );
        tohash!(
            res_hash,
            logisitcs_drone,
            LogisticsDrone,
            item!(
                LogisticsDrone,
                (Assembler),
                (recipe!(
                    4.0,
                    (
                        recitem!(2.0, "Thruster"),
                        recitem!(2.0, "Processor"),
                        recitem!(5.0, "Iron Ingot")
                    ),
                    (recitem!(1.0, "Logistics Drone"))
                ))
            )
        );
        tohash!(
            res_hash,
            interstellar_logistics_vessel,
            InterstellarLogisticsVessel,
            item!(
                InterstellarLogisticsVessel,
                (Assembler),
                (recipe!(
                    6.0,
                    (
                        recitem!(2.0, "Reinforced Thruster"),
                        recitem!(10.0, "Processor"),
                        recitem!(10.0, "Titanium Alloy")
                    ),
                    (recitem!(1.0, "Interstellar Logistics Vessel"))
                ))
            )
        );
        tohash!(
            res_hash,
            graviton_lens,
            GravitonLens,
            item!(
                GravitonLens,
                (Assembler),
                (recipe!(
                    6.0,
                    (recitem!(1.0, "Strange Matter"), recitem!(4.0, "Diamond")),
                    (recitem!(1.0, "Graviton Lens"))
                ))
            )
        );
        tohash!(
            res_hash,
            space_warper,
            SpaceWarper,
            item!(
                SpaceWarper,
                (Assembler),
                (
                    recipe!(
                        10.0,
                        (recitem!(1.0, "Graviton Lens")),
                        (recitem!(1.0, "Space Warper"))
                    ),
                    recipe!(
                        10.0,
                        (recitem!(1.0, "Gravity Matrix")),
                        (recitem!(8.0, "Space Warper"))
                    )
                )
            )
        );
        tohash!(
            res_hash,
            foundation,
            Foundation,
            item!(
                Foundation,
                (Assembler),
                (recipe!(
                    1.0,
                    (recitem!(1.0, "Steel"), recitem!(3.0, "Stone Brick")),
                    (recitem!(1.0, "Foundation"))
                ))
            )
        );
        // proliferator
        tohash!(
            res_hash,
            proliferator_mki,
            ProliferatorMki,
            item!(
                ProliferatorMki,
                (Assembler),
                (recipe!(
                    0.5,
                    (recitem!(1.0, "Coal")),
                    (recitem!(1.0, "Proliferator Mk.I"))
                ))
            )
        );
        tohash!(
            res_hash,
            proliferator_mkii,
            ProliferatorMkii,
            item!(
                ProliferatorMkii,
                (Assembler),
                (recipe!(
                    1.0,
                    (recitem!(1.0, "Diamond"), recitem!(2.0, "Proliferator Mk.I")),
                    (recitem!(1.0, "Proliferator Mk.II"))
                ))
            )
        );
        tohash!(
            res_hash,
            proliferator_mkiii,
            ProliferatorMkiii,
            item!(
                ProliferatorMkiii,
                (Assembler),
                (recipe!(
                    1.0,
                    (
                        recitem!(1.0, "Diamond"),
                        recitem!(2.0, "Proliferator Mk.II")
                    ),
                    (recitem!(1.0, "Proliferator Mk.III"))
                ))
            )
        );
        // fuel rods
        tohash!(
            res_hash,
            hydrogen_fuel_rod,
            HydrogenFuelRod,
            item!(
                HydrogenFuelRod,
                (Assembler),
                (recipe!(
                    6.0,
                    (recitem!(10.0, "Hydrogen"), recitem!(1.0, "Titanium Ingot")),
                    (recitem!(2.0, "Hydrogen Fuel Rod"))
                ))
            )
        );
        tohash!(
            res_hash,
            deuterium_fuel_rod,
            DeuteriumFuelRod,
            item!(
                DeuteriumFuelRod,
                (Assembler),
                (recipe!(
                    12.0,
                    (
                        recitem!(1.0, "Super-magnetic Ring"),
                        recitem!(20.0, "Deuterium"),
                        recitem!(1.0, "Titanium Alloy")
                    ),
                    (recitem!(2.0, "Deuterium Fuel Rod"))
                ))
            )
        );
        tohash!(
            res_hash,
            antimatter_fuel_rod,
            AntimatterFuelRod,
            item!(
                AntimatterFuelRod,
                (Assembler),
                (recipe!(
                    24.0,
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
        tohash!(
            res_hash,
            strange_anihilation_fuel_rod,
            StrangeAnnihilationFuelRod,
            item!(
                StrangeAnnihilationFuelRod,
                (Assembler),
                (recipe!(
                    32.0,
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
        // dyson shpere building
        tohash!(
            res_hash,
            solar_sail,
            SolarSail,
            item!(
                SolarSail,
                (Assembler),
                (recipe!(
                    4.0,
                    (recitem!(1.0, "Photon Combiner"), recitem!(1.0, "Graphene")),
                    (recitem!(2.0, "Solar Sail"))
                ))
            )
        );
        tohash!(
            res_hash,
            frame_material,
            FrameMaterial,
            item!(
                FrameMaterial,
                (Assembler),
                (recipe!(
                    6.0,
                    (
                        recitem!(1.0, "High-purity Silicon"),
                        recitem!(1.0, "Titanium Alloy"),
                        recitem!(4.0, "Carbon Nanotube")
                    ),
                    (recitem!(1.0, "Frame Material"))
                ))
            )
        );
        tohash!(
            res_hash,
            dyson_sphere_component,
            DysonSphereComponent,
            item!(
                DysonSphereComponent,
                (Assembler),
                (recipe!(
                    8.0,
                    (
                        recitem!(3.0, "Processor"),
                        recitem!(3.0, "Solar Sail"),
                        recitem!(3.0, "Frame Material")
                    ),
                    (recitem!(1.0, "Dyson Sphere Component"))
                ))
            )
        );
        tohash!(
            res_hash,
            small_carrier_rocket,
            SmallCarrierRocket,
            item!(
                SmallCarrierRocket,
                (Assembler),
                (recipe!(
                    6.0,
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
        tohash!(
            res_hash,
            electromagnetic_matrix,
            ElectromagneticMatrix,
            item!(
                ElectromagneticMatrix,
                (Lab),
                (recipe!(
                    3.0,
                    (
                        recitem!(1.0, "Circuit Board"),
                        recitem!(1.0, "Magnetic Coil")
                    ),
                    (recitem!(1.0, "Electromagnetic Matrix"))
                ))
            )
        );
        tohash!(
            res_hash,
            energy_matrix,
            EnergyMatrix,
            item!(
                EnergyMatrix,
                (Lab),
                (recipe!(
                    6.0,
                    (
                        recitem!(2.0, "Hydrogen"),
                        recitem!(2.0, "Energetic Graphite")
                    ),
                    (recitem!(1.0, "Energy Matrix"))
                ))
            )
        );
        tohash!(
            res_hash,
            structure_matrix,
            StructureMatrix,
            item!(
                StructureMatrix,
                (Lab),
                (recipe!(
                    8.0,
                    (recitem!(1.0, "Titanium Crystal"), recitem!(1.0, "Diamond")),
                    (recitem!(1.0, "Structure Matrix"))
                ))
            )
        );
        tohash!(
            res_hash,
            information_matrix,
            InformationMatrix,
            item!(
                InformationMatrix,
                (Lab),
                (recipe!(
                    10.0,
                    (
                        recitem!(1.0, "Particle Broadband"),
                        recitem!(2.0, "Processor")
                    ),
                    (recitem!(1.0, "Information Matrix"))
                ))
            )
        );
        tohash!(
            res_hash,
            gravity_matrix,
            GravityMatrix,
            item!(
                GravityMatrix,
                (Lab),
                (recipe!(
                    24.0,
                    (
                        recitem!(1.0, "Quantum Chip"),
                        recitem!(1.0, "Graviton Lens")
                    ),
                    (recitem!(2.0, "Gravity Matrix"))
                ))
            )
        );
        tohash!(
            res_hash,
            universe_matrix,
            UniverseMatrix,
            item!(
                UniverseMatrix,
                (Lab),
                (recipe!(
                    15.0,
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
        tohash!(
            res_hash,
            magnum_ammo_box,
            MagnumAmmoBox,
            item!(
                MagnumAmmoBox,
                (Assembler),
                (recipe!(
                    1.0,
                    (recitem!(1.0, "Copper Ingot")),
                    (recitem!(1.0, "Magnum Ammo Box"))
                ))
            )
        );
        tohash!(
            res_hash,
            titanium_ammo_box,
            TitaniumAmmoBox,
            item!(
                TitaniumAmmoBox,
                (Assembler),
                (recipe!(
                    2.0,
                    (
                        recitem!(2.0, "Titanium Ingot"),
                        recitem!(1.0, "Magnum Ammo Box")
                    ),
                    (recitem!(1.0, "Titanium Ammo Box"))
                ))
            )
        );
        tohash!(
            res_hash,
            superalloy_ammo_box,
            SuperalloyAmmoBox,
            item!(
                SuperalloyAmmoBox,
                (Assembler),
                (recipe!(
                    3.0,
                    (
                        recitem!(1.0, "Titanium Alloy"),
                        recitem!(1.0, "Titanium Ammo Box")
                    ),
                    (recitem!(1.0, "Superalloy Ammo Box"))
                ))
            )
        );
        tohash!(
            res_hash,
            explosive_unit,
            ExplosiveUnit,
            item!(
                ExplosiveUnit,
                (ChemicalPlant),
                (recipe!(
                    6.0,
                    (
                        recitem!(1.0, "Sulfuric Acid"),
                        recitem!(2.0, "Plastic"),
                        recitem!(2.0, "Combustible Unit")
                    ),
                    (recitem!(2.0, "Explosive Unit"))
                ))
            )
        );
        tohash!(
            res_hash,
            crystal_explosive_unit,
            CrystalExplosiveUnit,
            item!(
                CrystalExplosiveUnit,
                (ChemicalPlant),
                (recipe!(
                    24.0,
                    (
                        recitem!(8.0, "Crystal Silicon"),
                        recitem!(1.0, "Casimir Crystal"),
                        recitem!(8.0, "Explosive Unit")
                    ),
                    (recitem!(8.0, "Crystal Explosive Unit"))
                ))
            )
        );
        tohash!(
            res_hash,
            missile_set,
            MissileSet,
            item!(
                MissileSet,
                (Assembler),
                (recipe!(
                    2.0,
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
        tohash!(
            res_hash,
            supersonic_missile_set,
            SupersonicMissileSet,
            item!(
                SupersonicMissileSet,
                (Assembler),
                (recipe!(
                    4.0,
                    (
                        recitem!(2.0, "Thruster"),
                        recitem!(4.0, "Explosive Unit"),
                        recitem!(4.0, "Processor"),
                        recitem!(2.0, "Missile Set")
                    ),
                    (recitem!(2.0, "Supersonic Missile Set"))
                ))
            )
        );
        tohash!(
            res_hash,
            gravity_missile_set,
            GravityMissileSet,
            item!(
                GravityMissileSet,
                (Assembler),
                (recipe!(
                    6.0,
                    (
                        recitem!(3.0, "Strange Matter"),
                        recitem!(6.0, "Crystal Explosive Unit"),
                        recitem!(3.0, "Supersonic Missile Set")
                    ),
                    (recitem!(3.0, "Gravity Missile Set"))
                ))
            )
        );
        tohash!(
            res_hash,
            shell_set,
            ShellSet,
            item!(
                ShellSet,
                (Assembler),
                (recipe!(
                    1.5,
                    (
                        recitem!(2.0, "Explosive Unit"),
                        recitem!(9.0, "Copper Ingot")
                    ),
                    (recitem!(1.0, "Shell Set"))
                ))
            )
        );
        tohash!(
            res_hash,
            high_explosive_shell_set,
            HighExplosiveShellSet,
            item!(
                HighExplosiveShellSet,
                (Assembler),
                (recipe!(
                    3.0,
                    (
                        recitem!(2.0, "Explosive Unit"),
                        recitem!(6.0, "Titanium Ingot"),
                        recitem!(1.0, "Shell Set")
                    ),
                    (recitem!(1.0, "High-Explosive Shell Set"))
                ))
            )
        );
        tohash!(
            res_hash,
            crystal_shell_set,
            CrystalShellSet,
            item!(
                CrystalShellSet,
                (Assembler),
                (recipe!(
                    6.0,
                    (
                        recitem!(2.0, "Crystal Explosive Unit"),
                        recitem!(3.0, "Titanium Ingot"),
                        recitem!(1.0, "High-Explosive Shell Set")
                    ),
                    (recitem!(1.0, "Crystal Shell Set"))
                ))
            )
        );
        tohash!(
            res_hash,
            plasma_capsule,
            PlasmaCapsule,
            item!(
                PlasmaCapsule,
                (Assembler),
                (recipe!(
                    2.0,
                    (
                        recitem!(10.0, "Deuterium"),
                        recitem!(2.0, "Magnet"),
                        recitem!(1.0, "Graphene")
                    ),
                    (recitem!(1.0, "Plasma Capsule"))
                ))
            )
        );
        tohash!(
            res_hash,
            anitmatter_capsule,
            AntimatterCapsule,
            item!(
                AntimatterCapsule,
                (Assembler),
                (recipe!(
                    2.0,
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
        tohash!(
            res_hash,
            jamming_capsule,
            JammingCapsule,
            item!(
                JammingCapsule,
                (Assembler),
                (recipe!(
                    2.0,
                    (
                        recitem!(3.0, "Hydrogen"),
                        recitem!(1.0, "Plasma Exciter"),
                        recitem!(1.0, "Electromagnetic Turbine")
                    ),
                    (recitem!(1.0, "Jamming Capsule"))
                ))
            )
        );
        tohash!(
            res_hash,
            suppressing_capsule,
            SupressingCapsule,
            item!(
                SupressingCapsule,
                (Assembler),
                (recipe!(
                    8.0,
                    (
                        recitem!(2.0, "Glass"),
                        recitem!(1.0, "Super-magnetic Ring"),
                        recitem!(2.0, "Jamming Capsule")
                    ),
                    (recitem!(2.0, "Suppressing Capsule"))
                ))
            )
        );
        // drones and ships
        tohash!(
            res_hash,
            prototype,
            Prototype,
            item!(
                Prototype,
                (Assembler),
                (recipe!(
                    2.0,
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
        tohash!(
            res_hash,
            precision_drone,
            PrecisionDrone,
            item!(
                PrecisionDrone,
                (Assembler),
                (recipe!(
                    4.0,
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
        tohash!(
            res_hash,
            attack_drone,
            AttackDrone,
            item!(
                AttackDrone,
                (Assembler),
                (recipe!(
                    4.0,
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
        tohash!(
            res_hash,
            corvette,
            Corvette,
            item!(
                Corvette,
                (Assembler),
                (recipe!(
                    5.0,
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
        tohash!(
            res_hash,
            destroyer,
            Destroyer,
            item!(
                Destroyer,
                (Assembler),
                (recipe!(
                    8.0,
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
}

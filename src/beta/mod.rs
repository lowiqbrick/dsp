// file for defining all items and recipes
// import everything from essentials.rs
mod doc;
mod essentials;
pub mod items;
mod items_get;

use crate::beta::essentials::*;
use crate::beta::items::ItemType;
use std::collections::HashMap;

/// checks fo the environment key words that tell the function to display the
/// documentation for the program
///
/// # Return
///
/// returns true if the help message was displayed and false otherwise
fn display_help(args: &mut [String]) -> bool {
    for argument in args.iter_mut() {
        if argument.as_str() == "-h" || argument.as_mut_str() == "--help" {
            doc::print_help();
            return true;
        }
    }
    false
}

fn process_env_args(
    args: &mut [String],
    settings: &mut ProgramInfo,
    item_hashmap: &HashMap<ItemType, Item>,
) {
    let mut iterator = args.iter();
    let mut status = ArgState::Default;
    while let Some(argument) = iterator.next() {
        match argument.as_str() {
            // squash the output
            "-m" | "--merge" => settings.merge = true,
            // assume basics
            "-b" | "--basics" => settings.assume_basics = true,
            // proliferation
            "-p" | "--proliferation" => status = ArgState::ProlifLevel,
            "proliferator=1" => settings.proliferators = Proliferator::MKone,
            "proliferator=2" => settings.proliferators = Proliferator::MKtwo,
            "proliferator=3" => settings.proliferators = Proliferator::MKthree,
            // chemlab level
            "-c" | "--chemlab" => status = ArgState::ChemLabLevel,
            "chemlab=1" => settings.chemlab = ChemLabMK::Lab,
            "chemlab=2" => settings.chemlab = ChemLabMK::QuantumLab,
            // smelter
            "-s" | "--smelter" => status = ArgState::FurnaceLevel,
            "smelter=1" => settings.smelter = SmelterMK::Arc,
            "smelter=2" => settings.smelter = SmelterMK::Plane,
            "smelter=3" => settings.smelter = SmelterMK::Negentropy,
            // assembler
            "-a" | "--assembler" => status = ArgState::AssemblerLevel,
            "assembler=1" => settings.assembler = AssemblerMK::One,
            "assembler=2" => settings.assembler = AssemblerMK::Two,
            "assembler=3" => settings.assembler = AssemblerMK::Three,
            "assembler=4" => settings.assembler = AssemblerMK::Four,
            // research lab
            "-l" | "--lab" => status = ArgState::LabLevel,
            "lab=1" => settings.lab = LabMK::MatrixLab,
            "lab=2" => settings.lab = LabMK::SelfEvolutionLab,
            // no proliferation
            "-n" | "--noproliferation" => status = ArgState::NoProliferation,
            // additional items
            "-i" | "--items" => status = ArgState::AdditionalItems,
            // set the recipe
            "-r" | "--recipe" => status = ArgState::ItemRecipe,
            // display the documentation
            "-h" | "--help" => doc::print_help(),
            // only show the item requested and not the entire chain
            "-t" | "--this-item" => settings.this_item_only = true,
            // catch all
            _ => {
                // process the options demanding more elaborate processing
                match status {
                    // proliferation
                    ArgState::ProlifLevel => {
                        match argument.as_str() {
                            "1" => settings.proliferators = Proliferator::MKone,
                            "2" => settings.proliferators = Proliferator::MKtwo,
                            "3" => settings.proliferators = Proliferator::MKthree,
                            _ => eprintln!(
                                "invalid proliferation level was supplied (\"1\", \"2\" and \"3\" are valid; \"{}\" was supplied)", argument),
                        }
                        status = ArgState::Default;
                    }
                    // chemlab level
                    ArgState::ChemLabLevel => {
                        match argument.as_str() {
                            "1" => settings.chemlab = ChemLabMK::Lab,
                            "2" => settings.chemlab = ChemLabMK::QuantumLab,
                            _ => eprintln!(
                                "invalid chemlab level was supplied (\"1\", \"2\" are valid; \"{}\" was supplied)", argument),
                        }
                        status = ArgState::Default;
                    }
                    // smelter level
                    ArgState::FurnaceLevel => {
                        match argument.as_str() {
                            "1" => settings.smelter = SmelterMK::Arc,
                            "2" => settings.smelter = SmelterMK::Plane,
                            "3" => settings.smelter = SmelterMK::Negentropy,
                            _ => eprintln!(
                                "invalid smelter level was supplied (\"1\", \"2\" and \"3\" are valid; \"{}\" was supplied)", argument),
                        }
                        status = ArgState::Default;
                    }
                    // assembler level
                    ArgState::AssemblerLevel => {
                        match argument.as_str() {
                            "1" => settings.assembler = AssemblerMK::One,
                            "2" => settings.assembler = AssemblerMK::Two,
                            "3" => settings.assembler = AssemblerMK::Three,
                            "4" => settings.assembler = AssemblerMK::Four,
                            _ => eprintln!(
                                "invalid assembler level was supplied (\"1\", \"2\", \"3\" and \"4\"are valid; \"{}\" was supplied)", argument),
                        }
                        status = ArgState::Default;
                    }
                    // research lab level
                    ArgState::LabLevel => {
                        match argument.as_str() {
                            "1" => settings.lab = LabMK::MatrixLab,
                            "2" => settings.lab = LabMK::SelfEvolutionLab,
                            _ => eprintln!(
                                "invalid lab level was supplied (\"1\", \"2\" are valid; \"{}\" was supplied)", argument),
                        }
                        status = ArgState::Default;
                    }
                    // disable proliferation for specific items and downward
                    ArgState::NoProliferation => {
                        if let Ok(item) = argument.parse::<ItemType>() {
                            if item_hashmap.contains_key(&item) {
                                settings.no_proliferation.push(item);
                            } else {
                                eprintln!("{} isn't a valid item", argument);
                            }
                        } else {
                            eprintln!("{} isn't a valid item", argument);
                        }
                        status = ArgState::Default;
                    }
                    // add additional production for specific items in a crafting chain
                    // needs next element in for loop
                    ArgState::AdditionalItems => {
                        // save current and next arguments for processing later
                        let item_name = argument.clone();
                        let quantity = match iterator.next() {
                            Some(success) => success,
                            None => {
                                eprint!("no number specified after {}", item_name);
                                break;
                            }
                        };
                        if let Ok(item) = item_name.parse::<ItemType>() {
                            if item_hashmap.contains_key(&item) && quantity.parse::<f32>().is_ok() {
                                settings.additional_items.push(ItemAmount::new(
                                    quantity.parse::<f32>().expect("provided invalid number"),
                                    item,
                                ));
                            } else {
                                eprintln!("{} isn't a valid item", item_name);
                            }
                        } else {
                            eprintln!("{} isn't a valid item", item_name);
                        }
                        status = ArgState::Default;
                    }
                    // set the recipe for a given item
                    // needs next element in for loop
                    ArgState::ItemRecipe => {
                        // save current and next arguments for processing later
                        let item_name = argument.clone();
                        let quantity = match iterator.next() {
                            Some(success) => success,
                            None => {
                                eprintln!("no number specified after {}", item_name);
                                break;
                            }
                        };
                        if let Ok(item) = item_name.parse::<ItemType>() {
                            if let Some(t) = item_hashmap.get(&item) {
                                // check if recipe number given makes sense
                                match quantity.parse::<u8>() {
                                    Ok(recipe_num) => {
                                        let recipe_tot_num = t.recipes.len();
                                        // add item to vector if recipe number is a valid index for the vector
                                        if recipe_tot_num > recipe_num as usize {
                                            settings
                                                .item_recipe
                                                .push(ItemAmount::new(recipe_num as f32, item));
                                        } else {
                                            eprintln!(
                                                "recipe index {} out of range for {}",
                                                recipe_num, item_name
                                            );
                                        }
                                    }
                                    Err(_) => eprintln!("provided invalid number for recipe"),
                                }
                            } else {
                                eprintln!("{} isn't a valid item", item_name);
                            }
                        } else {
                            eprintln!("{} isn't a valid item", item_name);
                        }
                        status = ArgState::Default;
                    }
                    ArgState::Default => {
                        eprintln!("invalid argument {} won't be used", argument);
                    }
                }
            }
        }
    }
}

/// the main function of the beta of the game
pub fn beta(mut args: Vec<String>) {
    // declare the variables required for processing the inputs
    let args_len = args.len();
    // println!("args len: {}", args_len);
    let item_hashmap = items_get::get_items();
    // Default settings of the program
    let mut settings = ProgramInfo::new(
        EnumCombiner {
            proliferator: Proliferator::None,
            chemlab: ChemLabMK::Lab,
            smelter: SmelterMK::Arc,
            assembler: AssemblerMK::One,
            lab: LabMK::MatrixLab,
        },
        vec![],
        vec![],
        vec![],
        false,
        false,
        ItemAmount::new(0.0, ItemType::IronOre),
    );
    if display_help(&mut args) {
        return;
    }
    // sanity check
    if args_len < 2 {
        eprintln!("no arguments were given to the function");
        return;
    }
    // loop for processing the arguments
    // get the item to be produced
    let produced_item_type = match args[0].parse::<ItemType>() {
        Ok(item) if item_hashmap.contains_key(&item) => item,
        _ => {
            eprintln!("given item name {} isn't valid", args[0]);
            return;
        }
    };
    settings.produced_item = ItemAmount::new(
        args[1].parse::<f32>().expect("provided invalid number"),
        produced_item_type,
    );

    // remove the first two arguments
    args.remove(0);
    args.remove(0);
    // process the arguments
    process_env_args(&mut args, &mut settings, &item_hashmap);
    // Debugging only
    // println!("{:?}", settings);
    // call method on item
    let mut result_order = vec![];
    let mut result = HashMap::new();
    let prev_path = String::from("");
    let mut is_proliferated = true;
    if settings
        .no_proliferation
        .contains(&settings.produced_item.item_type)
    {
        is_proliferated = false;
    }
    match item_hashmap.get(&settings.produced_item.item_type) {
        // set off creation of the crafting chain
        Some(item) => {
            let twostrings = StringDuo {
                item_type: settings.produced_item.item_type,
                prev_path,
            };
            let twobools = BoolDuo {
                is_proliferated,
                is_first_item: true,
            };
            let mut ctx =
                CraftingContext::new(&item_hashmap, &settings, &mut result_order, &mut result);
            item.crafting_chain(
                &mut ctx,
                twostrings,
                settings.produced_item.amount,
                twobools,
            )
        }
        _ => {
            panic!("requested item not in crafting recipes");
        }
    }
    // Debugging only
    // println!("\nresult order vector: {:?}\n", result_order);
    // println!("result vector: {:?}\n", result);
    let vector_len = result_order.len();
    for (index, result_string) in result_order.iter().enumerate() {
        match result.get(result_string) {
            Some(result_match) => {
                println!("{}", result_match);
                if index + 1 < vector_len {
                    println!("----------------------------------------");
                }
            }
            None => {
                panic!("{} wasn't found in the item list", result_string);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::beta::{
        essentials::{
            BoolDuo, CraftingContext, ItemAmount, ItemResult, ManFac, ProgramInfo, StringDuo,
        },
        items::ItemType,
        items_get::get_items,
    };

    fn create_test_settings(amount: f32, item_type: ItemType) -> ProgramInfo {
        ProgramInfo {
            produced_item: ItemAmount { amount, item_type },
            basics: ProgramInfo::basics(),
            ..Default::default()
        }
    }

    fn crate_test_variables(
        amount: f32,
        item_type: ItemType,
        is_proliferated: bool,
    ) -> (
        StringDuo,
        f32,
        ProgramInfo,
        HashMap<String, ItemResult>,
        Vec<String>,
        BoolDuo,
    ) {
        (
            StringDuo {
                item_type,
                prev_path: "".to_string(),
            },
            amount,
            create_test_settings(amount, item_type),
            HashMap::new(),
            vec![],
            BoolDuo {
                is_proliferated,
                is_first_item: true,
            },
        )
    }

    /// the expected result for crating one electric motor every 2 seconds
    fn results_electric_motor() -> HashMap<String, ItemResult> {
        let mut correct_result = HashMap::new();
        correct_result.insert(
            "Electric Motor".to_string(),
            ItemResult {
                describer: "Electric Motor".to_string(),
                item_type: ItemType::ElectricMotor,
                target_rate: 0.5,
                num_station: 1.3333334,
                station: vec![ManFac::Assembler],
                requirements: vec![
                    ItemAmount {
                        amount: 0.5,
                        item_type: ItemType::MagneticCoil,
                    },
                    ItemAmount {
                        amount: 0.5,
                        item_type: ItemType::Gear,
                    },
                    ItemAmount {
                        amount: 1.0,
                        item_type: ItemType::IronIngot,
                    },
                ],
            },
        );
        correct_result.insert(
            "Iron Ingot -> Electric Motor".to_string(),
            ItemResult {
                describer: "Iron Ingot -> Electric Motor".to_string(),
                item_type: ItemType::IronIngot,
                target_rate: 1.0,
                num_station: 1.0,
                station: vec![ManFac::Furnace],
                requirements: vec![ItemAmount {
                    amount: 1.0,
                    item_type: ItemType::IronOre,
                }],
            },
        );
        correct_result.insert(
            "Iron Ore -> Iron Ingot -> Electric Motor".to_string(),
            ItemResult {
                describer: "Iron Ore -> Iron Ingot -> Electric Motor".to_string(),
                item_type: ItemType::IronOre,
                target_rate: 1.0,
                num_station: 0.0,
                station: vec![ManFac::Origin],
                requirements: vec![],
            },
        );
        correct_result.insert(
            "Gear -> Electric Motor".to_string(),
            ItemResult {
                describer: "Gear -> Electric Motor".to_string(),
                item_type: ItemType::Gear,
                target_rate: 0.5,
                num_station: 0.6666667,
                station: vec![ManFac::Assembler],
                requirements: vec![ItemAmount {
                    amount: 0.5,
                    item_type: ItemType::IronIngot,
                }],
            },
        );
        correct_result.insert(
            "Iron Ingot -> Gear -> Electric Motor".to_string(),
            ItemResult {
                describer: "Iron Ingot -> Gear -> Electric Motor".to_string(),
                item_type: ItemType::IronIngot,
                target_rate: 0.5,
                num_station: 0.5,
                station: vec![ManFac::Furnace],
                requirements: vec![ItemAmount {
                    amount: 0.5,
                    item_type: ItemType::IronOre,
                }],
            },
        );
        correct_result.insert(
            "Iron Ore -> Iron Ingot -> Gear -> Electric Motor".to_string(),
            ItemResult {
                describer: "Iron Ore -> Iron Ingot -> Gear -> Electric Motor".to_string(),
                item_type: ItemType::IronOre,
                target_rate: 0.5,
                num_station: 0.0,
                station: vec![ManFac::Origin],
                requirements: vec![],
            },
        );
        correct_result.insert(
            "Magnetic Coil -> Electric Motor".to_string(),
            ItemResult {
                describer: "Magnetic Coil -> Electric Motor".to_string(),
                item_type: ItemType::MagneticCoil,
                target_rate: 0.5,
                num_station: 0.33333334,
                station: vec![ManFac::Assembler],
                requirements: vec![
                    ItemAmount {
                        amount: 0.25,
                        item_type: ItemType::CopperIngot,
                    },
                    ItemAmount {
                        amount: 0.5,
                        item_type: ItemType::Magnet,
                    },
                ],
            },
        );
        correct_result.insert(
            "Magnet -> Magnetic Coil -> Electric Motor".to_string(),
            ItemResult {
                describer: "Magnet -> Magnetic Coil -> Electric Motor".to_string(),
                item_type: ItemType::Magnet,
                target_rate: 0.5,
                num_station: 0.75,
                station: vec![ManFac::Furnace],
                requirements: vec![ItemAmount {
                    amount: 0.5,
                    item_type: ItemType::IronOre,
                }],
            },
        );
        correct_result.insert(
            "Iron Ore -> Magnet -> Magnetic Coil -> Electric Motor".to_string(),
            ItemResult {
                describer: "Iron Ore -> Magnet -> Magnetic Coil -> Electric Motor".to_string(),
                item_type: ItemType::IronOre,
                target_rate: 0.5,
                num_station: 0.0,
                station: vec![ManFac::Origin],
                requirements: vec![],
            },
        );
        correct_result.insert(
            "Copper Ingot -> Magnetic Coil -> Electric Motor".to_string(),
            ItemResult {
                describer: "Copper Ingot -> Magnetic Coil -> Electric Motor".to_owned(),
                item_type: ItemType::CopperIngot,
                target_rate: 0.25,
                num_station: 0.25,
                station: vec![ManFac::Furnace],
                requirements: vec![ItemAmount {
                    amount: 0.25,
                    item_type: ItemType::CopperOre,
                }],
            },
        );
        correct_result.insert(
            "Copper Ore -> Copper Ingot -> Magnetic Coil -> Electric Motor".to_string(),
            ItemResult {
                describer: "Copper Ore -> Copper Ingot -> Magnetic Coil -> Electric Motor"
                    .to_string(),
                item_type: ItemType::CopperOre,
                target_rate: 0.25,
                num_station: 0.0,
                station: vec![ManFac::Origin],
                requirements: vec![],
            },
        );
        correct_result
    }

    fn results_electric_motor_double_rate() -> HashMap<String, ItemResult> {
        let mut multiplied_correct_result = results_electric_motor();
        for value in multiplied_correct_result.values_mut() {
            value.target_rate *= 2.0;
            value.num_station *= 2.0;
            for ingredient in value.requirements.iter_mut() {
                ingredient.amount *= 2.0;
            }
        }
        multiplied_correct_result
    }

    fn results_electric_motor_merged() -> HashMap<String, ItemResult> {
        let mut correct_result = HashMap::new();
        correct_result.insert(
            "Electric Motor".to_string(),
            ItemResult {
                describer: "Electric Motor".to_string(),
                item_type: ItemType::ElectricMotor,
                target_rate: 0.5,
                num_station: 1.3333334,
                station: vec![ManFac::Assembler],
                requirements: vec![
                    ItemAmount {
                        amount: 0.5,
                        item_type: ItemType::MagneticCoil,
                    },
                    ItemAmount {
                        amount: 0.5,
                        item_type: ItemType::Gear,
                    },
                    ItemAmount {
                        amount: 1.0,
                        item_type: ItemType::IronIngot,
                    },
                ],
            },
        );
        correct_result.insert(
            "Iron Ingot".to_string(),
            ItemResult {
                describer: "Iron Ingot".to_string(),
                item_type: ItemType::IronIngot,
                target_rate: 1.5,
                num_station: 1.5,
                station: vec![ManFac::Furnace],
                requirements: vec![ItemAmount {
                    amount: 1.5,
                    item_type: ItemType::IronOre,
                }],
            },
        );
        correct_result.insert(
            "Iron Ore".to_string(),
            ItemResult {
                describer: "Iron Ore".to_string(),
                item_type: ItemType::IronOre,
                target_rate: 2.0,
                num_station: 0.0,
                station: vec![ManFac::Origin],
                requirements: vec![],
            },
        );
        correct_result.insert(
            "Gear".to_string(),
            ItemResult {
                describer: "Gear".to_string(),
                item_type: ItemType::Gear,
                target_rate: 0.5,
                num_station: 0.6666667,
                station: vec![ManFac::Assembler],
                requirements: vec![ItemAmount {
                    amount: 0.5,
                    item_type: ItemType::IronIngot,
                }],
            },
        );
        correct_result.insert(
            "Magnetic Coil".to_string(),
            ItemResult {
                describer: "Magnetic Coil".to_string(),
                item_type: ItemType::MagneticCoil,
                target_rate: 0.5,
                num_station: 0.33333334,
                station: vec![ManFac::Assembler],
                requirements: vec![
                    ItemAmount {
                        amount: 0.25,
                        item_type: ItemType::CopperIngot,
                    },
                    ItemAmount {
                        amount: 0.5,
                        item_type: ItemType::Magnet,
                    },
                ],
            },
        );
        correct_result.insert(
            "Magnet".to_string(),
            ItemResult {
                describer: "Magnet".to_string(),
                item_type: ItemType::Magnet,
                target_rate: 0.5,
                num_station: 0.75,
                station: vec![ManFac::Furnace],
                requirements: vec![ItemAmount {
                    amount: 0.5,
                    item_type: ItemType::IronOre,
                }],
            },
        );
        correct_result.insert(
            "Copper Ingot".to_string(),
            ItemResult {
                describer: "Copper Ingot".to_owned(),
                item_type: ItemType::CopperIngot,
                target_rate: 0.25,
                num_station: 0.25,
                station: vec![ManFac::Furnace],
                requirements: vec![ItemAmount {
                    amount: 0.25,
                    item_type: ItemType::CopperOre,
                }],
            },
        );
        correct_result.insert(
            "Copper Ore".to_string(),
            ItemResult {
                describer: "Copper Ore".to_string(),
                item_type: ItemType::CopperOre,
                target_rate: 0.25,
                num_station: 0.0,
                station: vec![ManFac::Origin],
                requirements: vec![],
            },
        );
        correct_result
    }

    #[test]
    fn test_electric_motor() {
        let items = get_items();
        let (strings, items_per_sec, settings, mut result, mut result_order, bools) =
            crate_test_variables(0.5, ItemType::ElectricMotor, false);
        let mut ctx = CraftingContext::new(&items, &settings, &mut result_order, &mut result);
        items.get(&ItemType::ElectricMotor).unwrap().crafting_chain(
            &mut ctx,
            strings,
            items_per_sec,
            bools,
        );
        let correct_result = results_electric_motor();
        for result_value in &result {
            eprintln!(
                "result:         {:?}\ncorrect result: {:?}\n",
                result_value.1,
                correct_result.get(result_value.0).unwrap()
            );
            assert_eq!(result_value.1, correct_result.get(result_value.0).unwrap());
        }
        assert_eq!(result, correct_result);
    }

    #[test]
    fn test_electric_motor_double_rate() {
        let items = get_items();
        let (strings, items_per_sec, settings, mut result, mut result_order, bools) =
            crate_test_variables(1.0, ItemType::ElectricMotor, false);
        let mut ctx = CraftingContext::new(&items, &settings, &mut result_order, &mut result);
        items.get(&ItemType::ElectricMotor).unwrap().crafting_chain(
            &mut ctx,
            strings,
            items_per_sec,
            bools,
        );
        let correct_result = results_electric_motor_double_rate();
        for result_value in &result {
            eprintln!(
                "result:         {:?}\ncorrect result: {:?}\n",
                result_value.1,
                correct_result.get(result_value.0).unwrap()
            );
            assert_eq!(result_value.1, correct_result.get(result_value.0).unwrap());
        }
        assert_eq!(result, correct_result);
    }

    #[test]
    fn test_electric_motor_merge_output() {
        let items = get_items();
        let (strings, items_per_sec, mut settings, mut result, mut result_order, bools) =
            crate_test_variables(0.5, ItemType::ElectricMotor, false);
        settings.merge = true;
        let mut ctx = CraftingContext::new(&items, &settings, &mut result_order, &mut result);
        items.get(&ItemType::ElectricMotor).unwrap().crafting_chain(
            &mut ctx,
            strings,
            items_per_sec,
            bools,
        );
        let correct_result = results_electric_motor_merged();
        for result_value in &result {
            eprintln!(
                "result:         {:?}\ncorrect result: {:?}\n",
                result_value.1,
                correct_result.get(result_value.0).unwrap()
            );
            assert_eq!(result_value.1, correct_result.get(result_value.0).unwrap());
        }
        assert_eq!(result, correct_result);
    }

    #[test]
    fn test_electric_motor_assume_basics() {
        let items = get_items();
        let (strings, items_per_sec, mut settings, mut result, mut result_order, bools) =
            crate_test_variables(0.5, ItemType::ElectricMotor, false);
        settings.assume_basics = true;
        let mut ctx = CraftingContext::new(&items, &settings, &mut result_order, &mut result);
        items.get(&ItemType::ElectricMotor).unwrap().crafting_chain(
            &mut ctx,
            strings,
            items_per_sec,
            bools,
        );
        let mut correct_result = results_electric_motor();
        correct_result
            .remove("Iron Ore -> Iron Ingot -> Electric Motor")
            .unwrap();
        correct_result
            .remove("Iron Ore -> Iron Ingot -> Gear -> Electric Motor")
            .unwrap();
        correct_result
            .remove("Iron Ore -> Magnet -> Magnetic Coil -> Electric Motor")
            .unwrap();
        correct_result
            .remove("Copper Ore -> Copper Ingot -> Magnetic Coil -> Electric Motor")
            .unwrap();
        for result_value in &result {
            eprintln!(
                "result:         {:?}\ncorrect result: {:?}\n",
                result_value.1,
                correct_result.get(result_value.0).unwrap()
            );
            assert_eq!(result_value.1, correct_result.get(result_value.0).unwrap());
        }
        assert_eq!(result, correct_result);
    }

    #[test]
    fn test_electric_motor_merge_output_assume_basics() {
        let items = get_items();
        let (strings, items_per_sec, mut settings, mut result, mut result_order, bools) =
            crate_test_variables(0.5, ItemType::ElectricMotor, false);
        settings.merge = true;
        settings.assume_basics = true;
        let mut ctx = CraftingContext::new(&items, &settings, &mut result_order, &mut result);
        items.get(&ItemType::ElectricMotor).unwrap().crafting_chain(
            &mut ctx,
            strings,
            items_per_sec,
            bools,
        );
        let mut correct_result = results_electric_motor_merged();
        correct_result.remove("Iron Ore").unwrap();
        correct_result.remove("Copper Ore").unwrap();
        for result_value in &result {
            eprintln!(
                "result:         {:?}\ncorrect result: {:?}\n",
                result_value.1,
                correct_result.get(result_value.0).unwrap()
            );
            assert_eq!(result_value.1, correct_result.get(result_value.0).unwrap());
        }
        assert_eq!(result, correct_result);
    }
}

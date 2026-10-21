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
            "-r" | "--recipe" | "-recipe" => status = ArgState::ItemRecipe,
            // display the documentation
            "-h" | "--help" => doc::print_help(),
            // only show the item requested and not the entire chain
            "-t" | "--this-item" => settings.this_item_only = true,
            // catch all
            _ => {
                // process the options demanding more elaborate processing
                match status {
                    // proliferation
                    ArgState::ProlifLevel=> {
                        match argument.as_str() {
                            "1" => settings.proliferators = Proliferator::MKone,
                            "2" => settings.proliferators = Proliferator::MKtwo,
                            "3" => settings.proliferators = Proliferator::MKthree,
                            _ => eprintln!(
                                "invalid proliferation level was supplied (\"1\", \"2\" and \"3\" are valid; \"{}\" was supplied)", argument),
                        }
                    }
                    // chemlab level
                    ArgState::ChemLabLevel=> {
                        match argument.as_str() {
                            "1" => settings.chemlab = ChemLabMK::Lab,
                            "2" => settings.chemlab = ChemLabMK::QuantumLab,
                            _ => eprintln!(
                                "invalid chemlab level was supplied (\"1\", \"2\" are valid; \"{}\" was supplied)", argument),
                        }
                    }
                    // smelter level
                    ArgState::FurnaceLevel =>{
                        match argument.as_str() {
                            "1" => settings.smelter = SmelterMK::Arc,
                            "2" => settings.smelter = SmelterMK::Plane,
                            "3" => settings.smelter = SmelterMK::Negentropy,
                            _ => eprintln!(
                                "invalid smelter level was supplied (\"1\", \"2\" and \"3\" are valid; \"{}\" was supplied)", argument),
                        }
                    }
                    // assembler level
                    ArgState::AssemblerLevel=> {
                        match argument.as_str() {
                            "1" => settings.assembler = AssemblerMK::One,
                            "2" => settings.assembler = AssemblerMK::Two,
                            "3" => settings.assembler = AssemblerMK::Three,
                            "4" => settings.assembler = AssemblerMK::Four,
                            _ => eprintln!(
                                "invalid assembler level was supplied (\"1\", \"2\", \"3\" and \"4\"are valid; \"{}\" was supplied)", argument),
                        }
                    }
                    // research lab level
                    ArgState::LabLevel=> {
                        match argument.as_str() {
                            "1" => settings.lab = LabMK::MatrixLab,
                            "2" => settings.lab = LabMK::SelfEvolutionLab,
                            _ => eprintln!(
                                "invalid lab level was supplied (\"1\", \"2\" are valid; \"{}\" was supplied)", argument),
                        }
                    }
                    // disable proliferation for specific items and downward
                    ArgState::NoProliferation =>{
                        if let Ok(item) = argument.parse::<ItemType>() {
                            if item_hashmap.contains_key(&item) {
                                settings.no_proliferation.push(item);
                            } else {
                                eprintln!("{} isn't a valid item", argument);
                            }
                        } else {
                            eprintln!("{} isn't a valid item", argument);
                        }
                    }
                    // add additional production for specific items in a crafting chain
                    // needs next element in for loop
                    ArgState::AdditionalItems=> {
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
                            if item_hashmap.contains_key(&item) {
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
                    }
                    // set the recipe for a given item
                    // needs next element in for loop
                    ArgState::ItemRecipe=> {
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
                                            settings.item_recipe.push(ItemAmount::new(
                                                recipe_num as f32,
                                                item,
                                            ));
                                        } else {
                                            eprintln!("recipe index {} out of range for {}", recipe_num, item_name);
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
                    }
                    ArgState::Default=> {
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
        .contains(&settings.produced_item.item)
    {
        is_proliferated = false;
    }
    match item_hashmap.get(&settings.produced_item.item) {
        // set off creation of the crafting chain
        Some(item) => {
            let twostrings = StringDuo {
                item_name: settings.produced_item.item,
                prev_path,
            };
            let twobools = BoolDuo {
                is_proliferated,
                is_first_item: true,
            };
            item.crafting_chain(
                twostrings,
                settings.produced_item.amount,
                &settings,
                &mut result_order,
                &mut result,
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
    for (index, result_string) in result_order.clone().iter().enumerate() {
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

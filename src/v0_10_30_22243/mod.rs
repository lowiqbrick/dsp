// file for defining all items and recipes
// import everything from essentials.rs
mod essentials;
mod items_get;

pub mod items {
    use crate::v0_10_30_22243::essentials::item_logic::*;
    use crate::v0_10_30_22243::items_get::itemsmod::get_items;
    use std::collections::HashMap;

    /// function for outputting all items
    // fn print_items() {
    //     let mut map: HashMap<String, Item> = HashMap::new();
    //     map = get_items(map);
    //     println!("{:?}", map);
    // }

    /// the main function of version v0_10_30_22243
    pub fn v0_10_30_22243(mut args: Vec<String>) {
        // declare the variables required for processing the inputs
        let args_len = args.len();
        // println!("args len: {}", args_len);
        let mut item_hashmap: HashMap<String, Item> = HashMap::new();
        item_hashmap = get_items(item_hashmap);
        let mut status: ArgState = ArgState::Default;
        // Defualt settings of the program
        let mut settings: ProgamInfo = ProgamInfo::new(
            Proliferator::None,
            ChemLabMK::Lab,
            SmelterMK::ArcSmelter,
            AssemblerMK::MKone,
            LabMK::MatrixLab,
            vec![],
            vec![],
            vec![],
            false,
            false,
            ItemAmount::new(0.0, String::from("Default")),
        );
        // sanity check
        if args_len < 2 {
            panic!("no arguments were given to the function");
        }
        // loop for processing the arguments
        // get the item to be produced
        if item_hashmap.contains_key(&args[0]) {
            // println!("produced item set at index {}", index);
            settings.produced_item = ItemAmount::new(
                args[1].parse::<f32>().expect("provided invalid number"),
                String::from(args[0].clone()),
            );
        } else {
            eprintln!("given item name {} isn't valid", args[0]);
        }
        // remove the first two arguments
        args.remove(0);
        args.remove(0);
        // process the arguments
        let mut iterator = args.iter();
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
                "smelter=1" => settings.smelter = SmelterMK::ArcSmelter,
                "smelter=2" => settings.smelter = SmelterMK::PlaneSmelter,
                "smelter=3" => settings.smelter = SmelterMK::NegentropySmelter,
                // assembler
                "-a" | "--assembler" => status = ArgState::AssemblerLevel,
                "assembler=1" => settings.assembler = AssemblerMK::MKone,
                "assembler=2" => settings.assembler = AssemblerMK::MKtwo,
                "assembler=3" => settings.assembler = AssemblerMK::MKthree,
                "assembler=4" => settings.assembler = AssemblerMK::MKfour,
                // research lab
                "-l" | "--lab" => status = ArgState::LabLevel,
                "lab=1" => settings.lab = LabMK::MatrixLab,
                "lab=2" => settings.lab = LabMK::SelfEvolutionLab,
                // no proliferation
                "-n" | "--nopriliferation" => status = ArgState::NoProliferation,
                // additional items
                "-i" | "--items" => status = ArgState::AdditionalItems,
                // set the recipe
                "-r" | "-recipe" => status = ArgState::ItemRecipe,
                // process the options nemanding more elaborate processing
                // catch all
                _ => {
                    // proliferation
                    if status == ArgState::ProlifLevel {
                        match argument.as_str() {
                            "1" => settings.proliferators = Proliferator::MKone,
                            "2" => settings.proliferators = Proliferator::MKtwo,
                            "3" => settings.proliferators = Proliferator::MKthree,
                            _ => eprintln!(
                                "invalid proliferation level was supplied (\"1\", \"2\" and \"3\" are valid; \"{}\" was supplied)", argument),
                        }
                    }
                    // chemlab level
                    if status == ArgState::ChemLabLevel {
                        match argument.as_str() {
                            "1" => settings.chemlab = ChemLabMK::Lab,
                            "2" => settings.chemlab = ChemLabMK::QuantumLab,
                            _ => eprintln!(
                                "invalid chemlab level was supplied (\"1\", \"2\" are valid; \"{}\" was supplied)", argument),
                        }
                    }
                    // smelter level
                    if status == ArgState::FurnaceLevel {
                        match argument.as_str() {
                            "1" => settings.smelter = SmelterMK::ArcSmelter,
                            "2" => settings.smelter = SmelterMK::PlaneSmelter,
                            "3" => settings.smelter = SmelterMK::NegentropySmelter,
                            _ => eprintln!(
                                "invalid smelter level was supplied (\"1\", \"2\" and \"3\" are valid; \"{}\" was supplied)", argument),
                        }
                    }
                    // assembler level
                    if status == ArgState::AssemblerLevel {
                        match argument.as_str() {
                            "1" => settings.assembler = AssemblerMK::MKone,
                            "2" => settings.assembler = AssemblerMK::MKtwo,
                            "3" => settings.assembler = AssemblerMK::MKthree,
                            "4" => settings.assembler = AssemblerMK::MKfour,
                            _ => eprintln!(
                                "invalid assembler level was supplied (\"1\", \"2\", \"3\" and \"4\"are valid; \"{}\" was supplied)", argument),
                        }
                    }
                    // reserchlab level
                    if status == ArgState::LabLevel {
                        match argument.as_str() {
                            "1" => settings.lab = LabMK::MatrixLab,
                            "2" => settings.lab = LabMK::SelfEvolutionLab,
                            _ => eprintln!(
                                "invalid lab level was supplied (\"1\", \"2\" are valid; \"{}\" was supplied)", argument),
                        }
                    }
                    // disable proliferation for specific items and downward
                    if status == ArgState::NoProliferation {
                        if item_hashmap.contains_key(argument) {
                            settings.no_proliferation.push(argument.clone());
                        } else {
                            eprintln!("{} isn't a valid item", argument);
                        }
                    }
                    // add additional production for specific items in a crafting chain
                    // needs next element in for loop
                    if status == ArgState::AdditionalItems {
                        // save current and next arguments for processing later
                        let item_name = argument.clone();
                        let quantity = match iterator.next() {
                            Some(success) => success,
                            None => {
                                eprint!("no number specified after {}", item_name);
                                break;
                            }
                        };
                        if item_hashmap.contains_key(&item_name) {
                            settings.additional_items.push(ItemAmount::new(
                                quantity.parse::<f32>().expect("provided invalid number"),
                                argument.clone(),
                            ));
                        } else {
                            eprintln!("{} isn't a valid item", item_name);
                        }
                    }
                    // set the recepie for a given item
                    // needs next element in for loop
                    if status == ArgState::ItemRecipe {
                        // save current and next arguments for processing later
                        let item_name = argument.clone();
                        let quantity = match iterator.next() {
                            Some(success) => success,
                            None => {
                                eprint!("no number specified after {}", item_name);
                                break;
                            }
                        };
                        if item_hashmap.contains_key(&item_name) {
                            // check if recipe number given makes sense
                            let recipe_num: u8 =
                                quantity.parse::<u8>().expect("provided invalid number");
                            let item_rec_num_opt: Option<&Item> = item_hashmap.get(&item_name);
                            match item_rec_num_opt {
                                Some(t) => {
                                    // get item
                                    let item_clone: Item = t.clone();
                                    let recipe_tot_num = item_clone.recipes.len();
                                    // add item to vector if recipe number is a valid index for the vector
                                    if recipe_tot_num - 1 >= recipe_num as usize {
                                        settings.item_recipe.push(ItemAmount::new(
                                            recipe_num as f32,
                                            item_name.clone(),
                                        ));
                                    }
                                }
                                _ => eprintln!("item not in hashmap"),
                            }
                        } else {
                            eprintln!("{} isn't a valid item", item_name);
                        }
                    }
                }
            }
        }
        // Debugging only
        println!("{:?}", settings);
        // call method on item
        let mut result_order: Vec<String> = vec![];
        let mut result: HashMap<String, ItemResult> = HashMap::new();
        let prev_path = String::from("");
        let mut is_proliferated: bool = true;
        if settings
            .no_proliferation
            .contains(&settings.produced_item.item)
        {
            is_proliferated = false;
        }
        match item_hashmap.get(&settings.produced_item.item) {
            // set off creation of the crafting chain
            Some(item) => item.crafting_chain(
                settings.produced_item.item.clone(),
                settings.produced_item.amount as f32,
                &settings,
                &mut result_order,
                &mut result,
                prev_path,
                is_proliferated,
                true,
            ),
            _ => {
                panic!("requested item not in crafting recipes");
            }
        }
        // Debugging only
        // println!("\nresult order vector: {:?}\n", result_order);
        // println!("result vector: {:?}\n", result);
        let vector_len: usize = result_order.len();
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
}

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
    pub fn v0_10_30_22243(args: Vec<String>) {
        // declare the variables required for processing the inputs
        let args_len = args.len();
        // println!("args len: {}", args_len);
        let mut item_hashmap: HashMap<String, Item> = HashMap::new();
        item_hashmap = get_items(item_hashmap);
        let mut status = ArgState::Default;
        // Defualt settings of the program
        let mut settings = ProgamInfo::new(
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
            ItemAmount::new(0, String::from("Default")),
        );
        // sanity check
        if args_len < 2 {
            panic!("no arguments were given to the function");
        }
        // loop for processing the arguments
        let mut keep_looping = true;
        let mut index = 0;
        while keep_looping {
            // get the item to be produced
            if index == 0 {
                if item_hashmap.contains_key(&args[index]) {
                    // println!("produced item set at index {}", index);
                    settings.produced_item = ItemAmount::new(
                        args[index + 1]
                            .parse::<u8>()
                            .expect("provided invalid number"),
                        String::from(args[index].clone()),
                    );
                } else {
                    eprintln!("given item name {} isn't valid", args[index]);
                }
            }
            // arguments that are boolean
            // squash the output
            if args[index] == "-m" || args[index] == "--merge" {
                settings.merge = true;
            }
            // assume basics
            if args[index] == "-b" || args[index] == "--basics" {
                settings.assume_basics = true;
            }
            // arguments that change enums
            if args[index].contains("proliferator") {
                match args[index].as_str() {
                    "proliferator=1" => settings.proliferators = Proliferator::MKone,
                    "proliferator=2" => settings.proliferators = Proliferator::MKtwo,
                    "proliferator=3" => settings.proliferators = Proliferator::MKthree,
                    _ => eprintln!(
                        "the argument {} isn't valid and won't be incorporated",
                        args[index]
                    ),
                }
            }
            if status == ArgState::ProlifLevel {
                match args[index].as_str() {
                    "1" => settings.proliferators = Proliferator::MKone,
                    "2" => settings.proliferators = Proliferator::MKtwo,
                    "3" => settings.proliferators = Proliferator::MKthree,
                    _ => eprintln!(
                        "invalid proliferation level was supplied (\"1\", \"2\" and \"3\" are valid; \"{}\" was supplied)", args[index]),
                }
            }
            if args[index] == "-p" || args[index] == "--proliferation" {
                status = ArgState::ProlifLevel;
            }
            // chemlab level
            if args[index].contains("chemlab") {
                match args[index].as_str() {
                    "chemlab=1" => settings.chemlab = ChemLabMK::Lab,
                    "chemlab=2" => settings.chemlab = ChemLabMK::QuantumLab,
                    _ => eprintln!(
                        "the argument {} isn't valid and won't be incorporated",
                        args[index]
                    ),
                }
            }
            if status == ArgState::ChemLabLevel {
                match args[index].as_str() {
                    "1" => settings.chemlab = ChemLabMK::Lab,
                    "2" => settings.chemlab = ChemLabMK::QuantumLab,
                    _ => eprintln!(
                        "invalid chemlab level was supplied (\"1\", \"2\" are valid; \"{}\" was supplied)", args[index]),
                }
            }
            if args[index] == "-c" || args[index] == "--chemlab" {
                status = ArgState::ChemLabLevel;
            }
            // smelter leven; s take by squas so it is -f (furnace)
            if args[index].contains("smelter") {
                match args[index].as_str() {
                    "smelter=1" => settings.smelter = SmelterMK::ArcSmelter,
                    "smelter=2" => settings.smelter = SmelterMK::PlaneSmelter,
                    "smelter=3" => settings.smelter = SmelterMK::NegentropySmelter,
                    _ => eprintln!(
                        "the argument {} isn't valid and won't be incorporated",
                        args[index]
                    ),
                }
            }
            if status == ArgState::FurnaceLevel {
                match args[index].as_str() {
                    "1" => settings.smelter = SmelterMK::ArcSmelter,
                    "2" => settings.smelter = SmelterMK::PlaneSmelter,
                    "3" => settings.smelter = SmelterMK::NegentropySmelter,
                    _ => eprintln!(
                        "invalid smelter level was supplied (\"1\", \"2\" and \"3\" are valid; \"{}\" was supplied)", args[index]),
                }
            }
            if args[index] == "-s" || args[index] == "--smelter" {
                status = ArgState::FurnaceLevel;
            }
            // assembler level
            if args[index].contains("assembler") {
                match args[index].as_str() {
                    "assembler=1" => settings.assembler = AssemblerMK::MKone,
                    "assembler=2" => settings.assembler = AssemblerMK::MKtwo,
                    "assembler=3" => settings.assembler = AssemblerMK::MKthree,
                    "assembler=4" => settings.assembler = AssemblerMK::MKfour,
                    _ => eprintln!(
                        "the argument {} isn't valid and won't be incorporated",
                        args[index]
                    ),
                }
            }
            if status == ArgState::AssemblerLevel {
                match args[index].as_str() {
                    "1" => settings.assembler = AssemblerMK::MKone,
                    "2" => settings.assembler = AssemblerMK::MKtwo,
                    "3" => settings.assembler = AssemblerMK::MKthree,
                    "4" => settings.assembler = AssemblerMK::MKfour,
                    _ => eprintln!(
                        "invalid assembler level was supplied (\"1\", \"2\", \"3\" and \"4\"are valid; \"{}\" was supplied)", args[index]),
                }
            }
            if args[index] == "-a" || args[index] == "--assembler" {
                status = ArgState::AssemblerLevel;
            }
            // reserchlab level
            if args[index].contains("lab") {
                match args[index].as_str() {
                    "lab=1" => settings.lab = LabMK::MatrixLab,
                    "lab=2" => settings.lab = LabMK::SelfEvolutionLab,
                    _ => eprintln!(
                        "the argument {} isn't valid and won't be incorporated",
                        args[index]
                    ),
                }
            }
            if status == ArgState::LabLevel {
                match args[index].as_str() {
                    "1" => settings.lab = LabMK::MatrixLab,
                    "2" => settings.lab = LabMK::SelfEvolutionLab,
                    _ => eprintln!(
                        "invalid lab level was supplied (\"1\", \"2\" are valid; \"{}\" was supplied)", args[index]),
                }
            }
            if args[index] == "-l" || args[index] == "--lab" {
                status = ArgState::LabLevel;
            }
            // disable proliferation for specific items and downward
            if status == ArgState::NoProliferation {
                if item_hashmap.contains_key(&args[index]) {
                    settings.no_proliferation.push(args[index].clone());
                } else {
                    eprintln!("{} isn't a valid item", args[index]);
                }
            }
            if args[index] == "-n" || args[index] == "--noproliferation" {
                status = ArgState::NoProliferation;
            }
            // add additional production for specific items in a crafting chain
            if status == ArgState::AdditionalItems {
                if index >= args_len + 2 {
                    if item_hashmap.contains_key(&args[index]) {
                        settings.additional_items.push(ItemAmount::new(
                            args[index + 1]
                                .parse::<u8>()
                                .expect("provided invalid number"),
                            args[index].clone(),
                        ));
                    } else {
                        eprintln!("{} isn't a valid item", args[index]);
                    }
                }
            }
            if args[index] == "-i" || args[index] == "--items" {
                status = ArgState::AdditionalItems;
            }
            // set the recepie for a given item
            if status == ArgState::ItemRecipe {
                if index + 1 <= args_len - 1 {
                    if item_hashmap.contains_key(&args[index]) {
                        // check if recipe number given makes sense
                        let recipe_num: u8 = args[index + 1]
                            .parse::<u8>()
                            .expect("provided invalid number");
                        let item_rec_num_opt: Option<&Item> = item_hashmap.get(&args[index]);
                        match item_rec_num_opt {
                            Some(t) => {
                                // get item
                                let item_clone: Item = t.clone();
                                let recipe_tot_num = item_clone.recipes.len();
                                // add item to vector if recipe number is a valid index for the vector
                                if recipe_tot_num - 1 >= recipe_num as usize {
                                    settings
                                        .item_recipe
                                        .push(ItemAmount::new(recipe_num, args[index].clone()));
                                }
                            }
                            None => eprintln!("item not in hashmap"),
                        }
                    } else {
                        eprintln!("{} isn't a valid item", args[index]);
                    }
                }
            }
            if args[index] == "-r" || args[index] == "--recipe" {
                status = ArgState::ItemRecipe;
            }
            // prepare for next loop
            index += 1;
            // end loop
            if args_len - 1 < index {
                keep_looping = false;
            }
        }
        println!("{:?}", settings);
        // call method on item
        let mut result_order: Vec<String> = vec![];
        let mut result: HashMap<String, ItemResult> = HashMap::new();
        let prev_path = String::from("");
        let mut is_proliferated: bool = true;
        if settings.no_proliferation.contains(&settings.produced_item.item) {
            is_proliferated = false;
        }
        match item_hashmap.get(&settings.produced_item.item) {
            // set off creation of the crafting chain
            Some(item) => {
                item.crafting_chain(
                    settings.produced_item.item.clone(), 
                    settings.produced_item.amount as f32,
                    &settings,
                    &mut result_order, 
                    &mut result,
                    prev_path,
                    is_proliferated,
                true)
            }
            None => {
                panic!("requested item not in crafting recipes");
            }
        }
        for result_string in result_order.clone().iter() {
            match result.get(result_string) {
                Some(result_match) => {
                    println!("{}", result_match);
                }
                None => {
                    panic!("{} wasn't found in the item list", result_string);
                }
            }
        }
    }
}

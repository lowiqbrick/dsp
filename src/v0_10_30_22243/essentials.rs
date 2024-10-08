// file for all structs functions etc.

pub mod item_logic {
    use std::collections::HashMap;
    use std::fmt::{self, Display};

    use crate::v0_10_30_22243::items_get::itemsmod::get_items;

    /// enum for manufacturing facilities
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub enum ManFac {
        Origin,
        Assembler,
        Furnace,
        Lab,
        OilRefinery,
        ChemicalPlant,
        MiniatureParticleCollider,
    }

    /// struct for combining an item and an amount of said item
    #[derive(Debug, Clone, PartialEq)]
    pub struct ItemAmount {
        pub amount: f32,
        pub item: String,
    }

    impl ItemAmount {
        pub fn new(amount: f32, item: String) -> ItemAmount {
            ItemAmount { amount, item }
        }
    }

    /// enum for differentiation between items and not an item (nai)
    /// which is relevant for crafting recipes for items which cannot be
    /// crafted and only be obtained/mined (ores, critical photons, etc.)
    #[derive(Debug, Clone, PartialEq)]
    pub enum IsItem {
        Item(ItemAmount),
        // NotAnItem
        NAI,
    }

    impl IsItem {
        pub fn new(item_amount: ItemAmount) -> IsItem {
            IsItem::Item(item_amount)
        }

        pub fn new_nai() -> IsItem {
            IsItem::NAI
        }
    }

    /// struct for recipes
    #[derive(Debug, Clone)]
    pub struct Recipe {
        // crafting time (in seconds)
        crafting_time: f32,
        pub ingredients: Vec<IsItem>,
        pub products: Vec<IsItem>,
    }

    impl Recipe {
        pub fn new(crafting_time: f32, ingredients: Vec<IsItem>, products: Vec<IsItem>) -> Recipe {
            Recipe {
                crafting_time,
                ingredients,
                products,
            }
        }
    }
    /// Struct for storing the result of a part of the crafting chain
    #[derive(Debug, Clone)]
    pub struct ItemResult {
        pub describer: String,
        pub name: String,
        pub target_rate: f32,
        pub num_station: f32,
        pub station: Vec<ManFac>,
        pub requirements: Vec<ItemAmount>,
    }

    impl ItemResult {
        fn new(
            describer: String,
            name: String,
            target_rate: f32,
            num_station: f32,
            station: Vec<ManFac>,
            requirements: Vec<ItemAmount>,
        ) -> ItemResult {
            ItemResult {
                describer,
                name,
                target_rate,
                num_station,
                station,
                requirements,
            }
        }
    }

    impl Display for ItemResult {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // write all things with static values
            write!(f, "{}\n", self.describer)?;
            write!(f, "item target rate: {:.1}\n", self.target_rate)?;
            if self.station != vec![ManFac::Origin] {
                write!(
                    f,
                    "buildings to build: {} {:?} ({:.1})",
                    self.num_station.ceil(),
                    self.station,
                    self.num_station
                )?;
                write!(f, "\n")?;
                write!(f, "this requires the following (per second):\n")?;
                // write vector
                for (index, amount) in self.requirements.clone().iter().enumerate() {
                    // tab
                    if index == 0 {
                        print!("   ");
                    }
                    write!(f, "    {:.1} {}", amount.amount, amount.item)?;
                }
            }
            // hand over the results
            write!(f, "")
        }
    }
    /// struct for representing an item
    #[derive(Debug, Clone)]
    pub struct Item<'a> {
        pub name: &'a str,
        pub creation_facility: Vec<ManFac>,
        pub recipes: Vec<Recipe>,
    }

    macro_rules! modrate {
        ($item_rate: ident, $multiplier: literal, $manfac_counter: ident) => {
            $item_rate *= $multiplier;
            $manfac_counter += 1;
        };
    }

    /// an function to return the current proliferation factor based on curent settings
    fn prolif_factor(settings: &ProgamInfo) -> f32 {
        let factor: f32 = match settings.proliferators {
            Proliferator::MKone => 1.125,
            Proliferator::MKtwo => 1.2,
            Proliferator::MKthree => 1.25,
            Proliferator::None => 1.0,
        };
        factor
    }

    impl<'a> Item<'a> {
        pub fn new(name: &'a str, creation_facility: Vec<ManFac>, recipes: Vec<Recipe>) -> Item {
            Item {
                name,
                creation_facility,
                recipes,
            }
        }

        /// the central method on which everything in the program is build
        pub fn crafting_chain(
            &self,
            item_name: String,
            item_per_sec: f32,
            settings: &ProgamInfo,
            result_order: &mut Vec<String>,
            result: &mut HashMap<String, ItemResult>,
            mut prev_path: String,
            mut is_proliferated: bool,
            is_first_item: bool,
        ) {
            // get rid of warning
            if !is_first_item {
                let _prev_path_warn: &str = &prev_path[0..1];
            }
            // get all items
            let mut items_map: HashMap<String, Item> = HashMap::new();
            items_map = get_items(items_map);
            let mut new_path: String = String::from(&item_name);
            if !is_first_item {
                new_path.extend([" -> "]);
            }
            new_path.extend([prev_path.clone()]);
            prev_path = new_path.clone();
            // create result variable
            let mut result_var: ItemResult;
            // initialise result with default values
            if settings.merge {
                result_var = ItemResult::new(
                    // if results ar to be merged no ellaborate paths
                    // just the items name
                    item_name.clone(),
                    item_name.clone(),
                    -1.0,
                    -1.0,
                    vec![ManFac::Origin],
                    vec![],
                );
            } else {
                result_var = ItemResult::new(
                    new_path.clone(),
                    item_name.clone(),
                    -1.0,
                    -1.0,
                    vec![ManFac::Origin],
                    vec![],
                );
            }
            // save desired target rate for item
            result_var.target_rate = item_per_sec;
            // write item information in result if the output
            // is supposed to be merged
            let mut is_adding_new_item: bool = false;
            if settings.merge {
                // if result item already exists copy all values
                // if not remember that a new one needs to be created
                match result.get(&item_name) {
                    Some(existing_result) => {
                        result_var.target_rate =
                            existing_result.target_rate + result_var.target_rate;
                        result_var.num_station = existing_result.num_station;
                        result_var.station = existing_result.station.clone();
                        result_var.requirements = existing_result.requirements.clone();
                    }
                    _ => {
                        is_adding_new_item = true;
                    }
                }
                // only add String if not yet present
                if !result_order.contains(&item_name) {
                    result_order.push(item_name.clone());
                }
            } else {
                result_order.push(new_path.clone());
                is_adding_new_item = true;
            }
            // get a copy of the requested item from the item hashmap
            // after this match item_name can be assumed to be a valid key
            let current_item: &Item = match items_map.get(&item_name) {
                Some(item) => item,
                _ => {
                    panic!("requested item '{}' doesn't exist in hashmap", item_name)
                }
            };
            // is the recipe not the recipe at index 0?
            let mut current_recipe_index: usize = 0;
            // find out if current item is in the list to have a different recipe
            for vec_element in settings.item_recipe.iter() {
                // if item is in vector, the recipe index needs to be changed
                if item_name == vec_element.item {
                    current_recipe_index = vec_element.amount as usize;
                    // sanity check; is the given recipe index valid?
                    if current_recipe_index + 1 > current_item.recipes.len() {
                        panic!(
                            "recipe index given for item {} was {} but item only has {} recipes\nmaybe you forgot to subtract 1?",
                            item_name,
                            current_recipe_index,
                            current_item.recipes.len()
                        );
                    }
                }
            }
            // get amount of the item as output (and as an ingredient, if thats the case)
            // and calculate the production rate, with crafting speed, proliferation, etc.
            let mut output_amount: f32 = 0.0;
            let mut input_amount: f32 = 0.0;
            let current_recipe: Recipe = current_item.recipes[current_recipe_index].clone();
            for item_amount in current_recipe.products.clone().iter() {
                match item_amount {
                    IsItem::Item(item_match) => {
                        if item_match.item == result_var.name {
                            output_amount = item_match.amount;
                        }
                    }
                    IsItem::NAI => {
                        // do nothing
                        print!("");
                    }
                }
            }
            if output_amount == 0.0 {
                panic!(
                    "the output of recipe (index {}) from item '{}' doesn't contain the item",
                    current_recipe_index, result_var.name
                );
            }
            for item_amount in current_recipe.ingredients.clone().iter() {
                match item_amount {
                    IsItem::Item(item_match) => {
                        if item_match.item == result_var.name {
                            input_amount = item_match.amount;
                        }
                    }
                    IsItem::NAI => {
                        // do nothing
                        print!("");
                    }
                }
            }
            // calculate the net output of the recipe
            let net_output: f32 = output_amount - input_amount;
            if net_output <= 0.0 {
                panic!(
                    "recipe has net output of {} which doesn't make sense",
                    net_output
                );
            }
            let net_output_per_second: f32 = net_output as f32 / current_recipe.crafting_time;
            let mut net_output_proliferated: f32 = net_output_per_second;
            // factor in proliferation
            // proliferation deactivated by function?
            if is_proliferated {
                let mut no_proliferation_vector: bool = false;
                for item_prolif in settings.no_proliferation.clone().iter() {
                    if item_prolif == &result_var.name {
                        no_proliferation_vector = true;
                        is_proliferated = false;
                    }
                }
                if !no_proliferation_vector {
                    let prolif_factor: f32 = prolif_factor(settings);
                    net_output_proliferated = net_output_proliferated * prolif_factor;
                }
            }
            // handle the different crafting stations
            let mut net_output_machine: f32 = net_output_proliferated;
            let mut manfac_counter: i32 = 0;
            for manfac in current_item.creation_facility.clone() {
                match manfac {
                    ManFac::Assembler => match settings.assembler {
                        AssemblerMK::MKone => {
                            modrate!(net_output_machine, 0.75, manfac_counter);
                        }
                        AssemblerMK::MKtwo => {
                            modrate!(net_output_machine, 1.0, manfac_counter);
                        }
                        AssemblerMK::MKthree => {
                            modrate!(net_output_machine, 1.5, manfac_counter);
                        }
                        AssemblerMK::MKfour => {
                            modrate!(net_output_machine, 3.0, manfac_counter);
                        }
                    },
                    ManFac::Furnace => match settings.smelter {
                        SmelterMK::ArcSmelter => {
                            modrate!(net_output_machine, 1.0, manfac_counter);
                        }
                        SmelterMK::PlaneSmelter => {
                            modrate!(net_output_machine, 2.0, manfac_counter);
                        }
                        SmelterMK::NegentropySmelter => {
                            modrate!(net_output_machine, 3.0, manfac_counter);
                        }
                    },
                    ManFac::Lab => match settings.lab {
                        LabMK::MatrixLab => {
                            modrate!(net_output_machine, 1.0, manfac_counter);
                        }
                        LabMK::SelfEvolutionLab => {
                            modrate!(net_output_machine, 3.0, manfac_counter);
                        }
                    },
                    ManFac::OilRefinery => {
                        modrate!(net_output_machine, 1.0, manfac_counter);
                    }
                    ManFac::ChemicalPlant => match settings.chemlab {
                        ChemLabMK::Lab => {
                            modrate!(net_output_machine, 1.0, manfac_counter);
                        }
                        ChemLabMK::QuantumLab => {
                            modrate!(net_output_machine, 2.0, manfac_counter);
                        }
                    },
                    ManFac::MiniatureParticleCollider => {
                        modrate!(net_output_machine, 1.0, manfac_counter);
                    }
                    ManFac::Origin => {
                        net_output_machine *= 1.0;
                    }
                }
            }
            if manfac_counter > 1 {
                panic!("item rate was modified multiple times ({})", manfac_counter);
            }
            // calculate how many crafting machines are required for matching troughput
            let manvac_count: f32 = item_per_sec / net_output_machine;
            // calculate the multiplier for the amount of ingredients required
            let current_proliferator_factor: f32;
            if is_proliferated {
                current_proliferator_factor = prolif_factor(settings);
            } else {
                current_proliferator_factor = 1.0;
            }
            let ingredient_multiplicator: f32 = ((item_per_sec / current_recipe.crafting_time)
                / net_output_per_second)
                / current_proliferator_factor;
            // putting everything together
            let mut output_machine_ingredients: Vec<ItemAmount> = vec![];
            // apply modifier
            for isitem in current_recipe.ingredients.clone().iter() {
                match isitem {
                    IsItem::Item(ingredient) => {
                        output_machine_ingredients.push(ItemAmount::new(
                            ingredient.amount * ingredient_multiplicator,
                            ingredient.item.clone(),
                        ));
                    }
                    IsItem::NAI => {
                        // do nothing
                        print!("");
                    }
                }
            }
            if settings.merge {
                if is_adding_new_item {
                    result_var.num_station = manvac_count;
                    // Debugging
                    /*
                    println!(
                        "new item {} requires {} stations",
                        current_item.name, manvac_count
                    );*/
                } else {
                    // Debugging
                    /*
                    print!(
                        "item {} went with {} addiionsal stations from {} station to",
                        current_item.name, manvac_count, result_var.num_station
                    );*/
                    result_var.num_station += manvac_count;
                    // Debugging
                    /*
                    println!(" {} stations", result_var.num_station);*/
                }
            } else {
                result_var.num_station = manvac_count;
            }
            result_var.requirements = output_machine_ingredients;
            if result_var.station == vec![ManFac::Origin] {
                result_var.station = current_item.creation_facility.clone();
            }
            // sanity checks on result_var
            assert_ne!(result_var.num_station, -1.0);
            let help_assert: String = String::from(current_item.name);
            // only assert if the item doesn't have vec!(ManFac::Origin) naturally
            if !settings.basics.contains(&help_assert) {
                assert_ne!(
                    result_var.station,
                    vec!(ManFac::Origin),
                    "result doesn't have a crafting station"
                );
                assert_ne!(result_var.requirements, vec![]);
            }
            // write results into hashmap
            if is_adding_new_item {
                if settings.merge {
                    result.insert(result_var.name.clone(), result_var.clone());
                } else {
                    result.insert(result_var.describer.clone(), result_var.clone());
                }
            } else {
                if settings.merge {
                    // increase the values already present
                    match result.get_mut(&result_var.name) {
                        Some(found_item) => {
                            // increase amount of required items per minute
                            found_item.target_rate += item_per_sec;
                            // increase required prodiction buildings
                            found_item.num_station = result_var.num_station;
                            // increase the required ingrediences
                            for (index, _found_ingredient) in
                                found_item.requirements.clone().iter().enumerate()
                            {
                                found_item.requirements[index].amount +=
                                    result_var.requirements[index].amount;
                            }
                        }
                        _ => {
                            panic!(
                                "tried to write into the key '{}', which wasn't found",
                                result_var.name.clone()
                            );
                        }
                    }
                } else {
                    panic!(
                        "eventhough nothing should me merged the given path '{}' for the hashmap already exists",
                        result_var.describer);
                }
            }
            // finally make the function recursive by calling the function on the
            // ingredient items
            for ingredient in current_item.recipes[current_recipe_index]
                .ingredients
                .iter()
            {
                match ingredient {
                    IsItem::Item(real_ingredient) => match items_map.get(&real_ingredient.item) {
                        Some(call_item) => {
                            call_item.crafting_chain(
                                String::from(call_item.name),
                                real_ingredient.amount * ingredient_multiplicator,
                                &settings,
                                result_order,
                                result,
                                prev_path.clone(),
                                is_proliferated,
                                false,
                            );
                        }
                        None => {
                            panic!(
                                "failed to call 'fn crafting_chain' on key {}",
                                &real_ingredient.item
                            );
                        }
                    },
                    IsItem::NAI => {
                        // if item is origin do nothing
                        print!("");
                    }
                }
            }
        }
    }

    // enums and structs for the execution of the main function
    /// enum for saving the type of lab used
    #[derive(Debug)]
    pub enum ChemLabMK {
        // Chemical Plant
        Lab,
        // Quantum Chemical Plant
        QuantumLab,
    }

    /// enum for saving the type of assembler used
    #[derive(Debug)]
    pub enum SmelterMK {
        ArcSmelter,
        PlaneSmelter,
        NegentropySmelter,
    }

    /// enum for saving the type of assembler used
    #[derive(Debug)]
    pub enum AssemblerMK {
        MKone,
        MKtwo,
        MKthree,
        MKfour,
    }

    /// enum for saving the type of assembler used
    #[derive(Debug)]
    pub enum LabMK {
        MatrixLab,
        SelfEvolutionLab,
    }

    /// enum for saving the type of assembler used
    #[derive(Debug)]
    pub enum Proliferator {
        MKone,
        MKtwo,
        MKthree,
        None,
    }

    /// struct for containing all information for the main function
    #[derive(Debug)]
    pub struct ProgamInfo {
        // enums to save the facilities used
        pub proliferators: Proliferator,
        pub chemlab: ChemLabMK,
        pub smelter: SmelterMK,
        pub assembler: AssemblerMK,
        pub lab: LabMK,
        pub no_proliferation: Vec<String>,
        pub additional_items: Vec<ItemAmount>,
        // ItemAmount here is treated differently than in the item hashmap
        // ItemAmount.amount == Index of the to be used recipe
        // ItemAbount.item == Name of the Item which is supposed to have its recipe changed
        pub item_recipe: Vec<ItemAmount>,
        pub merge: bool,
        pub assume_basics: bool,
        pub basics: Vec<String>,
        pub produced_item: ItemAmount,
    }
    impl ProgamInfo {
        pub fn new(
            proliferators: Proliferator,
            chemlab: ChemLabMK,
            smelter: SmelterMK,
            assembler: AssemblerMK,
            lab: LabMK,
            no_proliferation: Vec<String>,
            additional_items: Vec<ItemAmount>,
            item_recipe: Vec<ItemAmount>,
            merge: bool,
            assume_basics: bool,
            produced_item: ItemAmount,
        ) -> ProgamInfo {
            let basics = vec![
                String::from("Iron Ore"),
                String::from("Copper Ore"),
                String::from("Stone"),
                String::from("Coal"),
                String::from("Silicon Ore"),
                String::from("Titanium Ore"),
                String::from("Water"),
                String::from("Crude Oil"),
                String::from("Core Element"),
                String::from("Critical Photon"),
                String::from("Kimberlite Ore"),
                String::from("Fractal Silicon"),
                String::from("Grating Crystal"),
                String::from("Stalagmite Crystal"),
                String::from("Unipolar Magnet"),
                String::from("Fire Ice"),
                String::from("Log"),
                String::from("Plant Fuel"),
                String::from("Dark Fog Matrix"),
                String::from("Energy Shard"),
                String::from("Silicon-based Neuron"),
                String::from("Negentropy Singularity"),
                String::from("Matter Recombinator"),
            ];
            ProgamInfo {
                proliferators,
                chemlab,
                smelter,
                assembler,
                lab,
                no_proliferation,
                additional_items,
                item_recipe,
                merge,
                assume_basics,
                basics: basics.clone(),
                produced_item,
            }
        }
    }

    /// enum for containing the current state of the processing
    /// of the arguments
    #[derive(PartialEq)]
    pub enum ArgState {
        // the default; only an item name and an amount
        Default,
        // items not to be proliferated
        NoProliferation,
        // additional items
        AdditionalItems,
        // specify proliferation
        ProlifLevel,
        // specify chemical labs
        ChemLabLevel,
        // specify furnace
        FurnaceLevel,
        // specify assembler
        AssemblerLevel,
        // specify laboratory
        LabLevel,
        // specify the recipe use for an item
        ItemRecipe,
    }
}

// macro for creating items in a more convienent manner
#[macro_export]
macro_rules! recipe {
    ($crafting_time: expr, ($($ingredients: tt)*), ($($products: tt)*)) => {
        Recipe::new($crafting_time, vec![$($ingredients)*], vec![$($products)*])
    };
}
#[macro_export]
macro_rules! recitem {
    ($amount: literal, $itemname: literal) => {
        IsItem::new(ItemAmount::new($amount, String::from($itemname)))
    };
}
#[macro_export]
macro_rules! item {
    ($name: literal, ($($item_vec: tt)*), ($($rec_vec: tt)*)) => {
        Item::new($name, vec![$($item_vec)*], vec![$($rec_vec)*])
    };
}
#[macro_export]
macro_rules! tohash {
    ($hashmap: ident, $item_name: ident, $item_str: literal, $item: expr) => {
        let $item_name: Item = $item;
        $hashmap.insert(String::from($item_str), $item_name);
    };
}

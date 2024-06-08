// adjustments
// imports
// read the terminal input
use std::env;

use v0_10_30_22243::items::{get_items, hello_combined, print_items};
use std::collections::HashMap;
mod v0_10_30_22243;

fn main() {
    //get the terminal arguments
    let args: Vec<String> = env::args().collect();
    // output the arguments for debugging purposes
    /*
    [src/main.rs:7:5] args = [
    "target/debug/dsp",
    ]
     */
    dbg!(args);
    hello_combined();
    print_items(); 
    let len_has = get_items(HashMap::new()).len();
    println!("the HashMap has a length of {}", len_has);
}

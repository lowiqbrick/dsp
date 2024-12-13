// adjustments
// imports
// read the terminal input
use std::env;

use beta::items::beta;
mod beta;

fn main() {
    //get the terminal arguments
    let mut args: Vec<String> = env::args().collect();
    // remove the path
    args.remove(0);
    // debugging only
    // println!("{:?}", args);
    beta(args);
}

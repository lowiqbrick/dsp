// adjustments
// imports
// read the terminal input
use std::env;

use v0_10_30_22243::items::v0_10_30_22243;
mod v0_10_30_22243;

fn main() {
    //get the terminal arguments
    let mut args: Vec<String> = env::args().collect();
    // remove the path
    args.remove(0);
    println!("{:?}", args);
    v0_10_30_22243(args);
}

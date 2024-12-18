/// display the documentation for the command
pub fn print_help() {
    println!(
        r#"
    Name
        dsp -- calculate crafting chains in Dyson Sphere Program
    
    Synopsis
        dsp item_name item_quantity_per_second [Options]
        
    Options
        -m --merge                  reduce the output so all instances on one items
                                    crafting chain get added together for better overview

        -b --basics                 don't display raw input things such as ores,crude oil, etc.

        -p --proliferation <NUMBER> specify the level of proliferation thats supposed to be used
                                    [default: 1; accepted: 1, 2, 3]
        or "proliferator=1" "proliferator=2" "proliferator=3"

        -c --chemlab <NUMBER>       specify the tier of chemical lab used
                                    [default: 1; accepted: 1, 2]
        or  "chemlab=1" "chemlab=2"

        -s --smelter <NUMBER>       specify the tier of smelter used
                                    [default: 1; accepted: 1, 2, 3]
        or "smelter=1" "smelter=2" "smelter=3"

        -a --assembler <NUMBER>     specify the tier of assembler used
                                    [default: 1; accepted: 1, 2, 3, 4]
        or "assembler=1" "assembler=2" "assembler=3"

        -l --lab <NUMBER>           specify the tier of research lab used
                                    [default: 1; accepted: 1, 2]
        or "lab=1" "lab=2"

        -n --noproliferation        specify items that aren't supposed to be proliferated
                                        -n "Copper Ingot"

        -i --items                  add additional quatities for items to the ones required for the chain
                                    for example: in case you want more gravity marices for warpers
                                        -i "Gravity Matrix" 1.0

        -r --recipe                 specify the recipe used, in case there is more than one way of producing something
                                        -r 2

        -t --this-item              print only the item that is at the top of the chain

        -h --help                   display instructions for this command"#
    );
}

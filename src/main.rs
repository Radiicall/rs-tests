use std::*;
pub mod shoppinglist;
pub mod rps;
pub mod calc;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please select a test to run");
        return;
    } else if args.iter().any(|i| &i == &"--help") || args.iter().any(|i| &i == &"-h") {
        println!("-------------\nAvailable tests: \n-s  --shoppinglist  Shopping List\n-r  --rps  Rock Paper Scissors\n-c  --calc Calculator\n-------------");
    }
    let arg = &args[1].to_lowercase();
    if arg == "-s" || arg == "--shoppinglist" {
        shoppinglist::run();
    } else if arg == "-r" || arg == "--rps" {
        rps::run();
    } else if arg == "-c" || arg == "--calc" {
        calc::run();
    } else {
        println!("Use --help to see available tests");
    }
}

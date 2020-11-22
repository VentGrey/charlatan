use std::env;
use std::process::exit;

// Mod usage
mod install;

// External crates
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        println!("{}", "Excess of arguments!".red());
        exit(1);
    } else if args.len() == 1 {
        println!("{}", "Empty arguments list!".red());
        exit(1);
    } else {
        let option = &args[1];
        match option.as_str() {
            "install" => install::download(),
            "update" => println!("Updating julia toolchain"),
            _ => {
                println!("Invalid argument...");
                exit(1);
            }
        }
    }
}

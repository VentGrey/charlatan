use std::env;
use std::process::exit;

// Mod usage
mod install;

// External crates
use colored::*;

/** Main crate function.
The following function can handle arguments in a small order */
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
            "install" => install::init(),
            "update" => println!("Updating julia toolchain"),
            "help" => help(),
            _ => {
                println!("Invalid argument...");
                exit(1);
            }
        }
    }
}

fn help() {
    const VER: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    const AUT: Option<&'static str> = option_env!("CARGO_PKG_AUTHORS");

    println!(
        "Charlatan ({})
A Rust program used to ironically install Julia

Authors: [{}]
License: [GPL-2.0]

USAGE:
    charlatan [ARGS]

ARGS:
    install    Installs a selected Julia Toolchain
    update     Updated the existing Julia Toolchain
    help       Displays this message

For more information about the development of this program please see
the README file included in:
https://github.com/VentGrey/charlatan/blob/master/README.md
",
        VER.unwrap_or("unknown"),
        AUT.unwrap_or("none"),
    );
}

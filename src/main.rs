use std::env;
use std::process::exit;

fn main() {
        let args: Vec<String> = env::args().collect();

        if args.len() > 2 {
                println!("Excess of arguments!");
                exit(1);
        } else if args.len() == 1 {
                println!("Empty arguments list!");
                exit(1);
        } else if args.is_empty() {
                println!("Empty argument list");
                exit(1);
        } else {
                let option = &args[1];
                match option.as_str() {
                        "install" => println!("Installing Julia Toolchain"),
                        "update"  => println!("Updating julia toolchain"),
                        _ => {
                                println!("Invalid argument...");
                                exit(1);
                        }
                }
        }
}

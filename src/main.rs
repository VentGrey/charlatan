use std::env;
use std::process::exit;

fn main() {
        let args: Vec<String> = env::args().collect();
        let option = &args[1];

        if args.len() > 2 {
                println!("Excess of arguments!");
                exit(1);
        }

        match option.as_str() {
                "install" => println!("Installing Julia Toolchain"),
                "update" => println!("Updating julia toolchain"),
                _ => {
                        println!("Invalid argument...");
                        exit(1);
                }
        }

        println!("First option: {:?}", option);
        println!("{:?}", args);
}

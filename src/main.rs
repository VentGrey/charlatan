use std::env;

fn main() {
        let args: Vec<String> = env::args().collect();
        let option = &args[1];

        if args.len() > 2 {
                panic!("excess arguments!");
        }

        match option.as_str() {
                "install" => println!("Installing Julia Toolchain"),
                "update" => println!("Updating julia toolchain"),
                _ => println!("wtf am I supposed to do?")
        }

        println!("First option: {:?}", option);
        println!("{:?}", args);
}

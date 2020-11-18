use std::process::Command;

// External Crates
use colored::*;

fn compatible() -> bool {
    let compat: bool = if cfg!(target_os = "windows") {
        eprintln!("{}", "Sorry Windows isn't compatible, it has an installer already");
        false
    } else if cfg!(target_os = "linux") {
        println!("You seem to be running {}...", "algo");
        true
    } else {
        eprintln!("Could not determine compatibility with your OS. Sorry");
        false
    };

    compat
}

pub fn download() {

}

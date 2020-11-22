use std::process::Command;
use std::process::exit;

// External Crates
use colored::*;

fn compatible() -> bool {
    let compat: bool = if cfg!(target_os = "windows") {
        eprintln!(
            "{}",
            "Sorry Windows isn't compatible, it has an installer already"
        );
        false
    } else if cfg!(target_os = "linux") {
        println!("{}", "Checking for wget...".blue());
        // Check if wget is installed on your system
        let is_wget = Command::new("which")
            .arg("wget")
            .status()
            .expect("Could not run which");

        if is_wget.code() != Some(0) {
            println!("Charlatan needs {} to be installed", "wget".yellow());
            exit(1);
        }

        true
    } else {
        eprintln!("Could not determine compatibility with your OS. Sorry");
        false
    };

    compat
}

pub fn download() {
    println!("Determining system compatibility...");

    let is_compat = compatible();

    if is_compat == false {
        println!("Your system is {} compatible with charlatan", "NOT".red());
        exit(1);
    }
}

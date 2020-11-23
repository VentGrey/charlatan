use std::process::exit;
use std::process::Command;

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
            return false;
        }

        true
    } else {
        eprintln!("Could not determine compatibility with your OS. Sorry");
        false
    };

    compat
}

pub fn install() {
    println!("Determining system compatibility...");

    if !compatible() {
        println!("Your system is {} compatible with charlatan", "NOT".red());
        exit(1);
    }




}

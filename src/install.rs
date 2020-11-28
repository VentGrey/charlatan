use std::process::exit;
use std::process::Command;

// External Crates
use colored::*;

// Own crates
use scanrs::*;

fn compatible() -> bool {
    let compat: bool = if cfg!(target_os = "windows") {
        eprintln!(
            "{}",
            "Sorry Windows isn't compatible, it has an installer already"
        );
        false
    } else if cfg!(target_os = "linux") {
        println!("{}", "Checking for wget...".blue());
        // Check if wget is installed on this system
        let is_wget = Command::new("which")
            .arg("wget")
            .status()
            .expect("Could not run which");

        println!("{}", "Checking for tar...".blue());
        // Check if tar is installed on this system
        let is_tar = Command::new("which")
            .arg("tar")
            .status()
            .expect("Could not run which");

        if (is_wget.code() != Some(0)) && (is_tar.code() != Some(0)) {
            println!(
                "Charlatan needs {} and {} to be installed",
                "wget".yellow(),
                "tar".yellow()
            );
            return false;
        }

        println!("[{}] Your system is compatible", "✔".green());
        true
    } else {
        eprintln!("Could not determine compatibility with your OS. Sorry");
        false
    };

    compat
}

fn download(ver: i32) {
    match ver {
        1 => {
            println!(
                "Downloading latest Julia Stable tarball to {}",
                "/tmp".blue()
            );

            Command::new("wget")
                .arg("https://julialang-s3.julialang.org/bin/linux/x64/1.5/julia-1.5.3-linux-x86_64.tar.gz")
                .arg("-P")
                .arg("/tmp")
                .status()
                .expect("could not run wget!");
        }
        2 => {
            println!(
                "Downloading latest Julia Nightly tarball to {}",
                "/tmp".blue()
            );

            Command::new("wget")
                .arg("https://julialangnightlies-s3.julialang.org/bin/linux/x64/julia-latest-linux64.tar.gz")
                .arg("-P")
                .arg("/tmp")
                .status()
                .expect("could not run wget!");
        }
        3 => {
            println!("Downloading latest Julia LTS tarball to {}", "/tmp".blue());

            Command::new("wget")
                .arg("https://julialang-s3.julialang.org/bin/linux/x64/1.0/julia-1.0.5-linux-x86_64.tar.gz")
                .arg("-P")
                .arg("/tmp")
                .status()
                .expect("could not run wget!");
        }
        _ => {
            eprintln!("Unknown option {}", ver);
            exit(100);
        }
    }
}

pub fn init() {
    println!("Determining system compatibility...");

    if !compatible() {
        println!("Your system is {} compatible with charlatan", "NOT".red());
        exit(1);
    }

    println!("Which version of julia do you wish to install?");
    println!("1) {}", "Stable".green());
    println!("2) {}", "Nightly".yellow());
    println!("3) {}", "LTS".blue());
    println!("You can either select a number or write the version :)");

    let version = scanln();

    match version.as_str() {
        "1" | "stable" | "Stable" | "STABLE" => {
            println!("[{}] Julia Stable selected...", "✔".green());
            download(1)
        }
        "2" | "nightly" | "Nightly" | "NIGHTLY" => {
            println!("[{}] Julia Nightly selected...", "✔".green());
            download(2)
        }
        "3" | "lts" | "Lts" | "LTS" => {
            println!("[{}] Julia LTS selected...", "✔".green());
            download(3)
        }
        _ => {
            eprintln!("Invalid option: {}", version.red());
        }
    };
}

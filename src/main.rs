use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("charlatan")
        .subcommand(
            App::new("self")
                .about("manage charlatan itself")
                .arg(Arg::with_name("update").help("update charlatan"))
                .arg(Arg::with_name("remove").help("uninstalls charlatan"))
                .arg(Arg::with_name("uninstall").help("uninstalls charlatan")),
        )
        .version("0.1.0")
        .about("Unnoficial Julia toolchain installer")
        .long_about(
            "A small program written in Rust that I made to avoid using
                external bash scripts and manage easily julia versions and
                toolkits.
                ",
        )
        .author("VentGrey")
        .bin_name("charlatan")
        .get_matches();
}

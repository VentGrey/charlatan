use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("charlatan")
        .subcommand(
            SubCommand::with_name("self")
                .about("Used to handle charlatan itself")
                .arg(
                    Arg::with_name("action")
                        .help(
                            "Can be either:
                        update: Downloads the latest charlatan binary
                        remove / uninstall: Uninstalls charlatan",
                        )
                        .index(1)
                        .required(true),
                ),
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

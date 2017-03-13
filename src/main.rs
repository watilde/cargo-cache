#[macro_use]
extern crate clap;
extern crate cargo;

use clap::{Arg, App, AppSettings, SubCommand};

fn main() {
    let m = App::new("cargo-cache")
        .about("A third-party cargo extension to manipulates packages cache")
        .version(concat!("v", crate_version!()))
        .bin_name("cargo")
        .settings(&[AppSettings::SubcommandRequired])
        .subcommand(SubCommand::with_name("cache")
            .about("A third-party cargo extension to manipulates packages cache")
            .args(&[
                Arg::with_name("ls")
                    .help("Show all data in the cache")
                    .index(1),
                Arg::with_name("clean")
                    .help("Delete data out of the cache folder")
                    .index(2)
            ]))
      .get_matches();
}

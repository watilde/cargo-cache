#[macro_use]
extern crate clap;
extern crate cargo;

use clap::{Arg, App, AppSettings, SubCommand};

use std::process;

fn main() {
    let m = App::new("cargo-cache")
        .about("A third-party cargo extension to manipulates packages cache")
        .version(concat!("v", crate_version!()))
        .bin_name("cargo")
        .settings(&[AppSettings::SubcommandRequired])
        .subcommand(SubCommand::with_name("cache")
            .about("A third-party cargo extension to manipulates packages cache")
            .arg(Arg::with_name("CRATE")
              .help("The name of the crate you would like to open")
              .required(true)
              .index(1)))
      .get_matches();
  let crate_name = m.subcommand_matches("open").unwrap().value_of("CRATE").unwrap();
  println!("{:?} months in a year.", 12);
}

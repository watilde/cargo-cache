#[macro_use]
extern crate clap;
extern crate cargo;

use clap::{Arg, App, SubCommand};

fn main() {
    let matches = App::new("cargo")
        .subcommand(SubCommand::with_name("cache")
            .version(concat!("v", crate_version!()))
            .author("Daijiro Wachi <daijiro.wachi@gmail.com>")
            .about("A third-party cargo extension to manipulates packages cache")
            .usage("cargo cache [ARGS]")
            .args(&[
                Arg::with_name("ls")
                    .help("Show all data in the cache"),
                Arg::with_name("clean")
                    .help("Delete data out of the cache folder")
            ]))
      .get_matches();

    if matches.is_present("ls") {
        println!("Awesomeness is turned on");
    }
}

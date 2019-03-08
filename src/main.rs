extern crate log;
extern crate env_logger;

extern crate clap;
use clap::{Arg, App};
use std::process::exit;

mod cmd;

fn main() {
    env_logger::init();

    let app = App::new("pretty-cue")
        .version("0.0.1")
        .author("i2tsuki <github.com/i2tsuki>")
        .about("pretty-cue is pretty formatter for cuesheet")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input cue file to read")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("output")
                .short("-o")
                .long("--output")
                .help("Sets the output cue file to write")
                .multiple(false)
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("overwrite")
                .long("--overwrite")
                .help("Overwrite cue file")
                .required(false),
        );

    match cmd::exec(app) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("{}", err);
            exit(1);
        }
    }
}

#[macro_use]
extern crate log;
extern crate env_logger;

extern crate clap;
use clap::{Arg, App};

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use cue_sys::PTI;
use cue::cd::CD;
use cue::rem::RemType;

fn main() -> std::io::Result<()> {
    env_logger::init();

    let matches = App::new("pretty-cue")
        .version("0.0.1")
        .author("i2tsuki <github.com/i2tsuki>")
        .about("pretty-cue is pretty formatter for cuesheet")
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input cue file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input = matches.value_of("input").unwrap();
    let mut file = File::open(input)?;
    let mut buf_reader = BufReader::new(file);
    let mut cue_sheet = String::new();
    buf_reader.read_to_string(&mut cue_sheet)?;

    let cd = CD::parse(cue_sheet).unwrap();
    println!(
        "PERFORMER \"{}\"",
        cd.get_cdtext().read(PTI::Performer).unwrap()
    );
    println!("TITLE \"{}\"", cd.get_cdtext().read(PTI::Title).unwrap());

    println!(
        "REM DATE \"{}\"",
        cd.get_rem().read(RemType::Date as usize).unwrap()
    );
    println!("GENRE \"{}\"", cd.get_cdtext().read(PTI::Genre).unwrap());
    println!("FILE \"{}\" WAVE", cd.tracks()[0].get_filename());


    for (index, track) in cd.tracks().iter().enumerate() {
        println!("  TRACK {:>02} AUDIO", index + 1);
        println!(
            "    TITLE \"{}\"",
            track.get_cdtext().read(PTI::Title).unwrap()
        );
        println!(
            "    PERFORMER \"{}\"",
            track.get_cdtext().read(PTI::Performer).unwrap()
        );

        // ToDo: Use `time_frame_to_msf(long frame, int *m, int *s, int *f)` to convert frame to msf
        // Ref:https://github.com/lipnitsk/libcue/blob/cbbde79f64042bef87f5c8b7661845525a04c97e/time.c#L26
        let index00_min = track.get_start() as u32 / 75 / 60;
        let index00_sec = track.get_start() as u32 / 75 % 60;
        let index00_frame = track.get_start() as u32 % 75;

        let index01_min = (track.get_start() as u32 + track.get_index(1) as u32) / 75 / 60;
        let index01_sec = (track.get_start() as u32 + track.get_index(1) as u32) / 75 % 60;
        let index01_frame = (track.get_start() as u32 + track.get_index(1) as u32) % 75;

        if index != 0 {
            println!(
            "    INDEX 00 {0:>02}:{1:>02}:{2:>02}",
            index00_min,
            index00_sec,
            index00_frame,
            );
        }
        println!(
            "    INDEX 01 {0:>02}:{1:>02}:{2:>02}",
            index01_min,
            index01_sec,
            index01_frame,
        );
    }
    Ok(())
}

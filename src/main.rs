extern crate log;
extern crate env_logger;

extern crate clap;
use clap::{Arg, App};

use std::fs::File;
use std::io::{BufReader, BufWriter};
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
        .get_matches();

    let input = matches.value_of("INPUT").unwrap();
    let in_file = File::open(input)?;
    let out_file: Box<Write> = match matches.value_of("output") {
        Some(output) => Box::new(File::create(output)?),
        None => Box::new(std::io::stdout()),
    };
    let mut buf_reader = BufReader::new(in_file);
    let mut buf_writer = BufWriter::new(out_file);
    let mut cue_sheet = String::new();
    buf_reader.read_to_string(&mut cue_sheet)?;

    let cd = CD::parse(cue_sheet).unwrap();
    buf_writer
        .write(
            format!(
                "PERFORMER \"{}\"\n",
                cd.get_cdtext().read(PTI::Performer).unwrap()
            ).as_bytes(),
        )
        .unwrap();
    buf_writer
        .write(
            format!("TITLE \"{}\"\n", cd.get_cdtext().read(PTI::Title).unwrap()).as_bytes(),
        )
        .unwrap();

    buf_writer
        .write(
            format!(
                "REM DATE \"{}\"\n",
                cd.get_rem().read(RemType::Date as usize).unwrap()
            ).as_bytes(),
        )
        .unwrap();
    buf_writer
        .write(
            format!("GENRE \"{}\"\n", cd.get_cdtext().read(PTI::Genre).unwrap()).as_bytes(),
        )
        .unwrap();
    buf_writer
        .write(
            format!("FILE \"{}\" WAVE\n", cd.tracks()[0].get_filename()).as_bytes(),
        )
        .unwrap();

    for (index, track) in cd.tracks().iter().enumerate() {
        buf_writer
            .write(format!("  TRACK {:>02} AUDIO\n", index + 1).as_bytes())
            .unwrap();
        buf_writer
            .write(
                format!(
                    "    TITLE \"{}\"\n",
                    track.get_cdtext().read(PTI::Title).unwrap()
                ).as_bytes(),
            )
            .unwrap();
        buf_writer
            .write(
                format!(
                    "    PERFORMER \"{}\"\n",
                    track.get_cdtext().read(PTI::Performer).unwrap()
                ).as_bytes(),
            )
            .unwrap();

        // ToDo: Use `time_frame_to_msf(long frame, int *m, int *s, int *f)` to convert frame to msf
        // Ref:https://github.com/lipnitsk/libcue/blob/cbbde79f64042bef87f5c8b7661845525a04c97e/time.c#L26
        let index00_min = track.get_start() as u32 / 75 / 60;
        let index00_sec = track.get_start() as u32 / 75 % 60;
        let index00_frame = track.get_start() as u32 % 75;

        let index01_min = (track.get_start() as u32 + track.get_index(1) as u32) / 75 / 60;
        let index01_sec = (track.get_start() as u32 + track.get_index(1) as u32) / 75 % 60;
        let index01_frame = (track.get_start() as u32 + track.get_index(1) as u32) % 75;

        if index != 0 {
            buf_writer
                .write(
                    format!(
                "    INDEX 00 {0:>02}:{1:>02}:{2:>02}\n",
                index00_min,
                index00_sec,
                index00_frame,
            ).as_bytes(),
                )
                .unwrap();
        }
        buf_writer
            .write(
                format!(
            "    INDEX 01 {0:>02}:{1:>02}:{2:>02}\n",
            index01_min,
            index01_sec,
            index01_frame,
        ).as_bytes(),
            )
            .unwrap();
    }
    buf_writer.flush().unwrap();
    Ok(())
}

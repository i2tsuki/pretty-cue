#[macro_use]
extern crate log;
extern crate env_logger;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use log::Level;

use cue_sys::PTI;
use cue::cd::CD;
use cue::rem::RemType;

use chrono::{NaiveTime, Timelike};

fn main() -> std::io::Result<()> {
    env_logger::init();

    let mut file = File::open("file.cue")?;
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
        let index00_dt =
            NaiveTime::from_num_seconds_from_midnight(track.get_start() as u32 / 75, 0);
        let index00_frame = track.get_start() as u32 % 75;

        let index01_dt = NaiveTime::from_num_seconds_from_midnight(
            (track.get_start() as u32 + track.get_index(1) as u32) / 75,
            0,
        );
        let index01_frame = (track.get_start() as u32 + track.get_index(1) as u32) % 75;
        if index != 0 {
            println!(
            "    INDEX 00 {0:>02}:{1:>02}:{2:>02}",
            index00_dt.minute() + index00_dt.hour() * 60,
            index00_dt.second(),
            index00_frame,
            );
        }
        println!(
            "    INDEX 01 {0:>02}:{1:>02}:{2:>02}",
            index01_dt.minute() + index01_dt.hour() * 60,
            index01_dt.second(),
                index01_frame,
        );
    }
    Ok(())
}

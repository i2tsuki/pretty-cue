pub mod exec;

use std::fmt::{Display, Formatter};

use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;

use cue_sys::PTI;
use cue::cd::CD;
use cue::rem::RemType;

#[derive(Debug)]
pub enum CmdError {
    File(std::io::Error),
}

impl From<std::io::Error> for CmdError {
    fn from(err: std::io::Error) -> CmdError {
        CmdError::File(err)
    }
}

impl Display for CmdError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "pretty-cue: command failed")
    }
}

pub fn exec(app: clap::App) -> Result<(), CmdError> {
    let matches = app.get_matches();
    let in_file = File::open(matches.value_of("INPUT").unwrap())?;
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
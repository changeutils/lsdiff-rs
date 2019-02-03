//!
//! The Lsdiff binary.
//!

use std::{fs, io};

#[derive(Debug)]
enum Error {
    Reading(io::Error),
    Lsdiff(lsdiff_rs::Error),
}

fn main() -> Result<(), Error> {
    let args = clap::App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            clap::Arg::with_name("patch")
                .help("The patch")
                .index(1)
                .value_name("PATCH")
                .takes_value(true)
                .required(true),
        )
        .get_matches();

    let patch = args.value_of("patch").unwrap();
    let patch = fs::read_to_string(patch).map_err(Error::Reading)?;

    for entry in lsdiff_rs::process(&patch).map_err(Error::Lsdiff)? {
        println!(
            "{} -> {} ({}, {})",
            entry.input_path, entry.output_path, entry.start_line, entry.lines_count
        );
    }

    Ok(())
}

use clap::{Arg, Command};
use std::fs;
use std::io;
use std::io::Read;

type BoxErr = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxErr> {
    let matches = Command::new(clap::crate_name!())
        .arg(Arg::new("filename").takes_value(true).required(true))
        .arg(Arg::new("section-name").takes_value(true).required(true))
        .get_matches();
    let filename = matches.value_of("filename").expect("no filename provided");
    let section_name = matches
        .value_of("section-name")
        .expect("no section-name provided");

    let mut bytes =
        fs::read(filename).map_err(|err| format!("failed to read {}: {}", filename, err))?;

    let mut input_buffer = Vec::new();
    io::stdin().read_to_end(&mut input_buffer)?;

    wasm_gen::write_custom_section(&mut bytes, section_name, &input_buffer);

    fs::write(format!("{}.out", filename), bytes)?;
    Ok(())
}

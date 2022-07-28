use clap::{Arg, Command};
use pretty_hex::pretty_hex;
use std::fs;
use std::io;
use std::io::Read;
use std::str;

type BoxErr = Box<dyn std::error::Error>;

fn main() -> Result<(), BoxErr> {
    let matches = Command::new(clap::crate_name!())
        .arg(Arg::new("filename").takes_value(true).required(true))
        .subcommand(
            Command::new("add")
                .about("Add a new custom section")
                .arg(Arg::new("section-name").takes_value(true).required(true)),
        )
        .subcommand(Command::new("list").about("List custom sections"))
        .subcommand(
            Command::new("show")
                .about("Show a specific custom section")
                .arg(Arg::new("section-name").takes_value(true).required(true)),
        )
        .subcommand_required(true)
        .get_matches();

    let filename = matches.value_of("filename").expect("no filename provided");

    if let Some(matches) = matches.subcommand_matches("add") {
        let section_name = matches
            .value_of("section-name")
            .expect("no section-name provided");

        add_custom_section(filename, section_name)
    } else if let Some(_matches) = matches.subcommand_matches("list") {
        list_custom_sections(filename)
    } else if let Some(matches) = matches.subcommand_matches("show") {
        let section_name = matches
            .value_of("section-name")
            .expect("no section-name provided");

        show_custom_sections(filename, section_name)
    } else {
        unreachable!("subcommand expected")
    }
}

fn list_custom_sections(filename: &str) -> Result<(), BoxErr> {
    let bytes =
        fs::read(filename).map_err(|err| format!("failed to read {}: {}", filename, err))?;

    let parser = wasmparser::Parser::new(0);

    for payload in parser.parse_all(&bytes) {
        if let wasmparser::Payload::CustomSection { name, data, .. } = payload? {
            println!("Section `{}` ({} bytes)", name, data.len());
        }
    }

    Ok(())
}

fn show_custom_sections(filename: &str, section_name: &str) -> Result<(), BoxErr> {
    let bytes =
        fs::read(filename).map_err(|err| format!("failed to read {}: {}", filename, err))?;

    let parser = wasmparser::Parser::new(0);

    for payload in parser.parse_all(&bytes) {
        if let wasmparser::Payload::CustomSection { name, data, .. } = payload? {
            if name == section_name {
                println!("Section `{}` ({} bytes):", name, data.len());
                println!("{}", pretty_hex(&data));
                return Ok(());
            }
        }
    }

    Err(format!("Section `{}` not found", section_name).into())
}

fn add_custom_section(filename: &str, section_name: &str) -> Result<(), BoxErr> {
    let mut bytes =
        fs::read(filename).map_err(|err| format!("failed to read {}: {}", filename, err))?;

    let mut input_buffer = Vec::new();
    io::stdin().read_to_end(&mut input_buffer)?;

    wasm_gen::write_custom_section(&mut bytes, section_name, &input_buffer);

    fs::write(format!("{}.out", filename), bytes)?;

    Ok(())
}

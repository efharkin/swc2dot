use std::fs::File;
use std::io::Write;

mod cli_parser;
mod components;
mod config;
mod swc_parser;
mod writer;

use cli_parser::{get_cli_arguments, get_filename_without_extension};
use components::Graph;
use config::Config;
use swc_parser::parse_file;
use writer::{ConfiguredToDot, Indent};

fn main() {
    let cli_matches = get_cli_arguments();
    let mut config: Config;
    match Config::new() {
        Ok(c) => config = c,
        _ => panic!("Could not load default config"),
    }
    match cli_matches.value_of("config") {
        Some(config_file) => {
            config.try_overload_from_file(&config_file.to_string());
        }
        None => {}
    }

    let input_file_name = cli_matches
        .value_of("INPUT")
        .expect("Required argument INPUT is missing.")
        .to_string();
    let swcneuron = parse_file(input_file_name.clone());
    let graphneuron = Graph::from(swcneuron);

    // Get the name of the output file
    // Fall back to the name of the input file with .dot suffix if none is provided.
    let mut output_file_name: String;
    match cli_matches.value_of("output") {
        Some(file_name) => output_file_name = file_name.to_string(),
        None => {
            output_file_name = get_filename_without_extension(input_file_name);
            output_file_name.push_str(".dot");
        }
    }

    let mut f = File::create(&output_file_name).expect(&format!(
        "Could not create output file {}.",
        &output_file_name
    ));
    f.write(
        &graphneuron
            .to_dot(false, Indent::flat(0), &config)
            .into_bytes(),
    );
    f.flush();
}

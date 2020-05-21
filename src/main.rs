use std::fs::File;
use std::io::Write;

mod cli_parser;
mod swc_parser;
mod components;
mod writer;
mod config;

use cli_parser::get_cli_arguments;
use swc_parser::parse_file;
use components::Graph;
use writer::{ConfiguredToDot, Indent};
use config::Config;


fn main() {
    let cli_matches = get_cli_arguments();
    let mut config: Config;
    match Config::new() {
        Ok(c) => config = c,
        _ => panic!("Could not load default config")
    }
    match cli_matches.value_of("config") {
        Some(config_file) => {config.try_overload_from_file(&config_file.to_string());},
        None => {},
    }

    let swcneuron = parse_file(cli_matches.value_of("INPUT").expect("Could not get input.").to_string());
    let graphneuron = Graph::from(swcneuron);

    let mut f = File::create(cli_matches.value_of("output").expect("Could not get output.")).expect("Could not create output file.");
    f.write(&graphneuron.to_dot(false, Indent::flat(0), &config).into_bytes());
    f.flush();
}

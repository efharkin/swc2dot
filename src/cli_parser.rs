use clap::{App, Arg, ArgMatches};

pub fn get_cli_arguments<'a>() -> ArgMatches<'a> {
    App::new("swc2dot")
              .version("0.1.0")
              .author("Emerson Harkin <emerson.f.harkin@gmail.com>")
              .about("Convert SWC neuron morphologies to DOT graph language.")
              .arg(Arg::with_name("output")
                   .short("o")
                   .long("output")
                   .help("Output file for morphology in DOT format")
                   .value_name("FILE")
                   .takes_value(true)
              )
              .arg(Arg::with_name("INPUT")
                   .help("SWC neuron morphology file to use as input")
                   .index(1)
                   .required(true)
              )
              .arg(Arg::with_name("config")
                  .short("c")
                  .long("config")
                  .help("Configuration file for node attributes.")
                  .value_name("FILE")
                  .takes_value(true)
              )
              .get_matches()
}

/// Get a filename with the extension removed.
///
/// If the file does not have an extension, the whole filename is returned.
pub fn get_filename_without_extension(filename: String) -> String {
    let extension_start_position: usize;
    match filename.rfind('.') {
        Some(position) => extension_start_position = position,
        None => extension_start_position = filename.len()
    }
    return filename[0..extension_start_position].to_string();
}

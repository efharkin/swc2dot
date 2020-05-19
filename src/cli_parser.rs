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
                   .required(true)
              )
              .arg(Arg::with_name("INPUT")
                   .help("SWC neuron morphology file to use as input")
                   .index(1)
                   .required(true)
              )
              .get_matches()
}

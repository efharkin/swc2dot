mod parser;
mod components;
mod writer;

use parser::parse_file;
use components::Graph;
use writer::ToDot;

use std::fs::File;
use std::io::Write;

fn main() {
    let swcneuron = parse_file("N3_6.CNG.swc".to_string());
    let graphneuron = Graph::from(swcneuron);

    let mut f = File::create("dot_neuron.dot").expect("Could not create file.");
    f.write(&graphneuron.to_dot().into_bytes());
    f.flush();
}

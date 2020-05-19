mod parser;
mod components;

use parser::*;
use components::Graph;

fn main() {
    let swcneuron = parse_file("N3_6.CNG.swc".to_string());
    let graphneuron = Graph::from(swcneuron);
}

mod parser;

use parser::*;

fn main() {
    let compartments = parse_file("N3_6.CNG.swc".to_string());
}

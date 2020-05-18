use std::io::{BufReader, BufRead};
use std::fs::File;

pub fn parse_file(file_name: String) -> Vec<SWCCompartment> {
    let reader = get_file_reader(file_name);
    parse_lines(reader)
}

fn get_file_reader(file_name: String) -> BufReader<File> {
    let f = File::open(file_name).expect("Could not open file.");
    let reader = BufReader::new(f);
    return reader;
}

fn parse_lines(reader: BufReader<File>) -> Vec<SWCCompartment> {
    let mut compartments = Vec::<SWCCompartment>::with_capacity(64);

    for line in reader.lines() {
        match line {
            Ok(l) => {
                if l.chars().next().unwrap() == '#' {
                    // Line is a comment
                    continue;
                } else {
                    let compartment = parse_line(l);
                    compartments.push(compartment);
                }
            },
            _ => panic!("Error reading line.")
        }
    }

    return compartments;
}

fn parse_line(line: String) -> SWCCompartment {
    let specs: Vec<&str> = line.split_whitespace().collect();
    assert!(specs.len() == 7);

    let id = specs[0].parse::<usize>().expect("Could not parse id");
    let compartment_kind = SWCCompartmentKind::from(specs[1].parse::<usize>().expect("Could not parse compartmentkind"));
    let position = Point {
        x: specs[2].parse::<f64>().expect("Could not parse x position"),
        y: specs[3].parse::<f64>().expect("Could not parse y position"),
        z: specs[4].parse::<f64>().expect("Could not parse z position")
    };
    let radius = specs[5].parse::<f64>().expect("Could not parse radius");

    let parent_id: Option<usize>;
    if specs[6].chars().next().unwrap() == '-' {
        // Negative parent id means there is no parent.
        parent_id = None;
    } else {
        parent_id = Some(specs[6].parse::<usize>().expect(&format!("Could not parse parent id {}", specs[6])));
    }

    SWCCompartment::new(id, compartment_kind, position, radius, parent_id)
}

pub struct SWCCompartment {
    pub id: usize,
    pub kind: SWCCompartmentKind,
    pub position: Point,
    pub radius: f64,
    pub parent_id: Option<usize>
}

impl SWCCompartment {
    pub fn new(id: usize, kind: SWCCompartmentKind, position: Point, radius: f64, parent_id: Option<usize>) -> SWCCompartment {
        SWCCompartment {
            id: id,
            kind: kind,
            position: position,
            radius: radius,
            parent_id: parent_id
        }
    }
}

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub enum SWCCompartmentKind {
    Undefined,
    Soma,
    Axon,
    Dendrite,
    ApicalDendrite,
    Custom
}

impl From<usize> for SWCCompartmentKind {
    fn from(kind: usize) -> SWCCompartmentKind {
        match kind {
            0 => SWCCompartmentKind::Undefined,
            1 => SWCCompartmentKind::Soma,
            2 => SWCCompartmentKind::Axon,
            3 => SWCCompartmentKind::Dendrite,
            4 => SWCCompartmentKind::ApicalDendrite,
            num if num >= 5 => SWCCompartmentKind::Custom,
            _ => panic!("kind is not usize")
        }
    }
}


use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::{BTreeMap, btree_map::{Entry, Iter}};

pub fn parse_file(file_name: String) -> SWCNeuron {
    let reader = get_file_reader(file_name);
    match parse_lines(reader) {
        Ok(neuron) => neuron,
        Err(msg) => panic!(msg)
    }
}

fn get_file_reader(file_name: String) -> BufReader<File> {
    let f = File::open(file_name).expect("Could not open file.");
    let reader = BufReader::new(f);
    return reader;
}

fn parse_lines(reader: BufReader<File>) -> Result<SWCNeuron, String> {
    let mut neuron = SWCNeuron::new();

    for line in reader.lines() {
        match parse_line(line.expect("Could not read line."))? {
            SWCLine::SWCCompartment(compartment) => neuron.try_insert(compartment)?,
            SWCLine::Comment(_) => {}
        }
    }

    return Ok(neuron);
}

fn parse_line(line: String) -> Result<SWCLine, String> {
    let mut parse_result: SWCLine;

    if line.chars().next().unwrap() == '#' {
        // Parse line as a comment, causing parse_result to be
        // SWCLine::Comment
        parse_result = SWCLine::Comment(line);
    } else {
        // Parse line as a compartment, causing parse_result to be
        // SWCLine::SWCCompartment
        parse_result = SWCLine::SWCCompartment(parse_line_as_compartment(line)?);
    }

    return Ok(parse_result);
}

enum SWCLine {
    SWCCompartment(SWCCompartment),
    Comment(String)
}

fn parse_line_as_compartment(line: String) -> Result<SWCCompartment, String> {
    let specs: Vec<&str> = line.split_whitespace().collect();

    // Check number of space-delimited items.
    if specs.len() != 7 {
        return Err(
            format!("Expected 7 space-delimited items in compartment line,
                got {} items instead.", specs.len())
        )
    }

    let id: usize;
    match specs[0].parse::<usize>() {
        Ok(parsed_id) => id = parsed_id,
        Err(_) => {
            return Err(format!("Could not parse {} as a compartment id.", specs[0]))
        }
    }
    let compartment_kind = SWCCompartmentKind::from(specs[1].parse::<usize>().expect("Could not parse compartmentkind"));
    let position = Point {
        x: specs[2].parse::<f64>().expect("Could not parse x position"),
        y: specs[3].parse::<f64>().expect("Could not parse y position"),
        z: specs[4].parse::<f64>().expect("Could not parse z position")
    };
    let radius = specs[5].parse::<f64>().expect("Could not parse radius");

    let parent_id: Option<usize>;
    if specs[6].chars().next().unwrap() == '-' {
        // Negative parent id means there is no parent; this is the root of the
        // neuron graph.
        parent_id = None;
    } else {
        let parsed_parent_id = specs[6].parse::<usize>().expect(&format!("Could not parse parent id {}", specs[6]));
        if parsed_parent_id >= id {
            return Err(
                format!("Expected parent_id for compartment {} to be less than {},
                    got {} instead.", id, id, parsed_parent_id)
            )
        }
        parent_id = Some(parsed_parent_id);
    }

    return Ok(SWCCompartment::new(id, compartment_kind, position, radius, parent_id));
}

pub struct SWCNeuron {
    compartments: BTreeMap<usize, SWCCompartment>
}

impl SWCNeuron {
    fn new() -> SWCNeuron {
        SWCNeuron {
            compartments: BTreeMap::<usize, SWCCompartment>::new()
        }
    }

    fn try_insert(&mut self, compartment: SWCCompartment) -> Result<(), String> {
        match self.compartments.entry(compartment.id) {
            Entry::Occupied(_) => Err(
                format!("More than one compartment with id {} exists", compartment.id)
            ),
            Entry::Vacant(entry) => {
                entry.insert(compartment);
                Ok(())
            }
        }
    }

    pub fn iter(&self) -> Iter<usize, SWCCompartment> {
        self.compartments.iter()
    }
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

/// Types of compartment defined by the most basic version of the SWC standard.
#[derive(Copy, Clone)]
pub enum SWCCompartmentKind {
    Undefined,
    Soma,
    Axon,
    Dendrite,
    ApicalDendrite,
    Custom
}

impl SWCCompartmentKind {
    pub fn iter() -> SWCCompartmentKindIterator {
        SWCCompartmentKindIterator::new()
    }
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

impl IntoIterator for SWCCompartmentKind {
    type Item = SWCCompartmentKind;
    type IntoIter = SWCCompartmentKindIterator;

    fn into_iter(self) -> SWCCompartmentKindIterator {
        SWCCompartmentKindIterator::new()
    }
}

/// Iterator over variants of `SWCCompartmentKind`
pub struct SWCCompartmentKindIterator {
    kinds: [SWCCompartmentKind; 6],
    ptr: usize
}

impl SWCCompartmentKindIterator {
    pub fn new() -> SWCCompartmentKindIterator {
        SWCCompartmentKindIterator {
            kinds: [
                SWCCompartmentKind::Undefined,
                SWCCompartmentKind::Soma,
                SWCCompartmentKind::Axon,
                SWCCompartmentKind::Dendrite,
                SWCCompartmentKind::ApicalDendrite,
                SWCCompartmentKind::Custom,
            ],
            ptr: 0
        }
    }
}

/// Iterate over variants of `SWCCompartmentKind` in no particular order.
impl Iterator for SWCCompartmentKindIterator {
    type Item = SWCCompartmentKind;

    /// Get the next SWCComparmentKind
    fn next(&mut self) -> Option<SWCCompartmentKind> {
        let result;
        if self.ptr < self.kinds.len() {
            result = Some(self.kinds[self.ptr]);
            self.ptr += 1;
        } else {
            result = None;
        }
        return result;
    }
}


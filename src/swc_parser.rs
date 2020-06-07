use std::collections::{
    btree_map::{Entry, Iter},
    BTreeMap,
};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn parse_file(file_name: String) -> SWCNeuron {
    let reader = get_file_reader(file_name);
    match parse_lines(reader) {
        Ok(neuron) => neuron,
        Err(msg) => panic!(msg),
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
            SWCLine::Comment(_) => {},
            SWCLine::Blank => {}
        }
    }

    return Ok(neuron);
}

fn parse_line(line: String) -> Result<SWCLine, String> {
    let trimmed_line = line.trim();  // Remove leading and trailing whitespace.

    let parse_result: SWCLine;
    if trimmed_line.is_empty() {
        // Line is empty.
        parse_result = SWCLine::Blank;
    } else {
        // Line is not empty.

        if trimmed_line.chars().next().unwrap() == '#' {
            // Parse line as a comment, causing parse_result to be
            // SWCLine::Comment
            parse_result = SWCLine::Comment(trimmed_line.to_string());
        } else {
            // Parse line as a compartment, causing parse_result to be
            // SWCLine::SWCCompartment
            parse_result = SWCLine::SWCCompartment(parse_line_as_compartment(trimmed_line.to_string())?);
        }
    }

    return Ok(parse_result);
}

enum SWCLine {
    SWCCompartment(SWCCompartment),
    Comment(String),
    Blank,
}

fn parse_line_as_compartment(line: String) -> Result<SWCCompartment, String> {
    let specs: Vec<&str> = line.split_whitespace().collect();

    // Check number of space-delimited items.
    if specs.len() != 7 {
        return Err(format!(
            "Expected 7 space-delimited items in compartment line,
                got {} items instead.",
            specs.len()
        ));
    }

    let id: usize;
    match specs[0].parse::<usize>() {
        Ok(parsed_id) => id = parsed_id,
        Err(_) => return Err(format!("Could not parse {} as a compartment id.", specs[0])),
    }
    let compartment_kind = SWCCompartmentKind::from(
        specs[1]
            .parse::<usize>()
            .expect("Could not parse compartmentkind"),
    );
    let position = Point {
        x: specs[2].parse::<f64>().expect("Could not parse x position"),
        y: specs[3].parse::<f64>().expect("Could not parse y position"),
        z: specs[4].parse::<f64>().expect("Could not parse z position"),
    };
    let radius = specs[5].parse::<f64>().expect("Could not parse radius");

    let parent_id: Option<usize>;
    if specs[6].chars().next().unwrap() == '-' {
        // Negative parent id means there is no parent; this is the root of the
        // neuron graph.
        parent_id = None;
    } else {
        let parsed_parent_id = specs[6]
            .parse::<usize>()
            .expect(&format!("Could not parse parent id {}", specs[6]));
        if parsed_parent_id >= id {
            return Err(format!(
                "Expected parent_id for compartment {} to be less than {},
                    got {} instead.",
                id, id, parsed_parent_id
            ));
        }
        parent_id = Some(parsed_parent_id);
    }

    return Ok(SWCCompartment::new(
        id,
        compartment_kind,
        position,
        radius,
        parent_id,
    ));
}

#[cfg(test)]
mod parse_line_as_compartment_tests {
    use super::*;

    /// An SWC line should have exactly seven space-delimited items. These
    /// tests ensure that lines are parsed as the correct length.
    mod line_length_tests {
        use super::*;

        #[test]
        fn too_many_space_delimited_items_raises_error() {
            let line = "2 3 4 5 6 7 1 1".to_string();
            match parse_line_as_compartment(line) {
                Ok(_) => assert!(false),
                Err(msg) => assert!(msg.contains("got 8 items"))
            }
        }

        #[test]
        fn too_few_space_delimited_items_raises_error() {
            let line = "2 3 4 5 6 7".to_string();
            match parse_line_as_compartment(line) {
                Ok(_) => assert!(false),
                Err(msg) => assert!(msg.contains("got 6 items"))
            }
        }

        #[test]
        fn leading_space_does_not_trigger_error() {
            let line = " 2 3 4 5 6 7 1".to_string();
            match parse_line_as_compartment(line) {
                Ok(_) => assert!(true),
                Err(_) => assert!(false)
            }
        }

        #[test]
        fn trailing_space_does_not_trigger_error() {
            let line = "2 3 4 5 6 7 1 ".to_string();
            match parse_line_as_compartment(line) {
                Ok(_) => assert!(true),
                Err(_) => assert!(false)
            }
        }

        #[test]
        fn extra_infix_spaces_do_not_trigger_error() {
            let line = "2 3   4  5 6     7 1".to_string();
            match parse_line_as_compartment(line) {
                Ok(_) => assert!(true),
                Err(_) => assert!(false)
            }
        }
    }

    #[cfg(test)]
    mod id {
        use super::*;

        #[test]
        fn first_item_is_id () {
            let trailing_values = " 2 3 4 5 6 7";
            for id in [10, 645, 938274].iter() {
                let mut swc_line = id.to_string();
                swc_line.push_str(trailing_values);
                let swc_compartment = parse_line_as_compartment(swc_line).unwrap();
                assert_eq!(swc_compartment.id, *id);
            }
        }

        #[test]
        fn position() {
            for (x, y, z) in [(1.2, 2.2, 3.7), (4.5, 5.5, 6.5), (-32.0, 125.333, -3.4)].iter() {
                let swc_line = format!("10 1 {} {} {} 5 6", x, y, z);
                let swc_compartment = parse_line_as_compartment(swc_line).unwrap();
                assert_eq!(swc_compartment.position, Point{x: *x, y: *y, z: *z});
            }
        }

        #[test]
        fn radius() {
            for rad in [4.3, 7.7, 9.9, 3.2].iter() {
                let swc_line = format!("10 1 3 3 3 {} 6", rad);
                let swc_compartment = parse_line_as_compartment(swc_line).unwrap();
                assert_eq!(swc_compartment.radius, *rad);
            }
        }

        #[test]
        fn positive_last_item_is_parent() {
            for parent_id in [2, 54, 893].iter() {
                let swc_line = format!("1000 1 3 3 3 3 {}", parent_id);
                let swc_compartment = parse_line_as_compartment(swc_line).unwrap();
                match swc_compartment.parent_id {
                    Some(parent) => assert_eq!(parent, *parent_id),
                    None => assert!(false, "Failed because no parent was found.")
                }
            }
        }

        #[test]
        fn negative_last_item_means_no_parent() {
            for parent_id in [-244, -2, -1].iter() {
                let swc_line = format!("1 1 3 3 3 3 {}", parent_id);
                let swc_compartment = parse_line_as_compartment(swc_line.clone()).unwrap();
                match swc_compartment.parent_id {
                    Some(_) => assert!(false, "A negative parent is no parent at all! Parent is not None for swc string `{}`", swc_line),
                    None => assert!(true)
                }
            }
        }
    }
}

pub struct SWCNeuron {
    compartments: BTreeMap<usize, SWCCompartment>,
}

impl SWCNeuron {
    fn new() -> SWCNeuron {
        SWCNeuron {
            compartments: BTreeMap::<usize, SWCCompartment>::new(),
        }
    }

    fn try_insert(&mut self, compartment: SWCCompartment) -> Result<(), String> {
        match self.compartments.entry(compartment.id) {
            Entry::Occupied(_) => Err(format!(
                "More than one compartment with id {} exists",
                compartment.id
            )),
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

#[cfg(test)]
mod swcneuron_tests {
    use super::*;

    #[test]
    fn insert_compartments_with_unique_ids() {
        // Create a neuron and insert a single root compartment.
        let mut neuron = SWCNeuron::new();
        let mut compartment = SWCCompartment::new(0, SWCCompartmentKind::Soma, Point{x: 0.0, y: 0.0, z: 0.0}, 0.5, None);
        neuron.try_insert(compartment.clone()).expect("Could not insert root node.");

        for compartment_id in [2, 5, 4, 7, 88, 903].iter() {
            compartment.parent_id = Some(0);
            compartment.id = *compartment_id;
            neuron.try_insert(compartment.clone()).expect(&format!("Could not insert compartment with unique id {}", compartment_id));
        }
    }

    #[test]
    fn insert_compartment_with_duplicate_ids_is_error() {
        // Create a neuron and insert a single root compartment.
        let mut neuron = SWCNeuron::new();
        let mut compartment = SWCCompartment::new(1, SWCCompartmentKind::Soma, Point{x: 0.0, y: 0.0, z: 0.0}, 0.5, None);
        neuron.try_insert(compartment.clone()).expect("Could not insert root node.");

        // Change all compartment attributes except id.
        compartment.radius += 1.0;
        compartment.position.x += 1.0;
        compartment.position.y += 1.0;
        compartment.position.z += 1.0;
        compartment.kind = SWCCompartmentKind::ApicalDendrite;
        compartment.parent_id = Some(0);

        // Since id is still the same, inserting compartment again is an error.
        match neuron.try_insert(compartment.clone()) {
            Ok(_) => assert!(false, "Inserting compartments with the same id should be an error"),
            Err(msg) => assert!(msg.to_lowercase().contains("more than one compartment with id 1"))
        }
    }
}

#[derive(Copy, Clone)]
pub struct SWCCompartment {
    pub id: usize,
    pub kind: SWCCompartmentKind,
    pub position: Point,
    pub radius: f64,
    pub parent_id: Option<usize>,
}

impl SWCCompartment {
    pub fn new(
        id: usize,
        kind: SWCCompartmentKind,
        position: Point,
        radius: f64,
        parent_id: Option<usize>,
    ) -> SWCCompartment {
        SWCCompartment {
            id: id,
            kind: kind,
            position: position,
            radius: radius,
            parent_id: parent_id,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Types of compartment defined by the most basic version of the SWC standard.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SWCCompartmentKind {
    Undefined,
    Soma,
    Axon,
    Dendrite,
    ApicalDendrite,
    Custom,
}

use std::fmt;
impl fmt::Display for SWCCompartmentKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SWCCompartmentKind::Undefined => write!(f, "undefined"),
            SWCCompartmentKind::Soma => write!(f, "somatic"),
            SWCCompartmentKind::Axon => write!(f, "axonal"),
            SWCCompartmentKind::Dendrite => write!(f, "(basal) dendritic"),
            SWCCompartmentKind::ApicalDendrite => write!(f, "apical dendritic"),
            SWCCompartmentKind::Custom => write!(f, "custom"),
        }
    }
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
            _ => panic!("kind is not usize"),
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
    ptr: usize,
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
            ptr: 0,
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

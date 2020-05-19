use std::convert::From;
use std::collections::BTreeMap;

use crate::parser::{SWCNeuron, SWCCompartment};

pub struct Vertex {
    data: SWCCompartment,
    children: Vec<usize>
}

impl Vertex {
    pub fn get_id(&self) -> usize {
        self.data.id
    }

    pub fn get_parent_id(&self) -> Option<usize> {
        self.data.parent_id
    }

    pub fn get_child_ids(&self) -> &Vec<usize> {
        &self.children
    }

    fn add_child(&mut self, child: &Vertex) {
        self.children.push(child.get_id());
    }
}

impl From<SWCCompartment> for Vertex {
    fn from(compartment: SWCCompartment) -> Vertex {
        Vertex {
            data: compartment,
            children: Vec::<usize>::with_capacity(4)
        }
    }
}

pub struct Graph {
    vertices: BTreeMap<usize, Vertex>
}

impl From<SWCNeuron> for Graph {
   fn from(neuron: SWCNeuron) -> Graph {
        let mut graph = Graph {
            vertices: BTreeMap::<usize, Vertex>::new()
        };

        for (_, compartment) in neuron.iter() {
            let vertex = Vertex::from(compartment.clone());

            match vertex.get_parent_id() {
                Some(parent_id) => {
                    // Preconditions that should be guaranteed by the parser:
                    // 1. The ID of the parent must be less than the ID of the child to comply with
                    //    SWC standard
                    // 2. It is invalid for a child to have a parent that does not exist.
                    debug_assert!(parent_id < vertex.get_id());
                    debug_assert!(graph.vertices.contains_key(&parent_id));

                    // Add vertex as a child of its parent.
                    let parent = graph.vertices.get_mut(&parent_id).unwrap();
                    parent.add_child(&vertex);
                },
                None => {}
            }

            debug_assert!(!graph.vertices.contains_key(&vertex.get_id()));
            graph.vertices.insert(vertex.get_id(), vertex);
        }

        return graph;
   }
}

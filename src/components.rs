use std::collections::{btree_map::Iter, BTreeMap};
use std::convert::From;

use crate::swc_parser::{SWCCompartment, SWCCompartmentKind, SWCNeuron};

#[derive(Clone)]
pub struct Vertex {
    data: SWCCompartment,
    children: Vec<usize>,
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

    pub fn get_kind(&self) -> SWCCompartmentKind {
        self.data.kind
    }

    fn add_child(&mut self, child: &Vertex) {
        self.children.push(child.get_id());
    }
}

impl From<SWCCompartment> for Vertex {
    fn from(compartment: SWCCompartment) -> Vertex {
        Vertex {
            data: compartment,
            children: Vec::<usize>::with_capacity(4),
        }
    }
}

pub struct Graph {
    vertices: BTreeMap<usize, Vertex>,
}

impl Graph {
    pub fn iter_vertices(&self) -> Iter<usize, Vertex> {
        self.vertices.iter()
    }

    pub fn iter_short_trees(&self) -> ShortTreeIter {
        let mut short_trees = Vec::with_capacity(self.vertices.len());
        for (id, vertex) in self.iter_vertices() {
            short_trees.push(ShortTree::from(vertex.clone()));
        }
        ShortTreeIter::new(short_trees)
    }

    pub fn len(&self) -> usize {
        self.vertices.len()
    }
}

impl From<SWCNeuron> for Graph {
    fn from(neuron: SWCNeuron) -> Graph {
        let mut graph = Graph {
            vertices: BTreeMap::<usize, Vertex>::new(),
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
                }
                None => {}
            }

            debug_assert!(!graph.vertices.contains_key(&vertex.get_id()));
            graph.vertices.insert(vertex.get_id(), vertex);
        }

        return graph;
    }
}

/// A tree of height 1.
///
/// In DOT language, a tree of height 1 can be declared in one line.
#[derive(Clone)]
pub struct ShortTree {
    root_id: usize,
    child_ids: Vec<usize>,
}

impl ShortTree {
    pub fn get_root_id(&self) -> usize {
        self.root_id
    }

    pub fn get_child_ids(&self) -> &Vec<usize> {
        &self.child_ids
    }
}

impl From<Vertex> for ShortTree {
    fn from(vertex: Vertex) -> ShortTree {
        ShortTree {
            root_id: vertex.get_id(),
            child_ids: vertex.get_child_ids().clone(),
        }
    }
}

#[derive(Clone)]
pub struct ShortTreeIter {
    trees: Vec<ShortTree>,
    ptr: usize,
}

impl ShortTreeIter {
    fn new(trees: Vec<ShortTree>) -> ShortTreeIter {
        ShortTreeIter {
            trees: trees,
            ptr: 0,
        }
    }
}

impl Iterator for ShortTreeIter {
    type Item = ShortTree;

    fn next(&mut self) -> Option<Self::Item> {
        let item;
        if self.ptr < self.trees.len() {
            item = Some(self.trees[self.ptr].clone());
        } else {
            item = None;
        }
        self.ptr += 1;
        return item;
    }
}

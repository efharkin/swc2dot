use std::cmp::max;

use itertools::Itertools;

use crate::components::{Graph, Vertex};

pub trait ToDot {
    fn to_dot(&self) -> String;
}

impl ToDot for Vertex {
    fn to_dot(&self) -> String {
        match self.get_child_ids().len() {
            0 => String::from(""),
            1 => format!("\n    {} -- {};", self.get_id(), self.get_child_ids()[0]),
            _ => format!("\n    {} -- {{{}}};", self.get_id(), self.get_child_ids().iter().format(", "))
        }
    }
}

impl ToDot for Graph {
    fn to_dot(&self) -> String {
        let mut graph_string = String::with_capacity(max(64 * self.len(), 5242880));

        graph_string.push_str("graph{");
        for (_, vertex) in self.iter() {
            graph_string.push_str(&vertex.to_dot())
        }
        graph_string.push_str("\n}");

        graph_string.shrink_to_fit();
        return graph_string;
    }
}

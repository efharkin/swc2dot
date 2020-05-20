use std::cmp::max;

use itertools::Itertools;

use crate::components::{Graph, Vertex, ShortTree};
use crate::config::Config;
use crate::swc_parser::SWCCompartmentKind;

pub trait ToDot {
    fn to_dot(&self, indent_level: u8, config: &Config) -> String;
}

pub fn indent(level: u8) -> String {
    let mut buf = String::with_capacity((level * 4) as usize);
    for _ in 0..level {
        buf.push_str(&"    ");
    }
    return buf;
}

impl ToDot for Vertex {
    /// Get a DOT representation of a single vertex.
    ///
    /// # Warning
    ///
    /// `config` options are currently not used.
    fn to_dot(&self, indent_level: u8, config: &Config) -> String {
        format!("\n{}{};", indent(indent_level), self.get_id())
    }
}

static graph_string_max_bufsize: usize = 5242880;

impl ToDot for Graph {
    fn to_dot(&self, indent_level: u8, config: &Config) -> String {
        let mut graph_string = String::with_capacity(max(64 * self.len(), graph_string_max_bufsize));

        graph_string.push_str("graph{");

        // Node configuration
        let mut buffers = VertexConfigBuffers::new(256);
        buffers.axon.push_str(&format!("{}{}", indent(indent_level + 2), config.get_config(SWCCompartmentKind::Axon).to_dot(indent_level + 2, config)));
        buffers.soma.push_str(&format!("{}{}", indent(indent_level + 2), config.get_config(SWCCompartmentKind::Soma).to_dot(indent_level + 2, config)));
        buffers.dendrite.push_str(&format!("{}{}", indent(indent_level + 2), config.get_config(SWCCompartmentKind::Dendrite).to_dot(indent_level + 2, config)));
        buffers.apicaldendrite.push_str(&format!("{}{}", indent(indent_level + 2), config.get_config(SWCCompartmentKind::ApicalDendrite).to_dot(indent_level + 2, config)));
        buffers.undefined.push_str(&format!("{}{}", indent(indent_level + 2), config.get_config(SWCCompartmentKind::Undefined).to_dot(indent_level + 2, config)));
        buffers.custom.push_str(&format!("{}{}", indent(indent_level + 2), config.get_config(SWCCompartmentKind::Custom).to_dot(indent_level + 2, config)));
        for (_, vertex) in self.iter_vertices() {
            match vertex.get_kind() {
                SWCCompartmentKind::Axon => buffers.axon.push_str(&vertex.to_dot(indent_level + 2, config)),
                SWCCompartmentKind::Soma => buffers.soma.push_str(&vertex.to_dot(indent_level + 2, config)),
                SWCCompartmentKind::Dendrite => buffers.dendrite.push_str(&vertex.to_dot(indent_level + 2, config)),
                SWCCompartmentKind::ApicalDendrite => buffers.apicaldendrite.push_str(&vertex.to_dot(indent_level + 2, config)),
                SWCCompartmentKind::Undefined => buffers.undefined.push_str(&vertex.to_dot(indent_level + 2, config)),
                SWCCompartmentKind::Custom => buffers.custom.push_str(&vertex.to_dot(indent_level + 2, config)),
            }
        }

        graph_string.push_str(&format!("\n{}{{{}\n{}}}\n{}{{{}\n{}}}\n{}{{{}\n{}}}", indent(1), buffers.axon, indent(1), indent(1), buffers.soma, indent(1), indent(1), buffers.dendrite, indent(1)));

        // Write edges
        for short_tree in self.iter_short_trees() {
            graph_string.push_str(&short_tree.to_dot(indent_level + 1, config))
        }
        graph_string.push_str("\n}");

        graph_string.shrink_to_fit();
        return graph_string;
    }
}

/// Struct for holding Vertex config strings
struct VertexConfigBuffers {
    axon: String,
    soma: String,
    dendrite: String,
    apicaldendrite: String,
    undefined: String,
    custom: String
}

impl VertexConfigBuffers {
    fn new(buffer_size: usize) -> VertexConfigBuffers {
        VertexConfigBuffers {
            axon: String::with_capacity(buffer_size),
            soma: String::with_capacity(buffer_size),
            dendrite: String::with_capacity(buffer_size),
            apicaldendrite: String::with_capacity(buffer_size),
            undefined: String::with_capacity(buffer_size),
            custom: String::with_capacity(buffer_size),
        }
    }
}

impl ToDot for ShortTree {
    /// Get DOT representation of a rooted tree of depth 1.
    ///
    /// Rooted trees of depth 1 can be written in one line in DOT.
    ///
    /// # Warning
    ///
    /// `config` options are currently not used.
    fn to_dot(&self, indent_level: u8, config: &Config) -> String {
        match self.get_child_ids().len() {
            0 => format!("\n{}{};", indent(indent_level), self.get_root_id()),
            1 => format!("\n{}{} -- {};", indent(indent_level), self.get_root_id(), self.get_child_ids()[0]),
            _ => format!("\n{}{} -- {{{}}};", indent(indent_level), self.get_root_id(), self.get_child_ids().iter().format(", "))
        }
    }
}

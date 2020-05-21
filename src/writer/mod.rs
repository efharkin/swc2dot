use std::cmp::max;

use itertools::Itertools;

use crate::components::{Graph, Vertex, ShortTree};
use crate::config::Config;
use crate::swc_parser::SWCCompartmentKind;

mod string_buffer;

pub use string_buffer::StringBuffer;

/// Get a `String` representation of an object in DOT format.
pub trait ToDot {
    fn to_dot(&self, leading_newline: bool, indent_level: u8) -> String;
}

static INDENT_SIZE: u8 = 4;

/// Get a `String` of spaces for indenting.
pub fn indent(level: u8) -> String {
    let mut buf = String::with_capacity((level * INDENT_SIZE) as usize);
    for _ in 0..level {
        buf.push_str("    ");
    }
    return buf;
}

impl ToDot for Vertex {
    /// Get a DOT representation of a single vertex.
    fn to_dot(&self, leading_newline: bool, indent_level: u8) -> String {
        let mut vertex_str = StringBuffer::new(leading_newline, indent_level, 32);
        vertex_str.push_str(&self.get_id().to_string());
        vertex_str.push_str("; ");
        return vertex_str.to_string();
    }
}

static GRAPH_STRING_MAX_BUFSIZE: usize = 5242880;

pub trait ConfiguredToDot {
    fn to_dot(&self, leading_newline: bool, indent_level: u8, config: &Config) -> String;
}

impl ConfiguredToDot for Graph {
    fn to_dot(&self, leading_newline: bool, indent_level: u8, config: &Config) -> String {
        let mut graph_string = String::with_capacity(max(64 * self.len(), GRAPH_STRING_MAX_BUFSIZE));

        graph_string.push_str("graph{");

        // Node configuration
        let mut buffers = VertexConfigBuffers::new(true, indent_level + 2, 256);
        buffers.weak_push_str_by_kind(SWCCompartmentKind::Axon, &format!("{}\n", config.get_config(SWCCompartmentKind::Axon).to_dot(false, indent_level + 2)));
        buffers.weak_push_str_by_kind(SWCCompartmentKind::Soma, &format!("{}\n", config.get_config(SWCCompartmentKind::Soma).to_dot(false, indent_level + 2)));
        buffers.weak_push_str_by_kind(SWCCompartmentKind::Dendrite, &format!("{}\n", config.get_config(SWCCompartmentKind::Dendrite).to_dot(false, indent_level + 2)));
        buffers.weak_push_str_by_kind(SWCCompartmentKind::ApicalDendrite, &format!("{}\n", config.get_config(SWCCompartmentKind::ApicalDendrite).to_dot(false, indent_level + 2)));
        buffers.weak_push_str_by_kind(SWCCompartmentKind::Undefined, &format!("{}\n", config.get_config(SWCCompartmentKind::Undefined).to_dot(false, indent_level + 2)));
        buffers.weak_push_str_by_kind(SWCCompartmentKind::Custom, &format!("{}\n", config.get_config(SWCCompartmentKind::Custom).to_dot(false, indent_level + 2)));
        for (_, vertex) in self.iter_vertices() {
            buffers.push_str_by_kind(vertex.get_kind(), &vertex.to_dot(false, 0));
        }

        graph_string.push_str(&buffers.to_dot(true, indent_level + 1));

        // Write edges
        for short_tree in self.iter_short_trees() {
            graph_string.push_str(&short_tree.to_dot(true, indent_level + 1))
        }
        graph_string.push_str("\n}");

        graph_string.shrink_to_fit();
        return graph_string;
    }
}

/// Struct for holding Vertex config strings
struct VertexConfigBuffers {
    axon: StringBuffer,
    soma: StringBuffer,
    dendrite: StringBuffer,
    apicaldendrite: StringBuffer,
    undefined: StringBuffer,
    custom: StringBuffer
}

impl VertexConfigBuffers {
    fn new(leading_newline: bool, indent_level: u8, capacity: usize) -> VertexConfigBuffers {
        VertexConfigBuffers {
            axon: StringBuffer::new(leading_newline, indent_level, capacity),
            soma: StringBuffer::new(leading_newline, indent_level, capacity),
            dendrite: StringBuffer::new(leading_newline, indent_level, capacity),
            apicaldendrite: StringBuffer::new(leading_newline, indent_level, capacity),
            undefined: StringBuffer::new(leading_newline, indent_level, capacity),
            custom: StringBuffer::new(leading_newline, indent_level, capacity),
        }
    }

    fn push_str_by_kind(&mut self, kind: SWCCompartmentKind, string: &str) {
        match kind {
            SWCCompartmentKind::Axon => self.axon.push_str(string),
            SWCCompartmentKind::Soma => self.soma.push_str(string),
            SWCCompartmentKind::Dendrite => self.dendrite.push_str(string),
            SWCCompartmentKind::ApicalDendrite => self.apicaldendrite.push_str(string),
            SWCCompartmentKind::Undefined => self.undefined.push_str(string),
            SWCCompartmentKind::Custom => self.custom.push_str(string),
        }
    }

    fn weak_push_str_by_kind(&mut self, kind: SWCCompartmentKind, string: &str) {
        match kind {
            SWCCompartmentKind::Axon => self.axon.weak_push_str(string),
            SWCCompartmentKind::Soma => self.soma.weak_push_str(string),
            SWCCompartmentKind::Dendrite => self.dendrite.weak_push_str(string),
            SWCCompartmentKind::ApicalDendrite => self.apicaldendrite.weak_push_str(string),
            SWCCompartmentKind::Undefined => self.undefined.weak_push_str(string),
            SWCCompartmentKind::Custom => self.custom.weak_push_str(string),
        }
    }

    fn len(&self) -> usize {
        self.axon.len()
            + self.soma.len()
            + self.dendrite.len()
            + self.apicaldendrite.len()
            + self.undefined.len()
            + self.custom.len()
    }
}

impl ToDot for VertexConfigBuffers {
    fn to_dot(&self, leading_newline: bool, indent_level: u8) -> String {
        let mut vertex_config_buf = StringBuffer::new(leading_newline, 0, self.len() + 64);

        if self.axon.len() > 0 {
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&indent(indent_level));
            vertex_config_buf.push_str("{");
            vertex_config_buf.push_str(self.axon.as_ref());
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&indent(indent_level));
            vertex_config_buf.push_str("}");
        }

        if self.soma.len() > 0 {
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&indent(indent_level));
            vertex_config_buf.push_str("{");
            vertex_config_buf.push_str(self.soma.as_ref());
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&indent(indent_level));
            vertex_config_buf.push_str("}");
        }

        if self.dendrite.len() > 0 {
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&indent(indent_level));
            vertex_config_buf.push_str("{");
            vertex_config_buf.push_str(self.dendrite.as_ref());
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&indent(indent_level));
            vertex_config_buf.push_str("}");
        }

        if self.apicaldendrite.len() > 0 {
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&indent(indent_level));
            vertex_config_buf.push_str("{");
            vertex_config_buf.push_str(self.apicaldendrite.as_ref());
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&indent(indent_level));
            vertex_config_buf.push_str("}");
        }

        if self.undefined.len() > 0 {
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&indent(indent_level));
            vertex_config_buf.push_str("{");
            vertex_config_buf.push_str(self.undefined.as_ref());
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&indent(indent_level));
            vertex_config_buf.push_str("}");
        }

        if self.custom.len() > 0 {
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&indent(indent_level));
            vertex_config_buf.push_str("{");
            vertex_config_buf.push_str(self.custom.as_ref());
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&indent(indent_level));
            vertex_config_buf.push_str("}");
        }

        return vertex_config_buf.to_string();
    }
}

impl ToDot for ShortTree {
    /// Get DOT representation of a rooted tree of depth 1.
    ///
    /// Rooted trees of depth 1 can be written in one line in DOT.
    fn to_dot(&self, leading_newline: bool, indent_level: u8) -> String {
        let mut tree_buf = StringBuffer::new(leading_newline, indent_level, 128);

        tree_buf.push_str(&self.get_root_id().to_string());
        match self.get_child_ids().len() {
            0 => {},
            1 => tree_buf.push_str(&format!(" -- {}", self.get_child_ids()[0])),
            _ => tree_buf.push_str(&format!(" -- {{{}}}", self.get_child_ids().iter().format(", ")))
        }
        tree_buf.push_str(";");
        return tree_buf.to_string();
    }
}

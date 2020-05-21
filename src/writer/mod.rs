use std::cmp::max;

use itertools::Itertools;

use crate::components::{Graph, ShortTree, Vertex};
use crate::config::Config;
use crate::swc_parser::SWCCompartmentKind;

mod string_buffer;

pub use string_buffer::{StringBuffer, Indent, get_indent};

/// Get a `String` representation of an object in DOT format.
pub trait ToDot {
    fn to_dot(&self, leading_newline: bool, indent: Indent) -> String;
}

impl ToDot for Vertex {
    /// Get a DOT representation of a single vertex.
    fn to_dot(&self, leading_newline: bool, indent: Indent) -> String {
        let mut vertex_str = StringBuffer::new(leading_newline, indent, 32);
        vertex_str.push_str(&self.get_id().to_string());
        vertex_str.push_str("; ");
        return vertex_str.to_string();
    }
}

static GRAPH_STRING_MAX_BUFSIZE: usize = 5242880;

pub trait ConfiguredToDot {
    fn to_dot(&self, leading_newline: bool, indent: Indent, config: &Config) -> String;
}

impl ConfiguredToDot for Graph {
    fn to_dot(&self, _leading_newline: bool, indent: Indent, config: &Config) -> String {
        let mut graph_string =
            String::with_capacity(max(64 * self.len(), GRAPH_STRING_MAX_BUFSIZE));

        graph_string.push_str("graph{");

        // Node configuration
        let mut buffers = VertexConfigBuffers::new(true, Indent::flat(indent.main + 2), 256);

        for kind in SWCCompartmentKind::iter() {
            buffers.weak_push_str_by_kind(kind, &config.get_config(kind).to_dot(false, Indent::absolute_first_line(0, indent.main + 2)));
            buffers.weak_push_str_by_kind(kind, " ")
        }
        for (_, vertex) in self.iter_vertices() {
            buffers.push_str_by_kind(vertex.get_kind(), &vertex.to_dot(false, Indent::zero()));
        }

        graph_string.push_str(&buffers.to_dot(true, Indent::flat(indent.main + 1)));

        // Write edges
        for short_tree in self.iter_short_trees() {
            graph_string.push_str(&short_tree.to_dot(true, Indent::flat(indent.main + 1)));
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
    custom: StringBuffer,
}

impl VertexConfigBuffers {
    fn new(leading_newline: bool, indent: Indent, capacity: usize) -> VertexConfigBuffers {
        VertexConfigBuffers {
            axon: StringBuffer::new(leading_newline, indent, capacity),
            soma: StringBuffer::new(leading_newline, indent, capacity),
            dendrite: StringBuffer::new(leading_newline, indent, capacity),
            apicaldendrite: StringBuffer::new(leading_newline, indent, capacity),
            undefined: StringBuffer::new(leading_newline, indent, capacity),
            custom: StringBuffer::new(leading_newline, indent, capacity),
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
    fn to_dot(&self, leading_newline: bool, indent: Indent) -> String {
        let mut vertex_config_buf = StringBuffer::new(leading_newline, indent, self.len() + 64);

        if self.axon.len() > 0 {
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&get_indent(indent.main));
            vertex_config_buf.push_str("{");
            vertex_config_buf.push_str(self.axon.as_ref());
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&get_indent(indent.main));
            vertex_config_buf.push_str("}");
        }

        if self.soma.len() > 0 {
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&get_indent(indent.main));
            vertex_config_buf.push_str("{");
            vertex_config_buf.push_str(self.soma.as_ref());
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&get_indent(indent.main));
            vertex_config_buf.push_str("}");
        }

        if self.dendrite.len() > 0 {
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&get_indent(indent.main));
            vertex_config_buf.push_str("{");
            vertex_config_buf.push_str(self.dendrite.as_ref());
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&get_indent(indent.main));
            vertex_config_buf.push_str("}");
        }

        if self.apicaldendrite.len() > 0 {
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&get_indent(indent.main));
            vertex_config_buf.push_str("{");
            vertex_config_buf.push_str(self.apicaldendrite.as_ref());
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&get_indent(indent.main));
            vertex_config_buf.push_str("}");
        }

        if self.undefined.len() > 0 {
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&get_indent(indent.main));
            vertex_config_buf.push_str("{");
            vertex_config_buf.push_str(self.undefined.as_ref());
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&get_indent(indent.main));
            vertex_config_buf.push_str("}");
        }

        if self.custom.len() > 0 {
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&get_indent(indent.main));
            vertex_config_buf.push_str("{");
            vertex_config_buf.push_str(self.custom.as_ref());
            vertex_config_buf.push_str("\n");
            vertex_config_buf.push_str(&get_indent(indent.main));
            vertex_config_buf.push_str("}");
        }

        return vertex_config_buf.to_string();
    }
}

impl ToDot for ShortTree {
    /// Get DOT representation of a rooted tree of depth 1.
    ///
    /// Rooted trees of depth 1 can be written in one line in DOT.
    fn to_dot(&self, leading_newline: bool, indent: Indent) -> String {
        let mut tree_buf = StringBuffer::new(leading_newline, indent, 128);

        tree_buf.push_str(&self.get_root_id().to_string());
        match self.get_child_ids().len() {
            0 => {}
            1 => tree_buf.push_str(&format!(" -- {}", self.get_child_ids()[0])),
            _ => tree_buf.push_str(&format!(
                " -- {{{}}}",
                self.get_child_ids().iter().format(", ")
            )),
        }
        tree_buf.push_str(";");
        return tree_buf.to_string();
    }
}

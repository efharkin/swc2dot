use std::cmp::max;
use std::collections::HashMap;

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

/// Get a configured `String` representation of an object in DOT format.
///
/// # See also
///
/// - `ToDot` trait
pub trait ConfiguredToDot {
    fn to_dot(&self, leading_newline: bool, indent: Indent, config: &Config) -> String;
}

impl ConfiguredToDot for Graph {
    fn to_dot(&self, _leading_newline: bool, indent: Indent, config: &Config) -> String {
        let mut graph_string =
            String::with_capacity(max(64 * self.len(), GRAPH_STRING_MAX_BUFSIZE));

        graph_string.push_str("graph{");

        // Node configuration
        use vertex_config_formatter::VertexConfigFormatter;
        let mut buffers = VertexConfigFormatter::new(true, Indent::flat(indent.main + 2), 256);

        for kind in SWCCompartmentKind::iter() {
            buffers.weak_push_config_str(kind, &config.get_config(kind).to_dot(false, Indent::zero()));
            buffers.weak_push_config_str(kind, " ");
        }
        for (_, vertex) in self.iter_vertices() {
            buffers.push_config_str(vertex.get_kind(), &vertex.to_dot(false, Indent::zero()));
        }

        graph_string.push_str(&buffers.to_dot(false, Indent::flat(indent.main + 1)));

        // Write edges
        for short_tree in self.iter_short_trees() {
            graph_string.push_str(&short_tree.to_dot(true, Indent::flat(indent.main + 1)));
        }
        graph_string.push_str("\n}");

        graph_string.shrink_to_fit();
        return graph_string;
    }
}

///
mod vertex_config_formatter {
    use super::*;

    /// Pretty formatting of `Vertex` attributes in DOT language.
    pub struct VertexConfigFormatter {
        vertex_config_strings: HashMap<SWCCompartmentKind, StringBuffer>,
    }

    impl VertexConfigFormatter {
        pub fn new(
            leading_newline: bool,
            indent: Indent,
            capacity: usize,
        ) -> VertexConfigFormatter {
            let mut vertex_config_strings = HashMap::with_capacity(6);
            for key in SWCCompartmentKind::iter() {
                vertex_config_strings
                    .insert(key, StringBuffer::new(leading_newline, indent, capacity));
            }
            VertexConfigFormatter {
                vertex_config_strings: vertex_config_strings,
            }
        }

        pub fn push_config_str(&mut self, vertex_kind: SWCCompartmentKind, string: &str) {
            let config_buffer: &mut StringBuffer = self
                .vertex_config_strings
                .get_mut(&vertex_kind)
                .expect(&format!(
                    "{:?} not found in VertexConfigBuffers instance",
                    vertex_kind
                ));
            config_buffer.push_str(string);
        }

        /// Add an optional component to the config string.
        ///
        /// Config strings that only contain optional components are ignored by
        /// `to_dot()`.
        ///
        /// # Recommended use case
        ///
        /// Use this method to add configuration details for compartment types that may or may
        /// not exist in the current graph, and use `push_config_str()` to add the names of all
        /// compartments of the given type (if any exist). If there are no compartments of the
        /// given type, `push_config_str()` will never be called, and the configuration details
        /// added using `weak_push_config_str()` will be left out of the output of `to_dot()`.
        pub fn weak_push_config_str(&mut self, vertex_kind: SWCCompartmentKind, string: &str) {
            let config_buffer: &mut StringBuffer = self
                .vertex_config_strings
                .get_mut(&vertex_kind)
                .expect(&format!(
                    "{:?} not found in VertexConfigBuffers instance",
                    vertex_kind
                ));
            config_buffer.weak_push_str(string);
        }

        pub fn len(&self) -> usize {
            let mut total_length: usize = 0;
            for val in self.vertex_config_strings.values() {
                total_length += val.len();
            }
            return total_length;
        }
    }

    impl ToDot for VertexConfigFormatter {
        /// Get node configuration in DOT language
        fn to_dot(&self, leading_newline: bool, indent: Indent) -> String {
            let mut full_config_string =
                StringBuffer::new(leading_newline, indent, self.len() + 64);

            for config_string in self.vertex_config_strings.values() {
                if config_string.len() > 0 {
                    // Opening brace on a new line
                    full_config_string.newline();
                    full_config_string.push_str("{");

                    // Configuration for the current compartment type
                    full_config_string.push_str(config_string.as_ref());

                    // Closing brace on a new line
                    full_config_string.newline();
                    full_config_string.push_str("}");
                }
            }

            return full_config_string.to_string();
        }
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

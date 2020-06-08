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

#[cfg(test)]
mod vertex_todot_tests {
    use super::*;

    #[test]
    fn formatted_id_appears_in_output() {
        let vertex = get_test_vertex();
        assert!(vertex
            .to_dot(false, Indent::zero())
            .contains(&format!("{}; ", vertex.get_id())));
    }

    #[test]
    fn leading_newline_zero_indent() {
        let vertex = get_test_vertex();
        assert_eq!(
            vertex.to_dot(true, Indent::zero()).chars().next().unwrap(),
            '\n',
            "Expected first char to be newline when argument `newline=true`"
        )
    }

    #[test]
    fn no_leading_newline_zero_indent() {
        let vertex = get_test_vertex();
        assert!(
            vertex.to_dot(false, Indent::zero()).chars().next().unwrap() != '\n',
            "Expected first char to not be newline when arguemnt `newline=false`"
        )
    }

    fn get_test_vertex() -> Vertex {
        use crate::swc_parser::{Point, SWCCompartment};
        let vertex = Vertex::from(SWCCompartment::new(
            64,
            SWCCompartmentKind::Dendrite,
            Point {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            },
            1.0,
            None,
        ));
        return vertex;
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

            for compartment_kind in SWCCompartmentKind::iter() {
                // Allocate buffer for vertex configuration settings for this compartment type.
                let mut compartment_config_string =
                    StringBuffer::new(leading_newline, indent, capacity);

                // Add a descriptive header.
                compartment_config_string.weak_push_str(&format!(
                    "/* Configuration for {} vertices. */",
                    compartment_kind
                ));
                compartment_config_string.newline();

                // Insert it into HashMap that will be stored in the VertexConfigFormatter.
                vertex_config_strings.insert(compartment_kind, compartment_config_string);
            }

            // Construct the new VertexConfigFormatter
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

    #[cfg(test)]
    mod vertexconfigformatter_todot_tests {
        use super::*;

        #[test]
        fn weak_push_yields_empty_string() {
            let mut formatter = VertexConfigFormatter::new(true, Indent::flat(1), 1024);

            // Push content that does not need to be printed.
            for kind in SWCCompartmentKind::iter() {
                formatter.weak_push_config_str(kind, "unnecessary content");
            }

            assert_eq!(formatter.to_dot(true, Indent::flat(1)), "");
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

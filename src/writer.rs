use std::cmp::max;

use itertools::Itertools;

use crate::components::{Graph, Vertex, ShortTree};
use crate::config::Config;
use crate::swc_parser::SWCCompartmentKind;

pub trait ToDot {
    fn to_dot(&self, leading_newline: bool, indent_level: u8) -> String;
}

static INDENT_SIZE: u8 = 4;

/// Get a new `String` buffer, optionally on a new line and optinally indented.
pub fn new_string_buffer(leading_newline: bool, indent_level: u8, capacity: usize) -> String {
    let mut buf = String::with_capacity((1 + INDENT_SIZE*indent_level) as usize + capacity);
    if leading_newline {
        buf.push_str("\n");
    }
    buf.push_str(&indent(indent_level));
    return buf;
}

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
        let mut vertex_str = new_string_buffer(leading_newline, indent_level, 32);
        vertex_str.push_str(&self.get_id().to_string());
        vertex_str.push_str("; ");
        vertex_str.shrink_to_fit();
        return vertex_str;
    }
}

static graph_string_max_bufsize: usize = 5242880;

pub trait ConfiguredToDot {
    fn to_dot(&self, leading_newline: bool, indent_level: u8, config: &Config) -> String;
}

impl ConfiguredToDot for Graph {
    fn to_dot(&self, leading_newline: bool, indent_level: u8, config: &Config) -> String {
        let mut graph_string = String::with_capacity(max(64 * self.len(), graph_string_max_bufsize));

        graph_string.push_str("graph{");

        // Node configuration
        let mut buffers = VertexConfigBuffers::new(true, indent_level + 1, 256);
        buffers.weak_push_str_by_kind(SWCCompartmentKind::Axon, &format!("{}\n", config.get_config(SWCCompartmentKind::Axon).to_dot(false, 0)));
        buffers.weak_push_str_by_kind(SWCCompartmentKind::Soma, &format!("{}\n", config.get_config(SWCCompartmentKind::Soma).to_dot(false, 0)));
        buffers.weak_push_str_by_kind(SWCCompartmentKind::Dendrite, &format!("{}\n", config.get_config(SWCCompartmentKind::Dendrite).to_dot(false, 0)));
        buffers.weak_push_str_by_kind(SWCCompartmentKind::ApicalDendrite, &format!("{}\n", config.get_config(SWCCompartmentKind::ApicalDendrite).to_dot(false, 0)));
        buffers.weak_push_str_by_kind(SWCCompartmentKind::Undefined, &format!("{}\n", config.get_config(SWCCompartmentKind::Undefined).to_dot(false, 0)));
        buffers.weak_push_str_by_kind(SWCCompartmentKind::Custom, &format!("{}\n", config.get_config(SWCCompartmentKind::Custom).to_dot(false, 0)));
        for (_, vertex) in self.iter_vertices() {
            buffers.push_str_by_kind(vertex.get_kind(), &vertex.to_dot(false, 0));
        }

        graph_string.push_str(&buffers.to_dot(true, indent_level + 2));

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

/// A `String` buffer that will appear to be empty if it has never been written to.
#[derive(Clone, Debug)]
struct StringBuffer {
    buf: String,
    empty_buf: String,
    has_been_written_to: bool
}

impl StringBuffer {
    fn new(leading_newline: bool, indent_level: u8, capacity: usize) -> StringBuffer {
        let mut buf = String::with_capacity((1 + INDENT_SIZE*indent_level) as usize + capacity);
        if leading_newline {
            buf.push_str("\n");
        }
        buf.push_str(&indent(indent_level));

        StringBuffer {
            buf: buf,
            empty_buf: "".to_string(),
            has_been_written_to: false
        }
    }

    /// Push `&str` onto the end of `StringBuffer`.
    fn push_str(&mut self, string: &str) {
        self.has_been_written_to = true;
        self.buf.push_str(string);
    }

    /// Push `&str` onto the end of `StringBuffer`, but don't flag the buffer as modified.
    fn weak_push_str(&mut self, string: &str) {
        self.buf.push_str(string);
    }

    /// Get contents of `StringBuffer`.
    ///
    /// Returns an empty `String` if `push_str()` has never been called.
    fn to_string(&self) -> String {
        if self.has_been_written_to {
            self.buf.clone()
        } else {
            self.empty_buf.clone()
        }
    }

    fn len(&self) -> usize {
        if self.has_been_written_to {
            self.buf.len()
        } else {
            self.empty_buf.len()
        }
    }
}

impl AsRef<String> for StringBuffer {
    fn as_ref(&self) -> &String {
        if self.has_been_written_to {
            &self.buf
        } else {
            &self.empty_buf
        }
    }
}

impl AsRef<str> for StringBuffer {
    fn as_ref(&self) -> &str {
        if self.has_been_written_to {
            &self.buf
        } else {
            &self.empty_buf
        }
    }
}

#[cfg(test)]
mod string_buffer_tests {
    use super::*;

    #[test]
    fn returns_empty_if_push_str_is_never_called() {
        // Initialize a `StringBuffer` with a newline and big indent.
        let mut string = StringBuffer::new(true, 5, 32);
        assert_eq!("".to_string(), string.to_string())
    }

    #[test]
    fn returns_indented_if_push_str_is_called() {
        let mut string = StringBuffer::new(false, 1, 32);
        string.push_str("something");
        assert_eq!("    something".to_string(), string.to_string());
    }

    #[test]
    fn returns_newline_if_push_str_is_called() {
        let mut string = StringBuffer::new(true, 0, 32);
        string.push_str("something");
        assert_eq!("\nsomething".to_string(), string.to_string());
    }

    fn compare_string_ref(a: &String, b: &String) -> bool {
        a == b
    }

    fn compare_str_ref(a: &str, b: &str) -> bool {
        a == b
    }

    #[test]
    fn asref_str_empty_if_push_str_is_never_called() {
        // Initialize a `StringBuffer` with a newline and big indent.
        let mut string = StringBuffer::new(true, 5, 32);
        if !compare_str_ref(string.as_ref(), "") {
            panic!("Failed");
        }
    }

    #[test]
    fn asref_str_indented_if_push_str_is_called() {
        let mut string = StringBuffer::new(false, 1, 32);
        string.push_str("something");
        if !compare_str_ref(string.as_ref(), "    something") {
            panic!("Failed");
        }
    }

    #[test]
    fn asref_str_newline_if_push_str_is_called() {
        let mut string = StringBuffer::new(true, 0, 32);
        string.push_str("something");
        if !compare_str_ref(string.as_ref(), "\nsomething") {
            panic!("Failed");
        }
    }
}

impl ToDot for ShortTree {
    /// Get DOT representation of a rooted tree of depth 1.
    ///
    /// Rooted trees of depth 1 can be written in one line in DOT.
    fn to_dot(&self, leading_newline: bool, indent_level: u8) -> String {
        let mut tree_buf = new_string_buffer(leading_newline, indent_level, 128);

        tree_buf.push_str(&self.get_root_id().to_string());
        match self.get_child_ids().len() {
            0 => {},
            1 => tree_buf.push_str(&format!(" -- {}", self.get_child_ids()[0])),
            _ => tree_buf.push_str(&format!(" -- {{{}}}", self.get_child_ids().iter().format(", ")))
        }
        tree_buf.push_str(";");
        return tree_buf;
    }
}

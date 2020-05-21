use super::{indent, INDENT_SIZE};

/// A `String` buffer that will appear to be empty if it has never been modified.
#[derive(Clone, Debug)]
pub struct StringBuffer {
    buf: String,
    empty_buf: String,
    has_been_written_to: bool,
    indent_level: u8,
    line_width: u32,
    cursor_position: u32,
}

impl StringBuffer {
    /// Create a new `StringBuffer`.
    pub fn new(leading_newline: bool, indent_level: u8, capacity: usize) -> StringBuffer {
        let mut buf = String::with_capacity((1 + INDENT_SIZE * indent_level) as usize + capacity);

        if leading_newline {
            buf.push_str("\n");
        }
        buf.push_str(&indent(indent_level));

        let mut string_buffer = StringBuffer {
            buf: buf,
            empty_buf: "".to_string(),
            has_been_written_to: false,
            indent_level: indent_level,
            line_width: 80,
            cursor_position: 0,
        };
        string_buffer.cursor_position = string_buffer.newline_cursor_position();
        string_buffer.assert_cursor_is_within_line();

        return string_buffer;
    }

    /// Push `string` onto the end of `StringBuffer`.
    ///
    /// Note: this will mark the `StringBuffer` as modified.
    pub fn push_str(&mut self, string: &str) {
        self.has_been_written_to = true;
        self.weak_push_str(string);
    }

    /// Insert a newline into the `StringBuffer`.
    ///
    /// Does not mark the buffer as modified.
    pub fn newline(&mut self) {
        self.buf.push_str("\n");
        self.buf.push_str(&indent(self.indent_level));
        self.cursor_position = self.newline_cursor_position();
        self.assert_cursor_is_within_line();
    }

    /// Push `&str` onto the end of `StringBuffer`, but don't flag the buffer as modified.
    pub fn weak_push_str(&mut self, string: &str) {
        self.assert_cursor_is_within_line();

        // Start on a new line if we will run out of room on the current one,
        // unless we're already at the start of a line.
        if (string.len() > self.remaining_space_on_line() as usize)
            & (self.cursor_position > self.newline_cursor_position())
        {
            self.newline();
        }

        // Add the string to the buffer
        self.buf.push_str(string);

        // Update cursor position
        if string.len() as u32 + self.cursor_position <= self.line_width {
            self.cursor_position += string.len() as u32;
        } else {
            // If the cursor went off the end of the line, go to a new line.
            self.newline();
            assert_eq!(self.cursor_position, self.newline_cursor_position());
        }

        self.assert_cursor_is_within_line();
    }

    /// Get contents of `StringBuffer`.
    ///
    /// Returns an empty `String` if the buffer has not been marked as modified.
    pub fn to_string(&self) -> String {
        if self.has_been_written_to {
            let mut result = self.buf.clone();
            result.shrink_to_fit();
            result
        } else {
            self.empty_buf.clone()
        }
    }

    /// Get the length of the `StringBuffer`.
    pub fn len(&self) -> usize {
        if self.has_been_written_to {
            self.buf.len()
        } else {
            self.empty_buf.len()
        }
    }

    /// Get the position of the cursor at the beginning of a blank line.
    #[inline]
    fn newline_cursor_position(&self) -> u32 {
        (self.indent_level * INDENT_SIZE) as u32
    }

    /// Get the remaining amount of space on the current line.
    #[inline]
    fn remaining_space_on_line(&self) -> u32 {
        self.assert_cursor_is_within_line();
        self.line_width - self.cursor_position
    }

    /// Assert that cursor is within the length of one line.
    ///
    /// For internal use only.
    ///
    /// # Panics
    ///
    /// Panics if the cursor is off the end of a line.
    #[inline]
    fn assert_cursor_is_within_line(&self) {
        assert!(
            self.cursor_position <= self.line_width,
            "Cursor position {} greater than line width {}.",
            self.cursor_position,
            self.line_width
        );
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
        let string = StringBuffer::new(true, 5, 32);
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

    fn compare_str_ref(a: &str, b: &str) -> bool {
        a == b
    }

    #[test]
    fn asref_str_empty_if_push_str_is_never_called() {
        // Initialize a `StringBuffer` with a newline and big indent.
        let string = StringBuffer::new(true, 5, 32);
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

    // LINE WRAPPING TESTS

    #[test]
    fn hard_wrap_short_line_without_indent() {
        let mut string = StringBuffer::new(false, 0, 32);
        string.line_width = 4;
        string.push_str("123");
        string.push_str("456");
        assert_eq!("123\n456".to_string(), string.to_string());
    }

    #[test]
    fn hard_wrap_full_line_without_indent() {
        let mut string = StringBuffer::new(false, 0, 32);
        string.line_width = 4;
        string.push_str("1234");
        string.push_str("5678");
        assert_eq!("1234\n5678".to_string(), string.to_string());
    }

    #[test]
    fn hard_wrap_short_line_with_indent() {
        let mut string = StringBuffer::new(false, 1, 32);
        string.line_width = 8;
        string.push_str("123");
        string.push_str("456");
        assert_eq!("    123\n    456".to_string(), string.to_string());
    }

    #[test]
    fn hard_wrap_full_line_with_indent() {
        let mut string = StringBuffer::new(false, 1, 32);
        string.line_width = 8;
        string.push_str("1234");
        string.push_str("5678");
        assert_eq!("    1234\n    5678".to_string(), string.to_string());
    }

    #[test]
    fn hard_wrap_long_line_without_indent() {
        let mut string = StringBuffer::new(false, 0, 32);
        string.line_width = 4;
        string.push_str("12345");
        string.push_str("678");
        assert_eq!("12345\n678".to_string(), string.to_string());
    }

    #[test]
    fn hard_wrap_long_second_line_with_indent() {
        let mut string = StringBuffer::new(false, 1, 32);
        string.line_width = 8;
        string.push_str("123");
        string.push_str("456789");
        string.push_str("0");
        assert_eq!("    123\n    456789\n    0".to_string(), string.to_string());
    }
}

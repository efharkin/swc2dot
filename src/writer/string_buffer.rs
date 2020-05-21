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
    pub fn new(leading_newline: bool, indent: Indent, capacity: usize) -> StringBuffer {
        let mut buf = String::with_capacity((32 + INDENT_SIZE * indent.first) as usize + capacity);

        if leading_newline {
            buf.push_str("\n");
        }
        buf.push_str(&get_indent(indent.first));

        let string_buffer = StringBuffer {
            buf: buf,
            empty_buf: "".to_string(),
            has_been_written_to: false,
            indent_level: indent.main,
            line_width: 80,
            cursor_position: (INDENT_SIZE * indent.first) as u32,
        };
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
        self.buf.push_str(&get_indent(self.indent_level));
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
        let string = StringBuffer::new(true, Indent::flat(5), 32);
        assert_eq!("".to_string(), string.to_string())
    }

    #[test]
    fn returns_indented_if_push_str_is_called() {
        let mut string = StringBuffer::new(false, Indent::flat(1), 32);
        string.push_str("something");
        assert_eq!("    something".to_string(), string.to_string());
    }

    #[test]
    fn returns_newline_if_push_str_is_called() {
        let mut string = StringBuffer::new(true, Indent::flat(0), 32);
        string.push_str("something");
        assert_eq!("\nsomething".to_string(), string.to_string());
    }

    fn compare_str_ref(a: &str, b: &str) -> bool {
        a == b
    }

    #[test]
    fn asref_str_empty_if_push_str_is_never_called() {
        // Initialize a `StringBuffer` with a newline and big indent.
        let string = StringBuffer::new(true, Indent::flat(5), 32);
        if !compare_str_ref(string.as_ref(), "") {
            panic!("Failed");
        }
    }

    #[test]
    fn asref_str_indented_if_push_str_is_called() {
        let mut string = StringBuffer::new(false, Indent::flat(1), 32);
        string.push_str("something");
        if !compare_str_ref(string.as_ref(), "    something") {
            panic!("Failed");
        }
    }

    #[test]
    fn asref_str_newline_if_push_str_is_called() {
        let mut string = StringBuffer::new(true, Indent::flat(0), 32);
        string.push_str("something");
        if !compare_str_ref(string.as_ref(), "\nsomething") {
            panic!("Failed");
        }
    }

    // LINE WRAPPING TESTS

    #[test]
    fn hard_wrap_short_line_without_indent() {
        let mut string = StringBuffer::new(false, Indent::flat(0), 32);
        string.line_width = 4;
        string.push_str("123");
        string.push_str("456");
        assert_eq!("123\n456".to_string(), string.to_string());
    }

    #[test]
    fn hard_wrap_full_line_without_indent() {
        let mut string = StringBuffer::new(false, Indent::flat(0), 32);
        string.line_width = 4;
        string.push_str("1234");
        string.push_str("5678");
        assert_eq!("1234\n5678".to_string(), string.to_string());
    }

    #[test]
    fn hard_wrap_short_line_with_indent() {
        let mut string = StringBuffer::new(false, Indent::flat(1), 32);
        string.line_width = 8;
        string.push_str("123");
        string.push_str("456");
        assert_eq!("    123\n    456".to_string(), string.to_string());
    }

    #[test]
    fn hard_wrap_full_line_with_indent() {
        let mut string = StringBuffer::new(false, Indent::flat(1), 32);
        string.line_width = 8;
        string.push_str("1234");
        string.push_str("5678");
        assert_eq!("    1234\n    5678".to_string(), string.to_string());
    }

    #[test]
    fn hard_wrap_long_line_without_indent() {
        let mut string = StringBuffer::new(false, Indent::flat(0), 32);
        string.line_width = 4;
        string.push_str("12345");
        string.push_str("678");
        assert_eq!("12345\n678".to_string(), string.to_string());
    }

    #[test]
    fn hard_wrap_long_second_line_with_indent() {
        let mut string = StringBuffer::new(false, Indent::flat(1), 32);
        string.line_width = 8;
        string.push_str("123");
        string.push_str("456789");
        string.push_str("0");
        assert_eq!("    123\n    456789\n    0".to_string(), string.to_string());
    }
}

/*
pub enum Indent {
    /// Set the indent of the first line to a different level than subsequent lines.
    ///
    /// `AbsoluteFirstLine(first_line_level, subsequent_lines)`
    AbsoluteFirstLine(u8, u8),
    /// Set the indent level of the first line relative to subsequent lines.
    ///
    /// `RelativeFirstLine(relative_first_line_level, subsequent_lines)`
    ///
    /// # Note
    ///
    /// `RelativeFirstLine(-1, 1)` is equivalent to `AbsoluteFirstLine(0, 1)`.
    RelativeFirstLine(i8, u8),
    /// All lines are indented to the same level.
    Flat(u8)
}
*/

/// Indentation style
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Indent {
    /// Indentation of the first line
    pub first: u8,
    /// Indentation of all subsequent lines
    pub main: u8
}

impl Indent {
    /// Indent first line to a different level.
    ///
    /// # Equivalence
    ///
    /// ```rust
    /// let absolute_indent = Indent::absolute_first_line(2, 3);
    /// let relative_indent = Indent::relative_first_line(-1, 3);
    ///
    /// assert_eq!(absolute_indent, relative_indent);
    /// ```
    pub fn absolute_first_line(first_line_level: u8, main_indent_level: u8) -> Indent {
        Indent{
            first: first_line_level,
            main: main_indent_level
        }
    }

    /// Offset indent level of first line.
    ///
    /// # Equivalence
    ///
    /// ```rust
    /// let relative_indent = Indent::relative_first_line(-1, 3);
    /// let absolute_indent = Indent::absolute_first_line(2, 3);
    ///
    /// assert_eq!(absolute_indent, relative_indent);
    /// ```
    pub fn relative_first_line(first_line_level: i8, main_indent_level: u8) -> Indent {
        Indent {
            first: (main_indent_level as i16 - first_line_level as i16) as u8,
            main: main_indent_level
        }
    }

    /// Indent all lines to the same level.
    ///
    /// # Equivalence
    ///
    /// ```rust
    /// let flat_indent = Indent::flat(3);
    /// let absolute_indent = Indent::absolute_first_line(3, 3);
    /// let relative_indent = Indent::relative_first_line(0, 3);
    ///
    /// assert_eq!(flat_indent, absolute_indent);
    /// assert_eq!(flat_indent, relative_indent);
    /// ```
    pub fn flat(indent_level: u8) -> Indent {
        Indent {
            first: indent_level,
            main: indent_level
        }
    }

    /// Convenience function to get a flat zero indent.
    pub fn zero() -> Indent {
        Indent {
            first: 0,
            main: 0
        }
    }
}

static INDENT_SIZE: u8 = 4;

/// Get a `String` of spaces for indenting.
pub fn get_indent(level: u8) -> String {
    let mut buf = String::with_capacity((level * INDENT_SIZE) as usize);
    for _ in 0..level {
        buf.push_str("    ");
    }
    return buf;
}


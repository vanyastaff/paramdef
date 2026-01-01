//! Code decoration for syntax-highlighted snippets.
//!
//! Code displays syntax-highlighted code examples.

use std::any::Any;

use crate::core::{Flags, Key, Metadata};
use crate::types::kind::NodeKind;
use crate::types::traits::{Decoration, Node, };

/// A syntax-highlighted code decoration.
///
/// Code displays code snippets with optional syntax highlighting.
/// It has no value and cannot contain children.
///
/// # Example
///
/// ```ignore
/// use paramdef::decoration::Code;
///
/// // JSON example
/// let json = Code::builder("example_json")
///     .language("json")
///     .code(r#"{"name": "test", "value": 42}"#)
///     .build();
///
/// // Bash command with line numbers
/// let curl = Code::builder("curl_example")
///     .language("bash")
///     .code("curl -X GET https://api.example.com/v1/data")
///     .show_line_numbers(true)
///     .build();
///
/// // Collapsible code block
/// let long_code = Code::builder("full_example")
///     .language("rust")
///     .code("fn main() {\n    println!(\"Hello\");\n}")
///     .collapsible(true)
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Code {
    metadata: Metadata,
    flags: Flags,
    content: String,
    language: String,
    show_line_numbers: bool,
    highlight_lines: Vec<usize>,
    collapsible: bool,
}

impl Code {
    /// Creates a new builder for a Code block.
    #[must_use]
    pub fn builder(key: impl Into<Key>) -> CodeBuilder {
        CodeBuilder::new(key)
    }

    /// Creates a JSON code block.
    #[must_use]
    pub fn json(key: impl Into<Key>, code: impl Into<String>) -> Self {
        Self::builder(key).language("json").code(code).build()
    }

    /// Creates a Bash code block.
    #[must_use]
    pub fn bash(key: impl Into<Key>, code: impl Into<String>) -> Self {
        Self::builder(key).language("bash").code(code).build()
    }

    /// Creates a Rust code block.
    #[must_use]
    pub fn rust(key: impl Into<Key>, code: impl Into<String>) -> Self {
        Self::builder(key).language("rust").code(code).build()
    }

    /// Returns the flags for this code block.
    #[must_use]
    pub fn flags(&self) -> Flags {
        self.flags
    }

    /// Returns the code content.
    #[must_use]
    pub fn code(&self) -> &str {
        &self.content
    }

    /// Returns the language identifier.
    #[must_use]
    pub fn language(&self) -> &str {
        &self.language
    }

    /// Returns whether line numbers are shown.
    #[must_use]
    pub fn show_line_numbers(&self) -> bool {
        self.show_line_numbers
    }

    /// Returns the highlighted line numbers.
    #[must_use]
    pub fn highlight_lines(&self) -> &[usize] {
        &self.highlight_lines
    }

    /// Returns whether the code block is collapsible.
    #[must_use]
    pub fn is_collapsible(&self) -> bool {
        self.collapsible
    }
}

impl Node for Code {
    fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    fn key(&self) -> &Key {
        self.metadata.key()
    }

    fn kind(&self) -> NodeKind {
        NodeKind::Decoration
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Decoration for Code {}

// =============================================================================
// Builder
// =============================================================================

/// Builder for [`Code`].
#[derive(Debug)]
pub struct CodeBuilder {
    key: Key,
    flags: Flags,
    content: String,
    language: String,
    show_line_numbers: bool,
    highlight_lines: Vec<usize>,
    collapsible: bool,
}

impl CodeBuilder {
    /// Creates a new builder with the given key.
    #[must_use]
    pub fn new(key: impl Into<Key>) -> Self {
        Self {
            key: key.into(),
            flags: Flags::empty(),
            content: String::new(),
            language: String::new(),
            show_line_numbers: false,
            highlight_lines: Vec::new(),
            collapsible: false,
        }
    }

    /// Sets the flags.
    #[must_use]
    pub fn flags(mut self, flags: Flags) -> Self {
        self.flags = flags;
        self
    }

    /// Sets the code content.
    #[must_use]
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.content = code.into();
        self
    }

    /// Sets the language identifier (e.g., "rust", "json", "bash").
    #[must_use]
    pub fn language(mut self, language: impl Into<String>) -> Self {
        self.language = language.into();
        self
    }

    /// Sets whether to show line numbers.
    #[must_use]
    pub fn show_line_numbers(mut self, show: bool) -> Self {
        self.show_line_numbers = show;
        self
    }

    /// Sets the lines to highlight.
    #[must_use]
    pub fn highlight_lines(mut self, lines: Vec<usize>) -> Self {
        self.highlight_lines = lines;
        self
    }

    /// Sets whether the code block is collapsible.
    #[must_use]
    pub fn collapsible(mut self, collapsible: bool) -> Self {
        self.collapsible = collapsible;
        self
    }

    /// Builds the Code block.
    #[must_use]
    pub fn build(self) -> Code {
        Code {
            metadata: Metadata::new(self.key),
            flags: self.flags,
            content: self.content,
            language: self.language,
            show_line_numbers: self.show_line_numbers,
            highlight_lines: self.highlight_lines,
            collapsible: self.collapsible,
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_json() {
        let code = Code::json("example", r#"{"key": "value"}"#);

        assert_eq!(code.key().as_str(), "example");
        assert_eq!(code.language(), "json");
        assert_eq!(code.code(), r#"{"key": "value"}"#);
        assert!(!code.show_line_numbers());
    }

    #[test]
    fn test_code_bash() {
        let code = Code::bash("cmd", "echo hello");

        assert_eq!(code.language(), "bash");
        assert_eq!(code.code(), "echo hello");
    }

    #[test]
    fn test_code_rust() {
        let code = Code::rust("example", "fn main() {}");

        assert_eq!(code.language(), "rust");
    }

    #[test]
    fn test_code_builder() {
        let code = Code::builder("full")
            .language("python")
            .code("print('hello')")
            .show_line_numbers(true)
            .highlight_lines(vec![1, 3, 5])
            .collapsible(true)
            .build();

        assert_eq!(code.language(), "python");
        assert!(code.show_line_numbers());
        assert_eq!(code.highlight_lines(), &[1, 3, 5]);
        assert!(code.is_collapsible());
    }

    #[test]
    fn test_code_kind() {
        let code = Code::json("test", "{}");

        assert_eq!(code.kind(), NodeKind::Decoration);
    }

    #[test]
    fn test_code_invariants() {
        let code = Code::json("test", "{}");

        assert!(!code.kind().has_own_value());
        assert!(!code.kind().has_value_access());
        assert!(!code.kind().can_have_children());
    }
}

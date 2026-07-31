#[derive(Clone, Debug)]
pub struct SourceFile {
    pub name: String,
    pub content: String,
}

impl SourceFile {
    /// Creates a new source file instance from a given file path and source code string.
    pub fn new(name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            content: content.into(),
        }
    }

    /// Helper to fetch specific lines for precise error reporting and diagnostics.
    pub fn get_line(&self, line_index: usize) -> Option<&str> {
        self.content.lines().nth(line_index)
    }
}
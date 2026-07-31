#[derive(Clone, Debug)]
pub struct SourceFile {
    #[allow(dead_code)]
    pub name: String,
    pub content: String,
}

impl SourceFile {
    pub fn new(name: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            content: content.into(),
        }
    }

    /// Helper to fetch specific lines for precise error reporting and diagnostics.
    #[allow(dead_code)]
    pub fn get_line(&self, line_index: usize) -> Option<&str> {
        self.content.lines().nth(line_index)
    }
}
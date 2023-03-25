
#[derive(Debug)]
pub enum ShoudevError{
    HighlightingError
}

impl From<tree_sitter_highlight::Error> for ShoudevError{
    fn from(_: tree_sitter_highlight::Error) -> Self {
        Self::HighlightingError
    }
}
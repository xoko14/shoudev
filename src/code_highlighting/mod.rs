use tree_sitter_highlight::{Highlighter, HighlightEvent, Highlight};
use std::fmt::Write;

use crate::errors::ShoudevError;

use self::languages::TS_LANGS;

mod languages;

pub fn highlight_code(w: &mut dyn Write, input: &str, language: &str) -> Result<(), ShoudevError>{
    let config = match TS_LANGS.get(language) {
        Some(c) => c,
        None => return Err(ShoudevError::HighlightingError)
    };

    let mut highlighter = Highlighter::new();
    let highlights = highlighter
        .highlight(&config, input.as_bytes(), None, |_| None)?;
    for highlight in highlights{
        let highlight = highlight?;
        match highlight{
            HighlightEvent::Source { start, end } => {
                // THIS SHOULD BE ESCAPED!!!!
                write!(w, "{}", &input[start..end]).unwrap();
            },
            HighlightEvent::HighlightStart(Highlight(i)) => {
                write!(w, r#"<i class=hh{}>"#, i).unwrap();
            },
            HighlightEvent::HighlightEnd => {   
                write!(w, r#"</i>"#).unwrap();
            }
        }
    }
    Ok(())
}
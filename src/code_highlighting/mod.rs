use axum::http::StatusCode;
use std::fmt::Write;
use tree_sitter_highlight::{Highlight, HighlightEvent, Highlighter};

use crate::errors::ShoudevError;

use self::languages::TS_LANGS;

mod languages;

pub fn highlight_code(w: &mut dyn Write, input: &str, language: &str) -> Result<(), ShoudevError> {
    let config = match TS_LANGS.get(language) {
        Some(c) => c,
        None => {
            return Err(ShoudevError::new(
                crate::errors::ShoudevErrorType::HighlightingError,
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    };

    let mut highlighter = Highlighter::new();
    let highlights = highlighter.highlight(&config, input.as_bytes(), None, |_| None)?;
    for highlight in highlights {
        let highlight = highlight?;
        match highlight {
            HighlightEvent::Source { start, end } => {
                write!(w, "{}", html_escape::encode_text(&input[start..end])).unwrap();
            }
            HighlightEvent::HighlightStart(Highlight(i)) => {
                write!(w, r#"<i class=hh{}>"#, i).unwrap();
            }
            HighlightEvent::HighlightEnd => {
                write!(w, r#"</i>"#).unwrap();
            }
        }
    }
    Ok(())
}


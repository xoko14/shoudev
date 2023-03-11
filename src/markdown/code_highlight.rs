use tree_sitter_highlight::{Highlighter, HighlightConfiguration, HighlightEvent, Highlight};
use std::fmt::Write;

pub fn highlight_code(w: &mut dyn Write, input: &str){
    let highlight_names = &[
        "attribute",
        "constant",
        "function.builtin",
        "function",
        "keyword",
        "operator",
        "property",
        "punctuation",
        "punctuation.bracket",
        "punctuation.delimiter",
        "string",
        "string.special",
        "tag",
        "type",
        "type.builtin",
        "variable",
        "variable.builtin",
        "variable.parameter",
    ];

    let mut config = HighlightConfiguration::new(
        tree_sitter_rust::language(),
        tree_sitter_rust::HIGHLIGHT_QUERY,
        "",
        "",
    ).expect("err");
    config.configure(highlight_names);

    let mut highlighter = Highlighter::new();
    let highlights = highlighter
        .highlight(&config, input.as_bytes(), None, |_| None)
        .expect("this error should be mapped");
    for highlight in highlights{
        let highlight = highlight.unwrap();
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
}
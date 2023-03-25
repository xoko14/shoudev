use pulldown_cmark::{Parser, Event, Tag, CodeBlockKind, html};

use crate::code_highlighting::highlight_code;

pub fn parse(text: &str) -> String{
    let parser = Parser::new(text);

    struct Code{
        lang: String,
        source: String
    }

    let mut current = None;

    let iter = parser.map(|e| {
        match &e {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(lang))) => {
                current = Some(Code{
                    lang: lang.to_string(),
                    source: Default::default()
                });
                return Event::Text("".into());
            },
            Event::End(Tag::CodeBlock(CodeBlockKind::Fenced(_))) => {
                //TODO: Stuff (highlighting, formatting...)
                if let Some(current) = current.take() {
                    let mut f_code = String::new();
                    match highlight_code(&mut f_code, &current.source, &current.lang){
                        Ok(_) => return Event::Html(format!("<pre class=highlighted-code><code>{}</code></pre>", f_code).into()),
                        Err(_) => return Event::Html(format!("<pre class=highlighted-code><code>{}</code></pre>", &current.source).into())
                    };
                    
                }
            },
            Event::Text(code) => {
                if let Some(current) = current.as_mut() {
                    current.source.push_str(code);
                    return Event::Text("".into());
                }
            },
            _ => {}
        }
        e
    });

    let mut out = String::new();
    html::push_html(&mut out, iter);
    out
}


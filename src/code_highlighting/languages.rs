use std::collections::HashMap;

use tree_sitter::Language;
use tree_sitter_highlight::HighlightConfiguration;

lazy_static::lazy_static!{
    

    pub static ref TS_LANGS: HashMap<String, HighlightConfiguration> = {
        let mut langs = HashMap::<String, HighlightConfiguration>::new();

        let highlight_names = &[
            "annotation",
            "attribute",
            "boolean",
            "character",
            "character.special",
            "comment",
            "conditional",
            "constant.builtin",
            "constant.macro",
            "constant",
            "constructor",
            "debug",
            "define",
            "enviroment",
            "enviroment.name",
            "error",
            "exception",
            "field",
            "float",
            "function.builtin",
            "function.macro",
            "function",
            "function.call",
            "include",
            "keyword",
            "keyword.function",
            "keyword.operator",
            "keyword.return",
            "label",
            "literal",
            "math",
            "method",
            "method.call",
            "namespace",
            "none",
            "number",
            "operator",
            "parameter",
            "parameter.reference",
            "pre.proc",
            "property",
            "punctuation.bracket",
            "punctuation.delimiter",
            "punctuation.special",
            "repeat",
            "storage.class",
            "storage.class.lifetime",
            "strike",
            "string",
            "string.escape",
            "string.regex",
            "string.special",
            "symbol",
            "tag",
            "tag.attribute",
            "tag.delimiter",
            "text",
            "text.reference",
            "title",
            "todo",
            "type",
            "type.builtin",
            "type.definition",
            "type.qualifier",
            "uri",
            "variable",
            "variable.builtin"
        ];

        langs.insert("rust".to_owned(), config_lang(
            tree_sitter_rust::language(),
            tree_sitter_rust::HIGHLIGHT_QUERY,
            "",
            "",
            highlight_names
        ));
        langs.insert("html".to_owned(), config_lang(
            tree_sitter_html::language(),
            tree_sitter_html::HIGHLIGHT_QUERY,
            tree_sitter_html::INJECTION_QUERY,
            "",
            highlight_names
        ));
        langs.insert("javascript".to_owned(), config_lang(
            tree_sitter_javascript::language(),
            tree_sitter_javascript::HIGHLIGHT_QUERY,
            tree_sitter_javascript::INJECTION_QUERY,
            tree_sitter_javascript::LOCALS_QUERY,
            highlight_names
        ));
        langs.insert("typescript".to_owned(), config_lang(
            tree_sitter_typescript::language_typescript(),
            tree_sitter_typescript::HIGHLIGHT_QUERY,
            "",
            tree_sitter_typescript::LOCALS_QUERY,
            highlight_names
        ));
        langs.insert("css".to_owned(), config_lang(
            tree_sitter_css::language(),
            tree_sitter_css::HIGHLIGHTS_QUERY,
            "",
            "",
            highlight_names
        )); 
        langs.insert("kdl".to_owned(), config_lang(
            tree_sitter_kdl::language(),
            tree_sitter_kdl::HIGHLIGHTS_QUERY,
            tree_sitter_kdl::INJECTIONS_QUERY,
            tree_sitter_kdl::LOCALS_QUERY,
            highlight_names
        ));
        langs.insert("c".to_owned(), config_lang(
            tree_sitter_c::language(),
            tree_sitter_c::HIGHLIGHT_QUERY,
            "",
            "",
            highlight_names
        ));
        langs.insert("cpp".to_owned(), config_lang(
            tree_sitter_cpp::language(),
            tree_sitter_cpp::HIGHLIGHT_QUERY,
            "",
            "",
            highlight_names
        ));
        langs.insert("go".to_owned(), config_lang(
            tree_sitter_go::language(),
            tree_sitter_go::HIGHLIGHT_QUERY,
            "",
            "",
            highlight_names
        ));
        langs.insert("java".to_owned(), config_lang(
            tree_sitter_java::language(),
            tree_sitter_java::HIGHLIGHT_QUERY,
            "",
            "",
            highlight_names
        ));
        langs.insert("python".to_owned(), config_lang(
            tree_sitter_python::language(),
            tree_sitter_python::HIGHLIGHT_QUERY,
            "",
            "",
            highlight_names
        ));
        langs.insert("csharp".to_owned(), config_lang(
            tree_sitter_c_sharp::language(),
            tree_sitter_c_sharp::HIGHLIGHT_QUERY,
            "",
            "",
            highlight_names
        ));
        langs.insert("toml".to_owned(), config_lang(
            tree_sitter_toml::language(),
            tree_sitter_toml::HIGHLIGHT_QUERY,
            "",
            "",
            highlight_names
        ));
        langs.insert("svelte".to_owned(), config_lang(
            tree_sitter_svelte::language(),
            tree_sitter_svelte::HIGHLIGHT_QUERY,
            tree_sitter_svelte::INJECTION_QUERY,
            tree_sitter_svelte::LOCALS_QUERY,
            highlight_names
        ));
        langs.insert("dart".to_owned(), config_lang(
            tree_sitter_dart::language(),
            "",
            "",
            "",
            highlight_names
        ));
        langs.insert("kotlin".to_owned(), config_lang(
            tree_sitter_kotlin::language(),
            "",
            "",
            "",
            highlight_names
        ));
        langs.insert("dockerfile".to_owned(), config_lang(
            tree_sitter_dockerfile::language(),
            "",
            "",
            "",
            highlight_names
        ));
        langs.insert("markdown".to_owned(), config_lang(
            tree_sitter_md::language(),
            tree_sitter_md::HIGHLIGHT_QUERY_BLOCK,
            tree_sitter_md::INJECTION_QUERY_BLOCK,
            "",
            highlight_names
        ));
        langs.insert("json".to_owned(), config_lang(
            tree_sitter_json::language(),
            tree_sitter_json::HIGHLIGHT_QUERY,
            "",
            "",
            highlight_names
        ));
        langs.insert("lua".to_owned(), config_lang(
            tree_sitter_lua::language(),
            "",
            "",
            "",
            highlight_names
        ));
        
        langs
    };
}

fn config_lang(lang: Language, highlights: &str,  injection: &str, locals: &str, names: &[&str; 67]) -> HighlightConfiguration{
   let mut config = HighlightConfiguration::new(
        lang,
        highlights,
        injection,
        locals,
    ).expect("err");
    config.configure(names);
    config
}
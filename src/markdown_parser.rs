use once_cell::sync::Lazy;
use pulldown_cmark::{Options, Parser};

static PARSER_OPTIONS: Lazy<Options> = Lazy::new(|| {
    // Set up options and parser. Strikethroughs are not part of the CommonMark standard
    // and we therefore must enable it explicitly.
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_YAML_STYLE_METADATA_BLOCKS);
    options
});

pub struct MarkdownParser;

impl MarkdownParser {
    pub fn to_html(markdown: &mut String) -> String {
        let parser = Parser::new_ext(markdown, *PARSER_OPTIONS);
        let mut html_output = String::new();
        pulldown_cmark::html::push_html(&mut html_output, parser);
        html_output
    }
}

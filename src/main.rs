use clap::{App, Arg};
use pulldown_cmark::{Options, Parser};

extern crate clap;

fn main() {
    let matches = App::new("Ecstatic")
        .version("0.1")
        .about("Simple and opinionated static blog generator")
        .arg(
            Arg::with_name("input-dir")
                .short("i")
                .long("input-dir")
                .takes_value(true)
                .value_name("ABSOLUTE PATH")
                .help("Sets the input dir that Ecstatic will read from."),
        )
        .arg(
            Arg::with_name("output-dir")
                .short("o")
                .long("output-dir")
                .takes_value(true)
                .value_name("ABSOLUTE PATH")
                .help("Sets the out dir that Ecstatic will write the static site files to."),
        )
        .get_matches();

    let input_dir = matches.value_of("input-dir").unwrap_or(".");
    let output_dir = matches.value_of("output-dir").unwrap_or(".");

    println!("Input directory: {}", input_dir);
    println!("Output directory: {}", output_dir);

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

    // For every file with .md in the input_dir, do the following and save it to a file
    let markdown_input =
        "---\ntitle: A static blog post\n---\n# Top level\n## Second level\nLorem ipsum";

    let markdown_parser = Parser::new_ext(markdown_input, options);
    let mut html_output = String::new();
    pulldown_cmark::html::push_html(&mut html_output, markdown_parser);

    print!("{}", html_output);
}

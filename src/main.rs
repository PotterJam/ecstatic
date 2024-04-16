use crate::markdown_parser::MarkdownParser;
use clap::{App, Arg};

extern crate clap;

mod markdown_parser;

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

    // For every file with .md in the input_dir, do the following and save it to a file
    let mut some_markdown = String::from(
        r#"---
title: A static blog post
---
# Top level
## Second level
Lorem ipsum"#,
    );

    let to_html = MarkdownParser::to_html(&mut some_markdown);
    let html_output = to_html;

    print!("{}", html_output);
}

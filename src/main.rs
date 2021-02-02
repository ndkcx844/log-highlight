use std::env;
use std::process;

extern crate log_highlight;
use log_highlight::config::Config;
use log_highlight::highlighter::{Highlighter};
use log_highlight::rule::Rules;

fn main() {
    // NOTE: 戻り値がある場合はunwrap_or_else
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        if ! err.to_string().is_empty() {
            eprintln!("error: create Config: {}", err);
        }
        usage();
        process::exit(1);
    });

    if config.show_help {
        usage();
        process::exit(0);
    }

    if config.show_rules {
        Rules::show();
        process::exit(0);
    }

    let highlighter = Highlighter::new(config.rules).unwrap_or_else(|err| {
        eprintln!("error: create Highlighter: {}", err);
        process::exit(1);
    });

    // NOTE: 戻り値がない場合は if let Err(e)
    if let Err(e) = highlighter.highlight(config.files) {
        eprintln!("error: highlight failed: {}", e);
        process::exit(1);
    }
}

fn usage() {
    println!("Usage: {} [OPTION]... [FILE]...", env!("CARGO_PKG_NAME"));
    println!("highlight each FILE to standard output based on rules.");
    println!("With no FILE, or when FILE is -, read standard input.");
    println!("");
    println!("  -c, --config=FILE    load highlight rules from FILE");
}

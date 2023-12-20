use std::{env, fmt::Error};
use text_colorizer::*;

#[derive(Debug)]
struct Arguments {
    target: String,
    replacement: String,
    filename: String,
    output: String,
}

fn parse_args(args: Vec<String>) -> Arguments {
    if args.len() < 5 {
        print_usage();
        eprintln!(
            "{} wrong number of arguments: expected 4, got {}.",
            "Error:".red().bold(),
            args.len()
        );
        std::process::exit(1);
    }

    Arguments {
        target: args[1].clone(),
        replacement: args[2].clone(),
        filename: args[3].clone(),
        output: args[4].clone(),
    }
}

fn print_usage() {
    eprintln!(
        "{} - change occurrences of one string into
        another",
        "quickreplace".green()
    );
    eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
}

/// Replaces all occurrences of `target` with `replacement` in `input`.
fn replace_text(input: &str, target: &str, replacement: &str) -> String {
    input.replace(target, replacement)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let parsed_args = parse_args(args);

    let file_content = match std::fs::read_to_string(&parsed_args.filename) {
        Ok(content) => content,
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}': {:?}",
                "Error:".red().bold(),
                &parsed_args.filename,
                e
            );
            std::process::exit(1);
        }
    };

    let replaced_text = replace_text(
        &file_content.as_str(),
        &parsed_args.target,
        &parsed_args.replacement,
    );

    match std::fs::write(&parsed_args.output, &replaced_text) {
        Ok(_) => (),
        Err(e) => {
            eprintln!(
                "{} failed to read from file '{}': {:?}",
                "Error:".red().bold(),
                &parsed_args.output,
                e
            );
            std::process::exit(1);
        }
    }

    std::fs::write(&parsed_args.output, &replaced_text).expect("Couldn't write to file");
    println!("{}", "Done!".green());
}

#[test]
fn test_replace_text() {
    let mut text = "Misco jones".to_string();
    text = replace_text(text.as_str(), "jones", "chavea");
    assert_eq!(text, "Misco chavea");
}

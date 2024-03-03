use std::io::{self, Write};
use std::process;
use std::{env, fs};

use cli::args::*;
use runtime::interpreter::evaluate;
use syntax::lexer::Lexer;
use syntax::parse::Parser;

mod cli;
mod error;
mod runtime;
mod syntax;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut options = RuntimeOptions {
        debug_mode: false,
        help: false,
        quiet: false,
    };
    if args.len() == 1 {
        repl(options)
    } else {
        // Determine if the first argument is a filename or a flag
        let first_arg = args.get(1).unwrap();
        let is_filename = first_arg.contains('.');

        for arg in args.iter().skip(2) {
            parse_arg(arg, &mut options)
        }

        if is_filename {
            let path = &args[1];
            if let Ok(contents) = fs::read_to_string(path) {
                parse_file(contents, options)
            } else {
                eprintln!("Error reading file: {}", path);
            }
        } else {
            parse_arg(first_arg, &mut options);
            repl(options)
        }

        // Check for other flags
    }
}

fn repl(options: RuntimeOptions) {
    println!("Velo REPL [beta]\nUse `quit` to exit safely\n");
    println!("NOTES TO SELF:");
    println!(
        "Implement function body parsing\nRefactors\nWarning Emission\nTuple Types in functions"
    );

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush stdout");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line :(");
        let input = input.trim();

        if input == "quit" {
            process::exit(0);
        }

        let mut lexer = Lexer::new(&input);
        let tokens = lexer.tokenize();
        let tokens = tokens.tokens;

        let mut parser = Parser::new(tokens);
        let _ = parser.parse();

        evaluate(parser.nodes, options.debug_mode);
    }
}

fn parse_file(contents: String, options: RuntimeOptions) {
    let mut lexer = Lexer::new(&contents);
    let tokens = lexer.tokenize();
    let tokens = tokens.tokens;

    let mut parser = Parser::new(tokens);
    let _ = parser.parse();
    println!("{:#?}", options);

    evaluate(parser.nodes, options.debug_mode);
}

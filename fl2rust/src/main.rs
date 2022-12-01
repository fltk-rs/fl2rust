#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

use fluid_parser::lexer::Lexer;
use fluid_parser::parser::Parser;
use std::error;
use std::fs;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let f = fs::read_to_string(&args[1])?;
    let lexer = Lexer::new(&f);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    println!("{}", fl2rust::gen::generate_with_directives_preamble(&ast));
    if args.contains(&"--print-ast".to_string()) {
        println!("{:#?}", ast);
    }
    Ok(())
}

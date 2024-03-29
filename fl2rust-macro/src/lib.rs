#![doc = include_str!("../README.md")]

extern crate proc_macro;
use proc_macro::TokenStream;
use fluid_parser::lexer::Lexer;
use fluid_parser::parser::Parser;

#[proc_macro]
pub fn include_ui(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let input = std::fs::read_to_string(&input[1..input.len() - 1]).unwrap();
    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    let out = fl2rust::gen::generate(&ast);
    out.parse().unwrap()
}

#[proc_macro]
pub fn build_ui(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let lexer = Lexer::new(&input);
    let mut parser = Parser::new(lexer);
    let ast = parser.parse();
    let out = fl2rust::gen::generate(&ast);
    out.parse().unwrap()
}
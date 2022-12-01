#![doc = include_str!("../README.md")]
#![allow(clippy::needless_doctest_main)]

pub mod gen;
mod utils;

use fluid_parser::lexer::Lexer;
use fluid_parser::parser::Parser;
use std::error;
use std::fs;
use std::path::*;

/// Generator struct
#[derive(Default)]
pub struct Generator {}

impl Generator {
    /// Takes an input and output files
    pub fn in_out<P: AsRef<Path>>(
        &self,
        inpath: P,
        outpath: P,
    ) -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(inpath)?;
        let lexer = Lexer::new(&content);
        let mut parser = Parser::new(lexer);
        fs::write(outpath, gen::generate(&parser.parse()))?;
        Ok(())
    }

    /// Takes an input and output files
    pub fn in_out_with_directives_preamble<P: AsRef<Path>>(
        &self,
        inpath: P,
        outpath: P,
    ) -> Result<(), Box<dyn error::Error>> {
        let content = fs::read_to_string(inpath)?;
        let lexer = Lexer::new(&content);
        let mut parser = Parser::new(lexer);
        fs::write(
            outpath,
            gen::generate_with_directives_preamble(&parser.parse()),
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        use super::*;
        let g = Generator::default();
        g.in_out("fl_tests/fl.fl", "fl_tests/fl.rs")
            .expect("Failed to generate rust from fl file!");
        g.in_out("fl_tests/fl2.fl", "fl_tests/fl2.rs")
            .expect("Failed to generate rust from fl file!");
        g.in_out("fl_tests/unt.fl", "fl_tests/unt.rs")
            .expect("Failed to generate rust from fl file!");
    }
}

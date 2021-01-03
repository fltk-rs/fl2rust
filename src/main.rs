use std::error;
use std::fs;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let f = fs::read_to_string(&args[1])?;
    println!("{}", fl2rust::gen::generate(&fl2rust::parser::parse(&f)));
    Ok(())
}
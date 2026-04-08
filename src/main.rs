use std::env::args;
use std::fs::read_to_string;
use eopl::grammar::ProgramParser;
use eopl::interp::value_of_program;

fn main() {
    let mut args = args();
    args.next();
    
    let input_path = args.next().expect("Missing input file path!");
    
    let content = read_to_string(input_path).expect("Cannot read file!");

    let parser = ProgramParser::new();
    let program = parser.parse(&content).expect("Parse Error!");
    
    match value_of_program(&program) {
        Ok(val) => println!("{}", val),
        Err(e) => eprintln!("Runtime Error: {:?}", e),
    }
}
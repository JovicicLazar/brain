use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

mod parser;
mod program;
mod token;
use crate::program::*;

static VERSION: &str = "1.0.0";

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        println!("Error: wrong number of arguments: {}", args[0]);
        return;
    }

    if args[1] == "--version" {
        println!("Brain: brainfuck interpreter version: {}", &VERSION);
        return;
    }

    let file_name = &args[1];

    // Check if the file has a .bf extension
    let path = Path::new(file_name);
    if path.extension().unwrap_or_default() != "bf" {
        println!("Error: file must have .bf extension");
        return;
    }

    let mut file = match File::open(file_name) {
        Ok(file) => file,
        Err(err) => {
            println!("Error: failed to open file: {}", err);
            return;
        }
    };

    // Read the contents of the file
    let mut code = String::new();
    if let Err(err) = file.read_to_string(&mut code) {
        println!("Error: failed to read file: {}", err);
        return;
    }

    let program = Program::compile(&code);
    run(program);

}
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate lalrpop_util;
extern crate extprim;

mod parser;
mod ast;
mod program;
mod errors;
mod lexer;
#[cfg(test)]
mod tests;

use std::env;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;
use parser::parse_Statements;
use program::{Program, RunningProgram, Memory};
use lexer::Lexer;
use lalrpop_util::{ErrorRecovery, ParseError};

use errors::Error;

fn main() {
    env_logger::init().unwrap();
    main_real().unwrap();
}

fn main_real() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut filename: String = String::from("/dev/stdin");
    let mut yo_filename: Option<String> = None;
    // FIXME -i, -d flags, timeout, default timeout
    match args.len() {
        2 => filename = args[1].clone(),
        3 => {
            filename = args[1].clone();
            yo_filename = Some(args[2].clone());
        },
        _ => {
            println!("Usage: hclrs FILENAME [MEMORY-IMAGE]");
            return Ok(());
        },
    }

    let path = Path::new(&filename);

    let file = File::open(path)?;
    let mut file_reader = BufReader::new(file);
    let mut contents = String::new();
    try!(file_reader.read_to_string(&mut contents));

    // FIXME: wrapping for ParseErrors (has lifetime issues)
    let mut errors = Vec::new();
    let mut lexer = Lexer::new(contents.as_str());
    let statements = parse_Statements(&mut errors, lexer).unwrap();

    let program = Program::new_y86(statements)?;
    let mut running_program = RunningProgram::new_y86(program);

    if let Some(filename) = yo_filename {
        let yo_file = File::open(Path::new(&filename))?;
        let mut yo_reader = BufReader::new(yo_file);
        debug!("about to load yo file");
        running_program.load_memory_y86(&mut yo_reader);
    } else {
        debug!("no yo file");
    }

    while !running_program.done() && running_program.cycle() < 100 {
        try!(running_program.step());
    }
    println!("{}", running_program.dump());
    println!("{}", running_program.dump_y86());
    Ok(())
}


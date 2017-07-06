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
mod io;
#[cfg(test)]
mod tests;

use std::io::Write;
use std::path::Path;
use lexer::Lexer;
use parser::parse_Statements;

pub use errors::Error;
pub use program::{Program, RunningProgram};
pub use io::FileContents;

pub fn read_y86_hcl(path: &Path) -> Result<FileContents, Error> {
    FileContents::new_from_file_with_preamble(program::Y86_PREAMBLE, path)
}

pub fn parse_y86_hcl(contents: &FileContents) -> Result<Program, Error> {
    let mut errors = Vec::new();
    let lexer = Lexer::new_for_file(contents);
    let statements;
    match parse_Statements(&mut errors, lexer) {
        Ok(s) => statements = s,
        Err(e) => {
            let mut errors: Vec<Error> = errors.into_iter().map(|err_rec| Error::from(err_rec)).collect();
            errors.push(Error::from(e));
            return Err(Error::MultipleErrors(errors));
        },
    }
    if errors.len() > 0 {
        return Err(Error::MultipleErrors(errors.into_iter().map(|err_rec| Error::from(err_rec)).collect()));
    }
    Program::new_y86(statements)
}

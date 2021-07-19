
#[macro_use]
extern crate log;
extern crate env_logger;

#[cfg(target_arch="wasm32")]
#[macro_use]
extern crate wasm_bindgen;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub parser);

#[cfg(feature="json")]
extern crate serde_json;
#[cfg(feature="json")]
#[macro_use]
extern crate serde;

#[cfg(target_arch="wasm32")]
extern crate console_error_panic_hook;

mod y86_disasm;
mod ast;
mod program;
mod errors;
mod lexer;
mod io;
#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use lexer::Lexer;
use lexer::LAST_LOC;
use parser::StatementsParser;
use std::panic::catch_unwind;
#[cfg(target_arch="wasm32")]
use wasm_bindgen::prelude::*;

pub use errors::Error;
pub use program::{Program, RunningProgram, RunOptions, Y86_PREAMBLE};
pub use io::FileContents;


pub fn read_y86_hcl(path: &Path) -> Result<FileContents, Error> {
    FileContents::new_from_file_with_preamble(program::Y86_PREAMBLE, path)
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn wasm_y86_hcl_to_file_contents(data: &str, filename: &str) -> FileContents {
    FileContents::new_from_data(program::Y86_PREAMBLE, data, filename)
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen(catch)]
pub fn wasm_parse_y86_hcl(contents: &FileContents) -> Result<Program, JsValue> {
    console_error_panic_hook::set_once();
    match parse_y86_hcl(contents) {
        Ok(s) => return Ok(s),
        Err(e) => {
            let mut error_u8s: Vec<u8> = Vec::new();
            e.format_for_contents(
                &mut error_u8s,
                contents
            ).unwrap();
            let error_string = String::from_utf8(error_u8s).unwrap();
            return Err(JsValue::from(error_string));
        },
    }
}

pub fn parse_y86_hcl(contents: &FileContents) -> Result<Program, Error> {
    match catch_unwind(|| internal_parse_y86_hcl(contents)) {
        Ok(x) => return x,
        Err(panic_value) => {
            let loc = LAST_LOC.with(|loc| { *loc.borrow() });
            return Err(Error::InternalParserErrorNear((loc, loc + 1), format!("{:?}", panic_value)));
        },
    }
}

#[cfg(target_arch="wasm32")]
fn convert_error_to_jsvalue(error: Error, file_contents: &FileContents) -> JsValue {
    let mut error_u8s: Vec<u8> = Vec::new();
    error.format_for_contents(&mut error_u8s, file_contents).unwrap();
    JsValue::from(String::from_utf8(error_u8s).unwrap())
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn wasm_new_run_options() -> RunOptions {
    RunOptions::default()
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen(catch)]
pub fn wasm_setup_program_y86(
    program: Program,
    yo_contents: String,
    run_options: RunOptions,
) -> Result<RunningProgram, JsValue> {
    let mut running_program = RunningProgram::new_y86(program);
    debug!("test");
    running_program.set_options(run_options);
    let mut yo_reader = yo_contents.as_bytes();
    match running_program.load_memory_y86(&mut yo_reader) {
        Ok(_) => {},
        Err(e) => {
            let empty_contents = FileContents::new_from_data("", "", "<unavailable>");
            let mut error_u8s: Vec<u8> = Vec::new();
            e.format_for_contents(&mut error_u8s, &empty_contents).unwrap();
            return Err(JsValue::from(String::from_utf8(error_u8s).expect("misformatted error")));
        }
    }
    Ok(running_program)
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen(catch)]
pub fn wasm_run_y86(running_program: &mut RunningProgram, contents: &FileContents) -> Result<String, JsValue> {
    let mut out: Vec<u8> = Vec::new();
    match running_program.run(&mut out) {
        Ok(_) => {},
        Err(e) => {
            return Err(convert_error_to_jsvalue(e, contents));
        }
    }
    Ok(String::from_utf8(out).unwrap())
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen(catch)]
pub fn wasm_step(running_program: &mut RunningProgram, contents: &FileContents) -> Result<String, JsValue> {
    let mut out: Vec<u8> = Vec::new();
    match running_program.step_with_output(&mut out) {
        Ok(_) => {},
        Err(e) => {
            return Err(convert_error_to_jsvalue(e, contents));
        }
    }
    Ok(String::from_utf8(out).unwrap())
}

#[cfg(target_arch="wasm32")]
#[wasm_bindgen]
pub fn wasm_dump_y86(running_program: &RunningProgram) -> String {
    running_program.dump_y86_str()
}

#[cfg(target_arch="wasm32")]
#[cfg(feature="json")]
#[wasm_bindgen]
pub fn wasm_running_program_state_as_json(running_program: &RunningProgram) -> JsValue {
    JsValue::from_serde(&running_program.state_as_json()).unwrap()
}

pub fn run_y86<W: Write>(mut running_program: RunningProgram, yo_path: &Path,
                         run_options: RunOptions, out: &mut W) -> Result<(), Error> {
    let mut yo_reader = BufReader::new(File::open(yo_path)?);
    running_program.load_memory_y86(&mut yo_reader)?;
    running_program.set_options(run_options);
    running_program.run(out)?;
    print!("{}", running_program.dump_y86_str());
    Ok(())
}


fn internal_parse_y86_hcl(contents: &FileContents) -> Result<Program, Error> {
    let mut errors = Vec::new();
    let lexer = Lexer::new_for_file(contents);
    let statements;
    match StatementsParser::new().parse(&mut errors, lexer) {
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

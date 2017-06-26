extern crate env_logger;
use std::env;
use std::fs::File;
use std::io::{BufReader, stderr};
use std::path::Path;

extern crate hclrs;

use hclrs::*;

fn main() {
    env_logger::init().unwrap();
    // FIXME: real error messages
    main_real().unwrap();
}

fn run_y86(file_contents: &FileContents, yo_path: &Path) -> Result<(), Error> {
    let program = parse_y86_hcl(file_contents)?;
    let mut running_program = RunningProgram::new_y86(program);
    let mut yo_reader = BufReader::new(File::open(yo_path)?);
    running_program.load_memory_y86(&mut yo_reader)?;
    running_program.run()?;
    println!("{}", running_program.dump_y86_str(true));
    Ok(())
}

fn main_real() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let filename: String;
    let yo_filename: String;
    // FIXME -i, -d flags, timeout, default timeout
    match args.len() {
        3 => {
            filename = args[1].clone();
            yo_filename = args[2].clone();
        },
        _ => {
            println!("Usage: hclrs FILENAME [MEMORY-IMAGE]");
            return Ok(());
        },
    }

    let path = Path::new(&filename);
    let file_contents = read_y86_hcl(path)?;
    let yo_path = Path::new(&yo_filename);
    match run_y86(&file_contents, yo_path) {
        Err(e) => format_error(&mut stderr(), &file_contents, &e)?,
        _ => {},
    }
    Ok(())
}


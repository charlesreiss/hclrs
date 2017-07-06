extern crate env_logger;
use std::env;
use std::fs::File;
use std::io::{BufReader, Write, stdout, stderr, sink};
use std::path::Path;

extern crate hclrs;
extern crate getopts;

use hclrs::*;

use getopts::Options;

fn main() {
    env_logger::init().unwrap();
    main_real().unwrap();
}

fn run_y86<W1: Write, W2: Write>(file_contents: &FileContents, yo_path: &Path,
           trace_out: &mut W1, step_out: &mut W2, dump_registers: bool, timeout: u32) -> Result<(), Error> {
    let program = parse_y86_hcl(file_contents)?;
    let mut running_program = RunningProgram::new_y86(program);
    let mut yo_reader = BufReader::new(File::open(yo_path)?);
    running_program.set_timeout(timeout);
    running_program.load_memory_y86(&mut yo_reader)?;
    running_program.run_with_trace(step_out, trace_out, dump_registers)?;
    println!("{}", running_program.dump_y86_str(dump_registers));
    Ok(())
}

fn print_usage(program_name: &str, opts: Options) { 
    let header = format!("Usage: {} [options] HCL-FILE YO-FILE [TIMEOUT]\nDefault timeout is 9999 cycles.",
        program_name);
    print!("{}", opts.usage(&header));
}

fn main_real() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("d", "debug", "output traces of all assignments for debugging");
    opts.optflag("q", "quiet", "only output state at the end");
    opts.optflag("t", "testing", "do not output custom register banks (for autograding)");
    opts.optflag("h", "help", "print this help menu");
    let parsed_opts = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if parsed_opts.opt_present("h") {
        print_usage(&program_name, opts);
        return Ok(());
    }
    let mut trace_out: Box<Write> = if parsed_opts.opt_present("q") { Box::new(sink()) } else { Box::new(stdout()) };
    let mut step_out: Box<Write> = if parsed_opts.opt_present("d") { Box::new(stdout()) } else { Box::new(sink()) };
    let dump_registers = !parsed_opts.opt_present("t");
    let free_args = parsed_opts.free;
    if free_args.len() < 2 || free_args.len() > 3 {
        print_usage(&program_name, opts);
        return Ok(());
    }
    let filename: &str = &free_args[0];
    let yo_filename: &str = &free_args[1];
    let timeout = 
        if free_args.len() > 2 {
            match u32::from_str_radix(&free_args[2], 10) {
                Ok(x) => x,
                Err(_) => panic!("timeout {} is not a number", &free_args[2]),
            }
        } else {
            9999
        };
    let path = Path::new(filename);
    let file_contents = read_y86_hcl(path)?;
    let yo_path = Path::new(yo_filename);
    match run_y86(&file_contents, yo_path, &mut trace_out, &mut step_out, dump_registers, timeout) {
        Err(e) => e.format_for_contents(&mut stderr(), &file_contents)?,
        _ => {},
    }
    Ok(())
}


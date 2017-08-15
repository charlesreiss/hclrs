extern crate env_logger;
use std::env;
use std::fs::File;
use std::io::{BufReader, Write, stdout, stderr, sink};
use std::path::Path;
use std::process;

extern crate hclrs;
extern crate getopts;

use hclrs::*;

use getopts::Options;

fn main() {
    env_logger::init().unwrap();
    let okay = main_real().unwrap();
    if okay {
        std::process::exit(0);
    } else {
        std::process::exit(1);
    }
}

fn parse_y86(file_contents: &FileContents) -> Result<RunningProgram, Error> {
    let program = parse_y86_hcl(file_contents)?;
    let running_program = RunningProgram::new_y86(program);
    Ok(running_program)
}

fn run_y86<W1: Write, W2: Write, W3: Write>(mut running_program: RunningProgram, yo_path: &Path,
           trace_out: &mut W1, step_out: &mut W2,
           disasm_out: &mut W3,  dump_registers: bool, timeout: u32) -> Result<(), Error> {
    let mut yo_reader = BufReader::new(File::open(yo_path)?);
    running_program.set_timeout(timeout);
    running_program.load_memory_y86(&mut yo_reader)?;
    running_program.run_with_trace(step_out, trace_out, disasm_out, dump_registers)?;
    print!("{}", running_program.dump_y86_str(dump_registers));
    Ok(())
}

fn print_usage(program_name: &str, opts: Options) { 
    let header = format!("Usage: {} [options] HCL-FILE [YO-FILE [TIMEOUT]]\n\
                          Runs HCL_FILE on YO-FILE. If --check is specified, no YO-FILE may be supplied.\n\
                          Default timeout is 9999 cycles.",
        program_name);
    print!("{}", opts.usage(&header));
}

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

fn main_real() -> Result<bool, Error> {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("c", "check", "check syntax only");
    opts.optflag("d", "debug", "output traces of all assignments for debugging");
    opts.optflag("q", "quiet", "only output state at the end");
    opts.optflag("t", "testing", "do not output custom register banks (for autograding)");
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("", "version", "print version number");
    let parsed_opts = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if parsed_opts.opt_present("h") {
        print_usage(&program_name, opts);
        return Ok(true);
    }
    if parsed_opts.opt_present("version") {
        print!("HCLRS version {}", VERSION);
        return Ok(true);
    }
    let mut step_out: Box<Write> = if parsed_opts.opt_present("q") { Box::new(sink()) } else { Box::new(stdout()) };
    let mut disasm_out: Box<Write> = if parsed_opts.opt_present("q") { Box::new(sink()) } else { Box::new(stdout()) };
    let mut trace_out: Box<Write> = if parsed_opts.opt_present("d") { Box::new(stdout()) } else { Box::new(sink()) };
    let dump_registers = !parsed_opts.opt_present("t");
    let check_only = parsed_opts.opt_present("c");
    let free_args = parsed_opts.free;
    if free_args.len() < 1 {
        print_usage(&program_name, opts);
        return Ok(false);
    }
    let filename: &str = &free_args[0];
    let path = Path::new(filename);
    let file_contents = match read_y86_hcl(path) {
        Err(e) => {
            writeln!(stderr(), "Error reading '{}': {}", path.display(), e).unwrap();
            return Ok(false)
        },
        Ok(contents) => contents
    };
    if free_args.len() > 3 {
        print_usage(&program_name, opts);
        return Ok(false);
    }
    let running_program =
        match parse_y86(&file_contents) {
            Err(e) => {
                e.format_for_contents(&mut stderr(), &file_contents)?;
                return Ok(false);
            },
            Ok(p) => p,
        };
    if check_only {
        return Ok(true);
    }
    if free_args.len() < 2 || free_args.len() > 3 {
        print_usage(&program_name, opts);
        return Ok(false);
    }
    let yo_filename: &str = &free_args[1];
    let timeout = 
        if free_args.len() > 2 {
            match u32::from_str_radix(&free_args[2], 10) {
                Ok(x) => x,
                Err(_) => {
                    writeln!(stderr(), "timeout {} is not a valid number", &free_args[2]).unwrap();
                    return Ok(false);
                }
            }
        } else {
            9999
        };
    let yo_path = Path::new(yo_filename);
    match run_y86(running_program, yo_path, &mut trace_out, &mut step_out, &mut disasm_out, dump_registers, timeout) {
        Err(e) => {
            e.format_for_contents(&mut stderr(), &file_contents)?;
            return Ok(false);
        },
        _ => {},
    }
    Ok(true)
}


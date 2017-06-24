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

use std::env;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;
use parser::{parse_Expr, parse_WireDecls, parse_ConstDecls, parse_Statements};
#[cfg(test)]
use ast::{Statement, Expr, ConstDecl, WireDecl, WireValue, WireValues, WireWidth, BinOpCode, UnOpCode, MuxOption};
#[cfg(test)]
use extprim::u128::u128;
use program::{Program, RunningProgram, Memory};
use lexer::Lexer;
use lalrpop_util::{ErrorRecovery, ParseError};

#[cfg(test)]
use std::sync::{Once, ONCE_INIT};

use errors::Error;

fn main() {
    main_real().unwrap();
}

fn main_real() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut filename: String = String::from("/dev/stdin");
    match args.len() {
        1 => {
            println!("Usage: hclrs FILENAME");
            return Ok(());
        },
        2 => filename = args[1].clone(),
        _ => {
            println!("Usage: hclrs FILENAME");
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

    while !running_program.done() && running_program.cycle() < 100 {
        try!(running_program.step());
    }
    println!("{}", running_program.dump());
    println!("{}", running_program.dump_y86());
    Ok(())
}

#[cfg(test)]
static TEST_LOGGER_ONCE: Once = ONCE_INIT;

#[cfg(test)]
fn init_test_logger() {
    TEST_LOGGER_ONCE.call_once(|| {
        env_logger::init().unwrap();
    })
}

type ParseErrorType<'input> = ParseError<usize, lexer::Tok<'input>, Error>;
type ErrorRecoveryType<'input> = ErrorRecovery<usize, (usize, &'input str), Error>;

#[cfg(test)]
fn parse_Expr_str<'input>(errors: &mut Vec<ErrorRecoveryType<'input>>, s: &'input str) ->
        Result<Box<Expr>, ParseErrorType<'input>> {
    let mut lexer = Lexer::new(s);
    parse_Expr(errors, lexer)
}

#[cfg(test)]
fn parse_WireDecls_str<'input>(errors: &mut Vec<ErrorRecoveryType<'input>>, s: &'input str) ->
        Result<Vec<WireDecl>, ParseErrorType<'input>> {
    let mut lexer = Lexer::new(s);
    parse_WireDecls(errors, lexer)
}


#[cfg(test)]
fn parse_ConstDecls_str<'input>(errors: &mut Vec<ErrorRecoveryType<'input>>, s: &'input str) ->
        Result<Vec<ConstDecl>, ParseErrorType<'input>> {
    let mut lexer = Lexer::new(s);
    parse_ConstDecls(errors, lexer)
}

#[cfg(test)]
fn parse_Statements_str<'input>(
    errors: &mut Vec<ErrorRecoveryType<'input>>,
    s: &'input str) -> Result<Vec<Statement>, ParseErrorType<'input>> {
    let mut lexer = Lexer::new(s);
    parse_Statements(errors, lexer)
}

#[test]
fn test_binops() {
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr_str(&mut errors, "0b1000 * 15").unwrap(),
        Box::new(Expr::BinOp(BinOpCode::Mul,
                    Box::new(Expr::Constant(WireValue::from_binary("1000"))),
                    Box::new(Expr::Constant(WireValue::from_decimal("15")))),
        )
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "0b1000 * 15 + 1").unwrap(),
        Box::new(Expr::BinOp(BinOpCode::Add,
            Box::new(Expr::BinOp(BinOpCode::Mul,
                                 Box::new(Expr::Constant(WireValue::from_binary("1000"))),
                                 Box::new(Expr::Constant(WireValue::from_decimal("15"))),
                                )),
            Box::new(Expr::Constant(WireValue::from_decimal("1"))),
        ))
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "0b1000 + 15 * 1").unwrap(),
        Box::new(Expr::BinOp(BinOpCode::Add,
            Box::new(Expr::Constant(WireValue::from_binary("1000"))),
            Box::new(Expr::BinOp(BinOpCode::Mul,
                                 Box::new(Expr::Constant(WireValue::from_decimal("15"))),
                                 Box::new(Expr::Constant(WireValue::from_decimal("1"))),
                                )),
        ))
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "0b1000 * 15 + 1 > 0").unwrap(),
        Box::new(Expr::BinOp(BinOpCode::Greater,
            parse_Expr_str(&mut errors, "0b1000 * 15 + 1").unwrap(),
            parse_Expr_str(&mut errors, "0").unwrap(),
        ))
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "0b1000 & (15 + 1) > 5 && 0x1234 < 3 || 4 >= 1 << 1 / 5").unwrap(),
        parse_Expr_str(&mut errors, "((0b1000 & (15 + 1)) > 5) && (0x1234 < 3) || (4 >= (1 << (1 / 5)))").unwrap()
    );
}

#[test]
fn test_unops() {
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr_str(&mut errors, "-0b1000").unwrap(),
        Box::new(Expr::UnOp(UnOpCode::Negate, Box::new(Expr::Constant(WireValue::from_binary("1000")))))
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "1+-0b1000").unwrap(),
        Box::new(Expr::BinOp(BinOpCode::Add,
            Box::new(Expr::Constant(WireValue::from_decimal("1"))),
            Box::new(Expr::UnOp(UnOpCode::Negate,
                Box::new(Expr::Constant(WireValue::from_binary("1000")))))
        ))
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "~42").unwrap(),
        Box::new(Expr::UnOp(UnOpCode::Complement,
            Box::new(Expr::Constant(WireValue::from_decimal("42")))))
    );
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_mux() {
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr_str(&mut errors, "[ 0 : 42; 0x42 : 43 ; 1 : 44; ]").unwrap(),
        Box::new(Expr::Mux(vec!(
            MuxOption {
                condition: Box::new(Expr::Constant(WireValue::from_decimal("0"))),
                value: Box::new(Expr::Constant(WireValue::from_decimal("42"))),
            },
            MuxOption {
                condition: Box::new(Expr::Constant(WireValue::from_hexadecimal("42"))),
                value: Box::new(Expr::Constant(WireValue::from_decimal("43"))),
            },
            MuxOption {
                condition: Box::new(Expr::Constant(WireValue::from_decimal("1"))),
                value: Box::new(Expr::Constant(WireValue::from_decimal("44"))),
            }
        )))
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "[ 0 : 42; 0x42 : 43 ; 1 : 44 ]").unwrap(),
        Box::new(Expr::Mux(vec!(
            MuxOption {
                condition: Box::new(Expr::Constant(WireValue::from_decimal("0"))),
                value: Box::new(Expr::Constant(WireValue::from_decimal("42"))),
            },
            MuxOption {
                condition: Box::new(Expr::Constant(WireValue::from_hexadecimal("42"))),
                value: Box::new(Expr::Constant(WireValue::from_decimal("43"))),
            },
            MuxOption {
                condition: Box::new(Expr::Constant(WireValue::from_decimal("1"))),
                value: Box::new(Expr::Constant(WireValue::from_decimal("44"))),
            }
        )))
    );
    assert_eq!(errors.len(), 0);
}


#[test]
fn test_wiredecls() {
    init_test_logger();
    let mut errors = Vec::new();
    assert_eq!(
        parse_WireDecls_str(&mut errors, "wire x : 32 , y : 2, z : 1").unwrap(),
        vec!(WireDecl { name: String::from("x"), width: WireWidth::Bits(32) },
             WireDecl { name: String::from("y"), width: WireWidth::Bits(2) },
             WireDecl { name: String::from("z"), width: WireWidth::Bits(1) })
    );
    assert_eq!(errors.len(), 0);
    errors.clear();
    assert_eq!(
        parse_WireDecls_str(&mut errors, "wire x : 64").unwrap(),
        vec!(WireDecl { name: String::from("x"), width: WireWidth::Bits(64) })
    );
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_constdecls() {
    init_test_logger();
    let mut errors = Vec::new();
    assert_eq!(
        parse_ConstDecls_str(&mut errors, "const x = 0x42, y=0").unwrap(),
        vec!(
            ConstDecl { name: String::from("x"), value: Box::new(
                Expr::Constant(WireValue::from_hexadecimal("42"))
            ) },
            ConstDecl { name: String::from("y"), value: Box::new(
                Expr::Constant(WireValue::from_decimal("0"))
            ) }
        )
    );
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_eval_binaryops() {
    init_test_logger();
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr_str(&mut errors, "0b1000 & 15").unwrap().evaluate_constant().unwrap(),
        WireValue { bits: u128::new(8), width: WireWidth::Bits(4) }
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "0b1000 & 15 == 0x8").unwrap().evaluate_constant().unwrap(),
        WireValue { bits: u128::new(1), width: WireWidth::Bits(1) }
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "1 ^ 0xFFFF == 0xFFFE").unwrap().evaluate_constant().unwrap(),
        WireValue { bits: u128::new(1), width: WireWidth::Bits(1) }
    );
}

#[test]
fn test_eval_unops() {
    init_test_logger();
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr_str(&mut errors, "-0b1000").unwrap().evaluate_constant().unwrap(),
        WireValue::from_binary("1000")
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "-0b01000").unwrap().evaluate_constant().unwrap(),
        WireValue::from_binary("11000")
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "1+-0b01000").unwrap().evaluate_constant().unwrap(),
        WireValue::from_binary("11001")
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "~42").unwrap().evaluate_constant().unwrap(),
        WireValue { bits: !u128::new(42), width: WireWidth::Unlimited }
    );
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_eval_mux() {
    init_test_logger();
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr_str(&mut errors, "[ 0 : 42; 0x42 : 43 ; 1 : 44; ]").unwrap().evaluate_constant().unwrap(),
        WireValue { bits: u128::new(43), width: WireWidth::Unlimited }
    );
    // FIXME: more tests
}

#[test]
fn test_program() {
    init_test_logger();
    let mut errors = Vec::new();
    let statements = parse_Statements_str(&mut errors,
        "const x = 42; wire y : 32; wire z : 32;
         z = [x > 43: 0; x < 43: y << 3; x == 43: 0]; y = x * 2;").unwrap();
    debug!("statements are {:?}", statements);
    let program = Program::new(statements, vec!()).unwrap();
    let mut running_program = RunningProgram::new(program, 0, 0);
    let mut expect_values = WireValues::new();
    expect_values.insert(String::from("x"), WireValue::from_decimal("42"));
    assert_eq!(running_program.values(), &expect_values);
    running_program.step().unwrap();
    expect_values.insert(String::from("y"), WireValue::from_decimal("84").as_width(WireWidth::from(32)));
    expect_values.insert(String::from("z"), WireValue::from_decimal("672").as_width(WireWidth::from(32)));
    assert_eq!(running_program.values(), &expect_values);
}

#[test]
fn test_program_registers() {
    init_test_logger();
    let mut errors = Vec::new();
    let statements = parse_Statements_str(&mut errors,
        "register xY { a: 32 = 1; };
         x_a = Y_a + 1;").unwrap();
    let program = Program::new(statements, vec!()).unwrap();
    let mut running_program = RunningProgram::new(program, 0, 0);
    assert_eq!(running_program.values().get("Y_a"), Some(&WireValue::from_decimal("1").as_width(WireWidth::from(32))));
    assert_eq!(running_program.values().get("x_a"), Some(&WireValue::from_decimal("1").as_width(WireWidth::from(32))));
    running_program.step().unwrap();
    assert_eq!(running_program.values().get("Y_a"), Some(&WireValue::from_decimal("1").as_width(WireWidth::from(32))));
    assert_eq!(running_program.values().get("x_a"), Some(&WireValue::from_decimal("2").as_width(WireWidth::from(32))));
    running_program.step().unwrap();
    assert_eq!(running_program.values().get("Y_a"), Some(&WireValue::from_decimal("2").as_width(WireWidth::from(32))));
    assert_eq!(running_program.values().get("x_a"), Some(&WireValue::from_decimal("3").as_width(WireWidth::from(32))));
}

#[test]
fn test_memory() {
    init_test_logger();
    let mut memory = Memory::new();
    assert_eq!(
        memory.read(0, 8),
        WireValue { bits: u128::new(0), width: WireWidth::Bits(64) }
    );
    assert_eq!(
        memory.read(1, 8),
        WireValue { bits: u128::new(0), width: WireWidth::Bits(64) }
    );
    assert_eq!(
        memory.read(9, 4),
        WireValue { bits: u128::new(0), width: WireWidth::Bits(32) }
    );
    memory.write(1, u128::new(0x0123456789ABCDEF), 8);
    assert_eq!(
        memory.read(5, 4),
        WireValue { bits: u128::new(0x01234567), width: WireWidth::Bits(32) }
    );
    assert_eq!(
        memory.read(3, 2),
        WireValue { bits: u128::new(0x89AB), width: WireWidth::Bits(16) }
    );
}

#[test]
fn test_memory_program() {
    init_test_logger();
    let mut errors = Vec::new();
    let statements = parse_Statements_str(&mut errors,
        "register xX { count: 64 = 1; }
        mem_read = X_count & 1 == 1;
        mem_write = !mem_read;
        mem_addr = 0x8 + X_count;
        mem_input = 0x0123456789ABCDEF;
        x_count = X_count + 1;
        pc = 0; Stat = 1;
        ").unwrap();
    let program = Program::new_y86(statements).unwrap();
    let mut running_program = RunningProgram::new_y86(program);
    assert_eq!(running_program.values().get("X_count"), Some(&WireValue::from_decimal("1").as_width(WireWidth::from(64))));
    assert_eq!(running_program.values().get("x_count"), Some(&WireValue::from_decimal("1").as_width(WireWidth::from(64))));
    running_program.step().unwrap();
    assert_eq!(running_program.values().get("mem_output"), Some(&WireValue::from_decimal("0").as_width(WireWidth::from(64))));
    running_program.step().unwrap();
    assert_eq!(running_program.values().get("mem_output"), Some(&WireValue::from_decimal("0").as_width(WireWidth::from(64))));
    running_program.step().unwrap();
    assert_eq!(running_program.values().get("mem_output"), Some(&WireValue::from_hexadecimal("000123456789ABCD").as_width(WireWidth::from(64))));
}

#[test]
fn test_eval_bitselect() {
    init_test_logger();
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr_str(&mut errors, "0b1001011[1..4]").unwrap().evaluate_constant().unwrap(),
        WireValue::from_binary("101")
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "0b1001011[0..4]").unwrap().evaluate_constant().unwrap(),
        WireValue::from_binary("1011")
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "0b1001011[0..1]").unwrap().evaluate_constant().unwrap(),
        WireValue::from_binary("1")
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "0b10001011[7..8]").unwrap().evaluate_constant().unwrap(),
        WireValue::from_binary("1")
    );
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_eval_bitconcat() {
    init_test_logger();
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr_str(&mut errors, "(0b100 .. 0b1011)").unwrap().evaluate_constant().unwrap(),
        WireValue::from_binary("1001011")
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "(0b1 .. 0b0)").unwrap().evaluate_constant().unwrap(),
        WireValue::from_binary("10")
    );
    assert_eq!(errors.len(), 0);
}

#[test]
fn test_regfile_program() {
    init_test_logger();
    let mut errors = Vec::new();
    let statements = parse_Statements_str(&mut errors,
        "register xX { count: 64 = 0; }
        wire count: 64;
        count = X_count;
        reg_inputE = count + 24;
        reg_dstE = (count & 0xF)[0..4];
        reg_srcA = ((count - 1) & 0xF)[0..4];
        x_count = X_count + 1;
        pc = 0; Stat = 1;
        ").unwrap();
    let program = Program::new_y86(statements).unwrap();
    let mut running_program = RunningProgram::new_y86(program);
    running_program.step().unwrap();
    let width64 = WireWidth::from(64);
    assert_eq!(running_program.values().get("reg_outputA"), Some(&WireValue::from_decimal("0").as_width(width64)));
    running_program.step().unwrap();
    assert_eq!(running_program.values().get("reg_outputA"), Some(&WireValue::from_decimal("24").as_width(width64)));
    running_program.step().unwrap();
    assert_eq!(running_program.values().get("reg_outputA"), Some(&WireValue::from_decimal("25").as_width(width64)));
    running_program.step().unwrap();
    assert_eq!(running_program.values().get("reg_outputA"), Some(&WireValue::from_decimal("26").as_width(width64)));
    for x in 3..16 {
        running_program.step().unwrap();
    }
    assert_eq!(running_program.values().get("reg_outputA"), Some(&WireValue::from_decimal("0").as_width(width64)));
    running_program.step().unwrap();
    assert_eq!(running_program.values().get("reg_outputA"), Some(&WireValue::from_decimal("40").as_width(width64)));
}

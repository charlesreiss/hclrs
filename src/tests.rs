use ast::{Statement, SpannedExpr, Expr, ConstDecl, WireDecl, WireValue, WireValues, WireWidth, BinOpCode, UnOpCode, MuxOption};
use program::{Program, RunningProgram, Y86_PREAMBLE};
use parser::{parse_Expr, parse_WireDecls, parse_ConstDecls, parse_Statements};
use lexer::{Lexer, Tok};
use errors::Error;
use super::*;

use extprim::u128::u128;
use lalrpop_util::{ErrorRecovery, ParseError};

use std::env;
use std::fs::{File, read_dir};
use std::io::{Read, BufReader};
use std::path::Path;
use std::sync::{Once, ONCE_INIT};
extern crate env_logger;

static TEST_LOGGER_ONCE: Once = ONCE_INIT;

type ParseErrorType<'input> = ParseError<usize, Tok<'input>, Error>;
type ErrorRecoveryType<'input> = ErrorRecovery<usize, Tok<'input>, Error>;

pub fn init_logger() {
    TEST_LOGGER_ONCE.call_once(|| {
        env_logger::init().unwrap();
    })
}

#[allow(non_snake_case)]
fn parse_Expr_str<'input>(errors: &mut Vec<ErrorRecoveryType<'input>>, s: &'input str) ->
        Result<SpannedExpr, ParseErrorType<'input>> {
    let lexer = Lexer::new(s);
    parse_Expr(errors, lexer)
}

#[allow(non_snake_case)]
fn parse_WireDecls_str<'input>(errors: &mut Vec<ErrorRecoveryType<'input>>, s: &'input str) ->
        Result<Vec<WireDecl>, ParseErrorType<'input>> {
    let lexer = Lexer::new(s);
    parse_WireDecls(errors, lexer)
}


#[allow(non_snake_case)]
fn parse_ConstDecls_str<'input>(errors: &mut Vec<ErrorRecoveryType<'input>>, s: &'input str) ->
        Result<Vec<ConstDecl>, ParseErrorType<'input>> {
    let lexer = Lexer::new(s);
    parse_ConstDecls(errors, lexer)
}

#[allow(non_snake_case)]
fn parse_Statements_str<'input>(
    errors: &mut Vec<ErrorRecoveryType<'input>>,
    s: &'input str) -> Result<Vec<Statement>, ParseErrorType<'input>> {
    let lexer = Lexer::new(s);
    parse_Statements(errors, lexer)
}

fn strip_spans(mut expr: SpannedExpr) -> SpannedExpr {
    expr.apply_to_all_mut(&mut |item| {
        item.span = (0, 0);
        Ok(())
    }).unwrap();
    expr
}

#[test]
fn parse_binops() {
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr_str(&mut errors, "0b1000 * 15").unwrap(),
        SpannedExpr::new(
            (0, 11),
            Expr::BinOp(BinOpCode::Mul,
                SpannedExpr::new((0, 6), Expr::Constant(WireValue::from_binary("1000"))),
                SpannedExpr::new((9, 11), Expr::Constant(WireValue::from_decimal("15")))
            ),
        )
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "0b1000 * 15 + 1").unwrap(),
        SpannedExpr::new(
            (0, 15),
            Expr::BinOp(BinOpCode::Add,
                SpannedExpr::new(
                    (0, 11),
                    Expr::BinOp(BinOpCode::Mul,
                        SpannedExpr::new((0, 6), Expr::Constant(WireValue::from_binary("1000"))),
                        SpannedExpr::new((9, 11), Expr::Constant(WireValue::from_decimal("15")))
                    ),
                ),
                SpannedExpr::new((14, 15),
                    Expr::Constant(WireValue::from_decimal("1"))
                ),
            ),
        )
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "0b1000 + 15 * 1").unwrap(),
        SpannedExpr::new(
            (0, 15),
            Expr::BinOp(BinOpCode::Add,
                SpannedExpr::new((0, 6), Expr::Constant(WireValue::from_binary("1000"))),
                SpannedExpr::new(
                    (9, 15),
                    Expr::BinOp(BinOpCode::Mul,
                        SpannedExpr::new((9, 11), Expr::Constant(WireValue::from_decimal("15"))),
                        SpannedExpr::new((14, 15), Expr::Constant(WireValue::from_decimal("1"))),
                    )
                ),
            ),
        )
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "0b1000 * 15 + 1 > 0").unwrap().to_expr(),
        &Expr::BinOp(BinOpCode::Greater,
            parse_Expr_str(&mut errors, "0b1000 * 15 + 1").unwrap(),
            SpannedExpr::new((18, 19), Expr::Constant(WireValue::from_decimal("0")))
        )
    );
    assert_eq!(
        strip_spans(
            parse_Expr_str(&mut errors, "  0b1000 & (15 + 1)  > 5  &&  0x1234 < 3  ||  4 >= 1  <<  1 / 5   ").unwrap()
        ),
        strip_spans(
            parse_Expr_str(&mut errors, "((0b1000 & (15 + 1)) > 5) && (0x1234 < 3) || (4 >= (1 << (1 / 5)))").unwrap()
        )
    );
}

#[test]
fn parse_unops() {
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr_str(&mut errors, "-0b1000").unwrap(),
        SpannedExpr::new((0, 7),
            Expr::UnOp(UnOpCode::Negate,
                SpannedExpr::new((1, 7), Expr::Constant(WireValue::from_binary("1000"))))
        )
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "1+-0b1000").unwrap(),
        SpannedExpr::new((0, 9), Expr::BinOp(BinOpCode::Add,
            SpannedExpr::new((0, 1), Expr::Constant(WireValue::from_decimal("1"))),
            SpannedExpr::new((2, 9), Expr::UnOp(UnOpCode::Negate,
                SpannedExpr::new((3, 9), Expr::Constant(WireValue::from_binary("1000")))))
        ))
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "~42").unwrap(),
        SpannedExpr::new((0, 3), Expr::UnOp(UnOpCode::Complement,
            SpannedExpr::new((1, 3), Expr::Constant(WireValue::from_decimal("42")))))
    );
    assert_eq!(errors.len(), 0);
}

#[test]
fn parse_mux() {
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr_str(&mut errors, "[0:42;0x42:43;1:44;]").unwrap(),
        SpannedExpr::new((0, 20),
            Expr::Mux(vec!(
                MuxOption {
                    condition: SpannedExpr::new(
                        (1, 2),
                        Expr::Constant(WireValue::from_decimal("0"))
                    ),
                    value: SpannedExpr::new(
                        (3, 5),
                        Expr::Constant(WireValue::from_decimal("42"))
                    ),
                },
                MuxOption {
                    condition: SpannedExpr::new(
                        (6, 10),
                        Expr::Constant(WireValue::from_hexadecimal("42"))
                    ),
                    value: SpannedExpr::new(
                        (11, 13),
                        Expr::Constant(WireValue::from_decimal("43"))
                    ),
                },
                MuxOption {
                    condition: SpannedExpr::new(
                        (14, 15),
                        Expr::Constant(WireValue::from_decimal("1"))
                    ),
                    value: SpannedExpr::new(
                        (16, 18),
                        Expr::Constant(WireValue::from_decimal("44"))
                    ),
                }
            ))
        )
    );
    assert_eq!(
        parse_Expr_str(&mut errors, "[0:42;0x42:43;1:44]").unwrap(),
        SpannedExpr::new((0, 19),
            Expr::Mux(vec!(
                MuxOption {
                    condition: SpannedExpr::new(
                        (1, 2),
                        Expr::Constant(WireValue::from_decimal("0"))
                    ),
                    value: SpannedExpr::new(
                        (3, 5),
                        Expr::Constant(WireValue::from_decimal("42"))
                    ),
                },
                MuxOption {
                    condition: SpannedExpr::new(
                        (6, 10),
                        Expr::Constant(WireValue::from_hexadecimal("42"))
                    ),
                    value: SpannedExpr::new(
                        (11, 13),
                        Expr::Constant(WireValue::from_decimal("43"))
                    ),
                },
                MuxOption {
                    condition: SpannedExpr::new(
                        (14, 15),
                        Expr::Constant(WireValue::from_decimal("1"))
                    ),
                    value: SpannedExpr::new(
                        (16, 18),
                        Expr::Constant(WireValue::from_decimal("44"))
                    ),
                }
            ))
        )
    );
    assert_eq!(errors.len(), 0);
}


#[test]
fn parse_wiredecls() {
    init_logger();
    let mut errors = Vec::new();
    assert_eq!(
        parse_WireDecls_str(&mut errors, "wire x : 32 , y : 2, z : 1").unwrap(),
        vec!(WireDecl { name: String::from("x"), width: WireWidth::Bits(32), span: (5, 11), },
             WireDecl { name: String::from("y"), width: WireWidth::Bits(2), span: (14, 19), },
             WireDecl { name: String::from("z"), width: WireWidth::Bits(1), span: (21, 26), })
    );
    assert_eq!(errors.len(), 0);
    errors.clear();
    assert_eq!(
        parse_WireDecls_str(&mut errors, "wire x : 64").unwrap(),
        vec!(WireDecl { name: String::from("x"), width: WireWidth::Bits(64), span: (5, 11), })
    );
    assert_eq!(errors.len(), 0);
}

#[test]
fn parse_constdecls() {
    init_logger();
    let mut errors = Vec::new();
    assert_eq!(
        parse_ConstDecls_str(&mut errors, "const x = 0x42, y=0").unwrap(),
        vec!(
            ConstDecl { name: String::from("x"), name_span: (6, 7), value: SpannedExpr::new( (10, 14),
                Expr::Constant(WireValue::from_hexadecimal("42"))
            ) },
            ConstDecl { name: String::from("y"), name_span: (16, 17), value: SpannedExpr::new( (18, 19),
                Expr::Constant(WireValue::from_decimal("0"))
            ) }
        )
    );
    assert_eq!(errors.len(), 0);
}

#[test]
fn eval_binaryops() {
    init_logger();
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
fn eval_unops() {
    init_logger();
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
fn eval_mux() {
    init_logger();
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr_str(&mut errors, "[ 0 : 42; 0x42 : 43 ; 1 : 44; ]").unwrap().evaluate_constant().unwrap(),
        WireValue { bits: u128::new(43), width: WireWidth::Unlimited }
    );
    // FIXME: more tests
}

#[test]
fn simple_program() {
    init_logger();
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
fn program_registers() {
    init_logger();
    let mut errors = Vec::new();
    let statements = parse_Statements_str(&mut errors,
        "register xY { a: 32 = 1; };
         x_a = Y_a + 1;").unwrap();
    let program = Program::new(statements, vec!()).unwrap();
    let mut running_program = RunningProgram::new(program, 0, 0);
    assert_eq!(running_program.values().get("Y_a"), Some(&WireValue::from_decimal("1").as_width(WireWidth::from(32))));
    assert_eq!(running_program.values().get("x_a"), Some(&WireValue::from_decimal("1").as_width(WireWidth::from(32))));
    running_program.step().unwrap();
    assert_eq!(running_program.values().get("Y_a"), Some(&WireValue::from_decimal("2").as_width(WireWidth::from(32))));
    assert_eq!(running_program.values().get("x_a"), Some(&WireValue::from_decimal("2").as_width(WireWidth::from(32))));
    running_program.step().unwrap();
    assert_eq!(running_program.values().get("Y_a"), Some(&WireValue::from_decimal("3").as_width(WireWidth::from(32))));
    assert_eq!(running_program.values().get("x_a"), Some(&WireValue::from_decimal("3").as_width(WireWidth::from(32))));
}


#[test]
fn memory_program() {
    init_logger();
    let mut errors = Vec::new();
    let statements = parse_Statements_str(&mut errors,
        "register xX { count: 64 = 1; }
        mem_readbit = X_count & 1 == 1;
        mem_writebit = !mem_readbit;
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
fn eval_bitselect() {
    init_logger();
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
fn eval_bitconcat() {
    init_logger();
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
fn regfile_program() {
    init_logger();
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
    for _ in 3..16 {
        running_program.step().unwrap();
    }
    assert_eq!(running_program.values().get("reg_outputA"), Some(&WireValue::from_decimal("0").as_width(width64)));
    running_program.step().unwrap();
    assert_eq!(running_program.values().get("reg_outputA"), Some(&WireValue::from_decimal("40").as_width(width64)));
}

fn expect_execute(program: &Program, yo_path: &Path, expect_output_path: &Path) -> Result<(), Error> {
    debug!("expect_execute(..., {:?}, {:?})", yo_path, expect_output_path);
    let mut running_program = RunningProgram::new_y86((*program).clone());
    let mut yo_reader = BufReader::new(File::open(yo_path)?);
    running_program.load_memory_y86(&mut yo_reader)?;
    // FIXME: control with env var
    ///running_program.run_with_trace(&mut stderr()).unwrap();
    running_program.run()?;
    let result = running_program.dump_y86_str(false);
    let mut expect_output_reader = BufReader::new(File::open(expect_output_path)?);
    let mut expect_output = String::new();
    expect_output_reader.read_to_string(&mut expect_output)?;
    if !expect_output_path.to_str().unwrap().contains("poptest") {
        assert_eq!(expect_output, result,
            "reference:\n{}\nactual:\n{}\n", expect_output, result
        );
    } else {
        if expect_output != result {
            warn!("*** disagreement on poptest for {}", expect_output_path.to_str().unwrap());
        }
    }
    Ok(())
}

fn check_hcl_with_references(hcl_path: &Path, reference_dir: &Path, yo_dir: &Path) -> Result<(), Error> {
    let file_contents = read_y86_hcl(hcl_path)?;
    let program = parse_y86_hcl(&file_contents)?;
    for entry in read_dir(reference_dir).unwrap() {
        let entry = entry.unwrap();
        if entry.file_name().to_str().unwrap().ends_with(".txt") {
            let ref_path = entry.path();
            let basename = ref_path.file_stem().unwrap();
            let mut yo_file = String::from(basename.to_str().unwrap());
            yo_file.push_str(".yo");
            let yo_file = yo_dir.join(yo_file);
            assert!(yo_file.is_file(), "{:?} is not file", yo_file);
            expect_execute(&program, yo_file.as_path(), ref_path.as_path())?;
        }
    }
    Ok(())
}

fn check_reference_dir(dir: &Path) {
    let mut errors = Vec::new();
    let mut entries = Vec::new();
    for entry in read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        if entry.file_name().to_str().unwrap().ends_with(".hcl") {
            entries.push(entry.path().to_owned());
        }
    }
    entries.sort();
    for entry in entries {
        let hcl_path = entry.as_path();
        debug!("found hcl {:?}", hcl_path);
        let basename = hcl_path.file_stem().unwrap();
        let mut reference_dir = String::from(basename.to_str().unwrap());
        reference_dir.push_str("-reference");
        let reference_dir = hcl_path.with_file_name(reference_dir);
        let yo_dir = hcl_path.with_file_name("y86");
        match check_hcl_with_references(hcl_path, reference_dir.as_path(),
                                  yo_dir.as_path()) {
            Err(e) => errors.push((hcl_path.to_owned(), e)),
            Ok(_) => {},
        }
    }
    if errors.len() > 0 {
        for (name, error) in errors {
            println!("{:?}: {:?}", name, error);
        }
        assert!(false);
    }
}

#[test] #[ignore]
fn external_reference() {
    init_logger();
    let mut dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).parent().unwrap().to_owned();
    dir.push("hclrs-testdata");
    assert!(dir.is_dir());
    check_reference_dir(&dir);
    let mut dir = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).parent().unwrap().to_owned();
    dir.push("hclrs-studentref");
    check_reference_dir(&dir);
}

fn get_errors_for(code: &str) -> String {
    let file_contents = FileContents::new_from_data(Y86_PREAMBLE, code, "test.hcl");
    match parse_y86_hcl(&file_contents) {
        Ok(_) => {
            panic!("expected compilation failure");
        },
        Err(e) => {
            let mut output: Vec<u8> = Vec::new();
            e.format_for_contents(&mut output, &file_contents).unwrap();
            return String::from_utf8(output).unwrap();
        }
    }
}

#[test]
fn error_mux_widths() {
    init_logger();
    let message = get_errors_for("
wire foo : 10, bar: 11, quux: 10;
foo = 0;
bar = 1;
quux = [
    foo > 3 : foo;
    foo < 3 : bar;
    1 : 0;
];
Stat = STAT_AOK;
pc = 0;
");
    debug!("error message is {}", message);
    assert!(message.contains("Mismatched wire widths for mux options"));
    assert!(message.contains("1 option is 10 bits wide"));
    assert!(message.contains("1 option is 11 bits wide"));
    assert!(message.contains("foo > 3 : foo;"));
    assert!(message.contains("foo < 3 : bar;"));
}

#[test]
fn error_expr_widths() {
    init_logger();
    let message = get_errors_for("
wire foo : 10, bar: 11, quux: 10;
foo = 0;
bar = 1;
quux = foo
            &
       bar;
Stat = STAT_AOK;
pc = 0;
");
    debug!("error message is {}", message);
    assert!(message.contains("Mismatched wire widths."));
    assert!(message.contains("is 10 bits wide"));
    assert!(message.contains("is 11 bits wide"));
    assert!(message.contains("quux = foo"));
    assert!(message.contains("bar;"));
}

#[test]
fn error_wire_widths() {
    init_logger();
    let message = get_errors_for("
wire foo : 10, bar: 11;
foo = 0;
bar = foo;
Stat = STAT_AOK;
pc = 0;
");
    debug!("error message is {}", message);
    assert!(message.contains("Mismatched wire widths."));
    assert!(message.contains("The wire 'bar' is declared as 11 bits wide."));
    assert!(message.contains("a 10 bit wide value is assigned to it"));
    assert!(message.contains("foo;"));
}

#[test]
fn error_register_init_widths() {
    init_logger();
    let message = get_errors_for("
register xF {
    foo : 14 = 0b100;
};

x_foo = 1;

Stat = STAT_AOK;
pc = 0;
");
    debug!("error message is {}", message);
    assert!(message.contains("Register 'foo' in bank 'xF' is 14 bits wide, but "));
    assert!(message.contains("3 bits wide:"));
    assert!(message.contains("= 0b100"));
}

#[test]
fn error_duplicate_register() {
    init_logger();
    let message = get_errors_for("
register xF {
    foo : 14 = 0;
    foo : 14 = 0;
};

Stat = STAT_AOK;
pc = 0;
");
    debug!("error message is {}", message);
    assert!(message.contains("Register 'foo' in bank 'xF' defined twice."));
}

#[test]
fn error_undefined_wire_assign() {
    init_logger();
    let message = get_errors_for("
foo = 42;
Stat = STAT_AOK;
pc = 0;
");
    debug!("error message is {}", message);
    assert!(message.contains("Undefined wire 'foo' assigned value:"));
    assert!(message.contains("foo = 42;"));
}

#[test]
fn error_undefined_wire_read() {
    init_logger();
    let message = get_errors_for("
wire foo : 16;
foo = bar + 42;
Stat = STAT_AOK;
pc = 0;
");
    debug!("error message is {}", message);
    assert!(message.contains("Usage of undefined value 'bar' in expression:"));
    assert!(message.contains("bar + 42"));
}

#[test]
fn error_nonconstant_wire_read_constant() {
    init_logger();
    let message = get_errors_for("
wire quux : 16;
const FOO = quux + 42;
quux = 42;
Stat = STAT_AOK;
pc = 0;
");
    debug!("error message is {}", message);
    assert!(message.contains("Usage of non-constant wire 'quux' in initial or constant value:"));
    assert!(message.contains("quux + 42"));
}

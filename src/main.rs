#[macro_use]
extern crate log;
extern crate env_logger;

extern crate lalrpop_util;
#[cfg(test)]
extern crate extprim;

pub mod parser;
mod ast;
mod program;
mod errors;

use std::env;
use parser::{parse_Expr, parse_WireDecls, parse_ConstDecls, parse_Statements};
#[cfg(test)]
use ast::{Expr, ConstDecl, WireDecl, WireValue, WireValues, WireWidth, BinOpCode, UnOpCode, MuxOption};
#[cfg(test)]
use extprim::u128::u128;
#[cfg(test)]
use program::Program;

fn main() {
    let mut errors = Vec::new();
    println!(
        "{:?}",
        parse_WireDecls(&mut errors, "wire x:32, y:2, z:1;").unwrap()
    );
    println!(
        "{:?}",
        parse_Expr(&mut errors, "0b1000").unwrap()
    );
    println!(
        "{:?}",
        parse_Expr(&mut errors, "0b1000 * 15").unwrap()
    );
    println!(
        "{:?}",
        parse_Expr(&mut errors, "0b1000 * 15 + 1").unwrap()
    );
    println!(
        "{:?}",
        parse_Expr(&mut errors, "0b1000 * 15 + 1 > 0").unwrap()
    );
    println!(
        "{:?}",
        parse_Expr(&mut errors, "0b1000 & (15 + 1) > 5 && 0x1234 < 3 || 4 >= 1 << 1 / 5").unwrap()
    );
}

#[test]
fn test_binops() {
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr(&mut errors, "0b1000 * 15").unwrap(),
        Box::new(Expr::BinOp(BinOpCode::Mul,
                    Box::new(Expr::Constant(WireValue::from_binary("1000"))),
                    Box::new(Expr::Constant(WireValue::from_decimal("15")))),
        )
    );
    assert_eq!(
        parse_Expr(&mut errors, "0b1000 * 15 + 1").unwrap(),
        Box::new(Expr::BinOp(BinOpCode::Add,
            Box::new(Expr::BinOp(BinOpCode::Mul,
                                 Box::new(Expr::Constant(WireValue::from_binary("1000"))),
                                 Box::new(Expr::Constant(WireValue::from_decimal("15"))),
                                )),
            Box::new(Expr::Constant(WireValue::from_decimal("1"))),
        ))
    );
    assert_eq!(
        parse_Expr(&mut errors, "0b1000 + 15 * 1").unwrap(),
        Box::new(Expr::BinOp(BinOpCode::Add,
            Box::new(Expr::Constant(WireValue::from_binary("1000"))),
            Box::new(Expr::BinOp(BinOpCode::Mul,
                                 Box::new(Expr::Constant(WireValue::from_decimal("15"))),
                                 Box::new(Expr::Constant(WireValue::from_decimal("1"))),
                                )),
        ))
    );
    assert_eq!(
        parse_Expr(&mut errors, "0b1000 * 15 + 1 > 0").unwrap(),
        Box::new(Expr::BinOp(BinOpCode::Greater,
            parse_Expr(&mut errors, "0b1000 * 15 + 1").unwrap(),
            parse_Expr(&mut errors, "0").unwrap(),
        ))
    );
    assert_eq!(
        parse_Expr(&mut errors, "0b1000 & (15 + 1) > 5 && 0x1234 < 3 || 4 >= 1 << 1 / 5").unwrap(),
        parse_Expr(&mut errors, "((0b1000 & (15 + 1)) > 5) && (0x1234 < 3) || (4 >= (1 << (1 / 5)))").unwrap()
    );
}

#[test]
fn test_unops() {
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr(&mut errors, "-0b1000").unwrap(),
        Box::new(Expr::UnOp(UnOpCode::Negate, Box::new(Expr::Constant(WireValue::from_binary("1000")))))
    );
    assert_eq!(
        parse_Expr(&mut errors, "1+-0b1000").unwrap(),
        Box::new(Expr::BinOp(BinOpCode::Add,
            Box::new(Expr::Constant(WireValue::from_decimal("1"))),
            Box::new(Expr::UnOp(UnOpCode::Negate,
                Box::new(Expr::Constant(WireValue::from_binary("1000")))))
        ))
    );
    assert_eq!(
        parse_Expr(&mut errors, "~42").unwrap(),
        Box::new(Expr::UnOp(UnOpCode::Complement,
            Box::new(Expr::Constant(WireValue::from_decimal("42")))))
    );
    assert_eq!(errors, vec!());
}

#[test]
fn test_mux() {
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr(&mut errors, "[ 0 : 42; 0x42 : 43 ; 1 : 44; ]").unwrap(),
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
        parse_Expr(&mut errors, "[ 0 : 42; 0x42 : 43 ; 1 : 44 ]").unwrap(),
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
    assert_eq!(errors, vec!());
}


#[test]
fn test_wiredecls() {
    let mut errors = Vec::new();
    assert_eq!(
        parse_WireDecls(&mut errors, "wire x : 32 , y : 2, z : 1").unwrap(),
        vec!(WireDecl { name: String::from("x"), width: WireWidth::Bits(32) },
             WireDecl { name: String::from("y"), width: WireWidth::Bits(2) },
             WireDecl { name: String::from("z"), width: WireWidth::Bits(1) })
    );
    assert_eq!(errors, vec!());
    errors.clear();
    assert_eq!(
        parse_WireDecls(&mut errors, "wire x : 64").unwrap(),
        vec!(WireDecl { name: String::from("x"), width: WireWidth::Bits(64) })
    );
    assert_eq!(errors, vec!());
}

#[test]
fn test_constdecls() {
    let mut errors = Vec::new();
    assert_eq!(
        parse_ConstDecls(&mut errors, "const x = 0x42, y=0").unwrap(),
        vec!(
            ConstDecl { name: String::from("x"), value: Box::new(
                Expr::Constant(WireValue::from_hexadecimal("42"))
            ) },
            ConstDecl { name: String::from("y"), value: Box::new(
                Expr::Constant(WireValue::from_decimal("0"))
            ) }
        )
    );
    assert_eq!(errors, vec!());
}

#[test]
fn test_eval_binaryops() {
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr(&mut errors, "0b1000 & 15").unwrap().evaluate_constant(),
        Ok(WireValue { bits: u128::new(8), width: WireWidth::Bits(4) })
    );
    assert_eq!(
        parse_Expr(&mut errors, "0b1000 & 15 == 0x8").unwrap().evaluate_constant(),
        Ok(WireValue { bits: u128::new(1), width: WireWidth::Bits(1) })
    );
    assert_eq!(
        parse_Expr(&mut errors, "1 ^ 0xFFFF == 0xFFFE").unwrap().evaluate_constant(),
        Ok(WireValue { bits: u128::new(1), width: WireWidth::Bits(1) })
    );
}

#[test]
fn test_eval_unops() {
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr(&mut errors, "-0b1000").unwrap().evaluate_constant(),
        Ok(WireValue::from_binary("1000"))
    );
    assert_eq!(
        parse_Expr(&mut errors, "-0b01000").unwrap().evaluate_constant(),
        Ok(WireValue::from_binary("11000"))
    );
    assert_eq!(
        parse_Expr(&mut errors, "1+-0b01000").unwrap().evaluate_constant(),
        Ok(WireValue::from_binary("11001"))
    );
    assert_eq!(
        parse_Expr(&mut errors, "~42").unwrap().evaluate_constant(),
        Ok(WireValue { bits: !u128::new(42), width: WireWidth::Unlimited })
    );
    assert_eq!(errors, vec!());
}

#[test]
fn test_eval_mux() {
    let mut errors = Vec::new();
    assert_eq!(
        parse_Expr(&mut errors, "[ 0 : 42; 0x42 : 43 ; 1 : 44; ]").unwrap().evaluate_constant(),
        Ok(WireValue { bits: u128::new(43), width: WireWidth::Unlimited })
    );
    // FIXME: more tests
}

#[test]
fn test_program() {
    env_logger::init().unwrap();
    let mut errors = Vec::new();
    let statements = parse_Statements(&mut errors,
        "const x = 42; wire y : 32; wire z : 32;
         z = [x > 43: 0; x < 43: y << 3; x == 43: 0]; y = x * 2;").unwrap();
    let program = Program::new(statements).unwrap();
    let mut values = program.constants();
    let mut expect_values = WireValues::new();
    expect_values.insert(String::from("x"), WireValue::from_decimal("42"));
    assert_eq!(&values, &expect_values);
    program.step_in_place(&mut values).unwrap();
    expect_values.insert(String::from("y"), WireValue::from_decimal("84"));
    expect_values.insert(String::from("z"), WireValue::from_decimal("672"));
    assert_eq!(&values, &expect_values);
}

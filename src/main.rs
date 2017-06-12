extern crate lalrpop_util;
#[cfg(test)]
extern crate extprim;

pub mod parser;
mod ast;

use parser::{parse_Expr, parse_WireDecls};
#[cfg(test)]
use ast::{Expr, WireDecl, WireValue, WireWidth, BinOpCode, UnOpCode};

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
fn test_wiredecls() {
    let mut errors = Vec::new();
    assert_eq!(
        parse_WireDecls(&mut errors, "wire x : 32 , y : 2, z : 1;").unwrap(),
        vec!(WireDecl { name: String::from("x"), width: WireWidth::Bits(32) },
             WireDecl { name: String::from("y"), width: WireWidth::Bits(2) },
             WireDecl { name: String::from("z"), width: WireWidth::Bits(1) })
    );
    assert_eq!(errors, vec!());
    errors.clear();
    assert_eq!(
        parse_WireDecls(&mut errors, "wire x : 64;").unwrap(),
        vec!(WireDecl { name: String::from("x"), width: WireWidth::Bits(64) })
    );
    assert_eq!(errors, vec!());
}

extern crate lalrpop_util;

pub mod parser;
mod ast;

use parser::{parse_Expr, parse_WireDecls};

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
        parse_Expr(&mut errors, "0b1000 * 15 + 0x1234 / 5 > 10").unwrap()
    );
}

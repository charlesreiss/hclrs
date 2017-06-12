extern crate extprim;

use extprim::u128::u128;

use std::str::FromStr;
use std::num::ParseIntError;
use std::cmp;

#[derive(Clone,Debug,Eq,PartialEq)]
pub enum WireWidth {
    Bits(usize),
    Unlimited,
}

impl From<usize> for WireWidth {
    fn from(s: usize) -> Self { WireWidth::Bits(s) }
}

impl FromStr for WireWidth {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match usize::from_str(s) {
            Ok(s) => Ok(WireWidth::Bits(s)),
            Err(x) => Err(x)
        }
    }
}

impl WireWidth {
    fn bits_or(&self, s: usize) -> usize {
        match self {
            &WireWidth::Bits(t) => t,
            &WireWidth::Unlimited => s,
        }
    }
        
    fn min(&self, w: WireWidth) -> WireWidth {
        match self {
            &WireWidth::Unlimited => w,
            &WireWidth::Bits(s) => WireWidth::Bits(cmp::min(s, w.bits_or(s)))
        }
    }
        
    fn max(&self, w: WireWidth) -> WireWidth {
        match self {
            &WireWidth::Unlimited => w,
            &WireWidth::Bits(s) => WireWidth::Bits(cmp::max(s, w.bits_or(s)))
        }
    }
}

// FIXME: disallow Eq?
#[derive(Clone,Eq,PartialEq,Debug)]
pub struct WireValue {
    pub bits: u128,
    pub width: WireWidth
}

impl WireValue {
    pub fn new(v: u128) -> WireValue {
        WireValue { bits: v, width: WireWidth::Unlimited }
    }

    pub fn from_binary(s: &str) -> WireValue {
        WireValue {
            bits: u128::from_str_radix(s, 2).unwrap(),
            width: WireWidth::Bits(s.len()),
        }
    }

    pub fn from_decimal(s: &str) -> WireValue {
        WireValue {
            bits: u128::from_str_radix(s, 10).unwrap(),
            width: WireWidth::Unlimited,
        }
    }
    
    pub fn from_hexadecimal(s: &str) -> WireValue {
        WireValue {
            bits: u128::from_str_radix(s, 16).unwrap(),
            width: WireWidth::Unlimited,
        }
    }
}

#[derive(Debug,Eq,PartialEq)]
pub struct WireDecl {
    pub name: String,
    pub width: WireWidth,
}

#[derive(Debug,Eq,PartialEq)]
pub enum BinOpCode {
    Add,
    Sub,
    Mul,
    Div,
    Or,
    Xor,
    And,
    Equal,
    NotEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    LogicalAnd,
    LogicalOr,
    LeftShift,
    RightShift,
}

#[derive(Debug,Eq,PartialEq)]
pub enum UnOpCode {
    Negate,
    Complement,
}

#[derive(Debug,Eq,PartialEq)]
pub struct MuxOption {
    condition: Box<Expr>,
    value: Box<Expr>,
}

#[derive(Debug,Eq,PartialEq)]
pub enum Expr {
    Constant(WireValue),
    BinOp(BinOpCode, Box<Expr>, Box<Expr>),
    UnOp(UnOpCode, Box<Expr>),
    Mux(Vec<MuxOption>),
}

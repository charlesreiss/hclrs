extern crate extprim;
extern crate num_traits;

use extprim::u128::u128;

use std::cmp;
use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::convert::From;
use std::num::ParseIntError;
use std::str::FromStr;

use self::num_traits::cast::ToPrimitive;

use errors::Error;


#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum WireWidth {
    Bits(u8),
    Unlimited,
}

impl From<usize> for WireWidth {
    fn from(s: usize) -> Self { WireWidth::Bits(s as u8) }
}

impl FromStr for WireWidth {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match u8::from_str(s) {
            Ok(s) => Ok(WireWidth::Bits(s)),
            Err(x) => Err(x)
        }
    }
}

impl WireWidth {
    fn bits_or(self, s: u8) -> u8 {
        match self {
            WireWidth::Bits(t) => t,
            WireWidth::Unlimited => s,
        }
    }

    pub fn min(self, w: WireWidth) -> WireWidth {
        match self {
            WireWidth::Unlimited => w,
            WireWidth::Bits(s) => WireWidth::Bits(cmp::min(s, w.bits_or(s)))
        }
    }

    pub fn max(self, w: WireWidth) -> WireWidth {
        match self {
            WireWidth::Unlimited => w,
            WireWidth::Bits(s) => WireWidth::Bits(cmp::max(s, w.bits_or(s)))
        }
    }

    pub fn combine(self, other: WireWidth) -> Result<WireWidth, Error> {
        match (self, other) {
            (WireWidth::Unlimited, _) => Ok(other),
            (_, WireWidth::Unlimited) => Ok(self),
            (WireWidth::Bits(s), WireWidth::Bits(t)) =>
                if s == t {
                    Ok(self)
                } else {
                    Err(Error::MismatchedWidths)
                }
        }
    }

    pub fn mask(self) -> u128 {
        match self {
            WireWidth::Unlimited => !u128::new(0),
            WireWidth::Bits(s) => ((!u128::new(0)) >> (128 - s)),
        }
    }
}

// FIXME: disallow Eq?
#[derive(Clone,Copy,Eq,PartialEq,Debug)]
pub struct WireValue {
    pub bits: u128,
    pub width: WireWidth
}

impl WireValue {
    pub fn new(v: u128) -> WireValue {
        WireValue { bits: v, width: WireWidth::Unlimited }
    }

    pub fn from_u64(v: u64) -> WireValue {
        WireValue { bits: u128::new(v), width: WireWidth::Unlimited }
    }

    pub fn from_binary(s: &str) -> WireValue {
        WireValue {
            bits: u128::from_str_radix(s, 2).unwrap(),
            width: WireWidth::Bits(s.len() as u8),
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

    pub fn as_width(self, new_width: WireWidth) -> WireValue {
        WireValue { bits: self.bits & new_width.mask(), width: new_width }
    }

    pub fn op<F>(self, other: WireValue, f: F, new_width: WireWidth) -> WireValue
            where F: Fn(u128, u128) -> u128 {
        WireValue { bits: f(self.bits, other.bits) & new_width.mask(), width: new_width }
    }

    pub fn is_true(self) -> bool {
        self.bits > u128::new(0)
    }

    pub fn is_false(self) -> bool {
        self.bits == u128::new(0)
    }
}

impl From<u64> for WireValue {
    fn from(x: u64) -> WireValue { WireValue::new(u128::new(x)) }
}

#[derive(Debug,Eq,PartialEq,Clone,Copy)]
enum BinOpKind {
    Boolean,
    EqualWidth
}

#[derive(Debug,Eq,PartialEq)]
pub struct WireDecl {
    pub name: String,
    pub width: WireWidth,
}

#[derive(Debug,Eq,PartialEq)]
pub struct ConstDecl {
    pub name: String,
    pub value: Box<Expr>,
}

#[derive(Debug,Eq,PartialEq,Clone,Copy)]
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

fn boolean_to_value(x: bool) -> u128 {
    if x { u128::new(1) } else { u128::new(0) }
}

impl BinOpCode {
    fn kind(self) -> BinOpKind {
        match self {
            BinOpCode::LogicalAnd => BinOpKind::Boolean,
            BinOpCode::LogicalOr => BinOpKind::Boolean,
            BinOpCode::Equal => BinOpKind::Boolean,
            BinOpCode::LessEqual => BinOpKind::Boolean,
            BinOpCode::GreaterEqual => BinOpKind::Boolean,
            BinOpCode::Less => BinOpKind::Boolean,
            BinOpCode::Greater => BinOpKind::Boolean,
            _ => BinOpKind::EqualWidth,
        }
    }

    fn apply_raw(self, left: u128, right: u128) -> u128 {
        match self {
            BinOpCode::Add => left.wrapping_add(right),
            BinOpCode::Sub => left.wrapping_sub(right),
            BinOpCode::Mul => left.wrapping_mul(right),
            BinOpCode::Div => left.wrapping_div(right),  // FIXME: handle divide-by-zero
            BinOpCode::Or =>  left | right,
            BinOpCode::Xor => left ^ right,
            BinOpCode::And => left & right,
            BinOpCode::Equal => boolean_to_value(left == right),
            BinOpCode::NotEqual => boolean_to_value(left != right),
            BinOpCode::LessEqual => boolean_to_value(left <= right),
            BinOpCode::GreaterEqual => boolean_to_value(left >= right),
            BinOpCode::Less => boolean_to_value(left < right),
            BinOpCode::Greater => boolean_to_value(left > right),
            BinOpCode::LogicalAnd => boolean_to_value(
                left != u128::new(0) && right != u128::new(0)
            ),  // FIXME: shortcircuit support?
            BinOpCode::LogicalOr =>  boolean_to_value(
                left != u128::new(0) || right != u128::new(0)
            ),
            BinOpCode::LeftShift =>  match (
                    left.wrapping_shl(right.to_u32().unwrap_or(0)),
                    right >= u128::new(128)
                ) {
                (_, true) => u128::new(0),
                (x, false) => x,
            },
            BinOpCode::RightShift => match (
                    left.wrapping_shr(right.to_u32().unwrap_or(0)),
                    right >= u128::new(128)
                ) {
                (_, true) => u128::new(0),
                (x, false) => x,
            },
        }
    }

    fn apply(self, left: WireValue, right: WireValue) -> Result<WireValue, Error> {
        let final_width = match self.kind() {
            BinOpKind::EqualWidth => try!(left.width.combine(right.width)),
            BinOpKind::Boolean => WireWidth::Bits(1),
        };
        Ok(left.op(right, |l, r| self.apply_raw(l, r), final_width))
    }
}

#[derive(Debug,Eq,PartialEq,Copy,Clone)]
pub enum UnOpCode {
    Negate,
    Complement,
}

impl UnOpCode {
    fn apply(self, value: WireValue) -> Result<WireValue, Error> {
        let new_value = match self {
            UnOpCode::Negate => !value.bits + u128::new(1),
            UnOpCode::Complement => !value.bits,
        };
        Ok(WireValue { bits: new_value & value.width.mask(), width: value.width })
    }
}

#[derive(Debug,Eq,PartialEq,Clone)]
pub struct MuxOption {
    pub condition: Box<Expr>,
    pub value: Box<Expr>,
}

#[derive(Debug,Eq,PartialEq,Clone)]
pub enum Expr {
    Constant(WireValue),
    BinOp(BinOpCode, Box<Expr>, Box<Expr>),
    UnOp(UnOpCode, Box<Expr>),
    Mux(Vec<MuxOption>),
    NamedWire(String),
}

pub type WireValues = HashMap<String, WireValue>;

impl Expr {
    pub fn width<'a>(&self, wires: &'a WireValues) -> Result<WireWidth, Error> {
        match *self {
            Expr::Constant(ref value) => Ok(value.width),
            Expr::BinOp(opcode, ref left, ref right) =>
                match opcode.kind() {
                    BinOpKind::EqualWidth => try!(left.width(wires)).combine(try!(right.width(wires))),
                    BinOpKind::Boolean => Ok(WireWidth::Bits(1)),
                },
            Expr::Mux(ref options) =>
                options.iter().fold(Ok(WireWidth::Unlimited),
                                    |maybe_width, ref item| try!(maybe_width).combine(try!(item.value.width(wires)))),
            Expr::UnOp(UnOpCode::Negate, _) => Ok(WireWidth::Bits(1)),
            Expr::UnOp(UnOpCode::Complement, ref covered) => covered.width(wires),
            Expr::NamedWire(ref name) => match wires.get(name) {
                Some(value) => Ok(value.width),
                None => Err(Error::UndefinedWire(name.clone())),
            },
            _ => unimplemented!(),
        }
    }

    pub fn evaluate_constant(&self) -> Result<WireValue, Error> {
        self.evaluate(&WireValues::new())
    }

    pub fn evaluate<'a>(&self, wires: &'a WireValues) -> Result<WireValue, Error> {
        match *self {
            Expr::Constant(value) => Ok(value),
            Expr::BinOp(opcode, ref left, ref right) => {
                let left_value = try!(left.evaluate(wires));
                let right_value = try!(right.evaluate(wires));
                opcode.apply(left_value, right_value)
            },
            Expr::UnOp(opcode, ref inner) => {
                let inner_value = try!(inner.evaluate(wires));
                opcode.apply(inner_value)
            },
            Expr::Mux(ref options) => {
                let mut result: WireValue = WireValue::new(u128::new(0));
                // FIXME: consider warning for using default?
                for ref option in options {
                    if try!(option.condition.evaluate(wires)).is_true() {
                        result = try!(option.value.evaluate(wires));
                        break;
                    }
                }
                Ok(result.as_width(try!(self.width(wires))))
            },
            Expr::NamedWire(ref name) => match wires.get(name) {
                Some(value) => Ok(*value),
                None => Err(Error::UndefinedWire(name.clone())),
            },
        }
    }

    fn accumulate_referenced_wires<'a, 'b>(&'a self, set: &'b mut HashSet<&'a str>) {
        match *self {
            Expr::Constant(_) => {},
            Expr::BinOp(_, ref left, ref right) => {
                left.accumulate_referenced_wires(set);
                right.accumulate_referenced_wires(set);
            },
            Expr::UnOp(_, ref inner) => {
                inner.accumulate_referenced_wires(set);
            },
            Expr::Mux(ref options) => {
                for ref option in options {
                    option.condition.accumulate_referenced_wires(set);
                    option.value.accumulate_referenced_wires(set);
                }
            },
            Expr::NamedWire(ref name) => {
                set.insert(name.as_str());
            },
        }
    }

    pub fn referenced_wires<'a>(&'a self) -> HashSet<&'a str> {
        let mut result = HashSet::new();
        self.accumulate_referenced_wires(&mut result);
        result
    }

    pub fn errors() -> Vec<Error> {
        unimplemented!();
    }
}

#[test]
fn test_referenced_wires() {
    let mut just_foo = HashSet::new();
    just_foo.insert("foo");
    let mut foo_and_bar = HashSet::new();
    foo_and_bar.insert("foo");
    foo_and_bar.insert("bar");
    assert_eq!(
        Expr::NamedWire(String::from("foo")).referenced_wires(),
        just_foo
    );
    assert_eq!(
        Expr::UnOp(UnOpCode::Negate,
            Box::new(Expr::NamedWire(String::from("foo")))).referenced_wires(),
        just_foo
    );
    assert_eq!(
        Expr::BinOp(BinOpCode::Add,
            Box::new(Expr::NamedWire(String::from("foo"))),
            Box::new(Expr::NamedWire(String::from("bar")))).referenced_wires(),
        foo_and_bar
    );
}

#[derive(Debug,Eq,PartialEq)]
pub struct Assignment {
    pub names: Vec<String>,
    pub value: Box<Expr>,
}


#[derive(Debug,Eq,PartialEq)]
pub enum Statement {
    ConstDecls(Vec<ConstDecl>),
    WireDecls(Vec<WireDecl>),
    Assignment(Assignment),
}

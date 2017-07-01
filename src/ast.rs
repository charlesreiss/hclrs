extern crate extprim;
extern crate num_traits;

use extprim::u128::u128;

use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;
use std::convert::From;
use std::fmt;
use std::fmt::{Display, LowerHex, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

use self::num_traits::cast::ToPrimitive;

use errors::Error;

// if true:
// *  require equality for non-bitwise binary ops; (otherwise, take maximum)
// *  require boolean arguments for &&, ||, etc.
const STRICT_WIRE_WIDTHS: bool = false;


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
    pub fn bits_or_128(&self) -> u8 {
        match *self {
            WireWidth::Bits(x) => x,
            _ => 128,
        }
    }

    pub fn possibly_boolean(self) -> bool {
        self == WireWidth::Unlimited || self == WireWidth::Bits(1)
    }

    pub fn combine(self, other: WireWidth) -> Option<WireWidth> {
        match (self, other) {
            (WireWidth::Unlimited, _) => Some(other),
            (_, WireWidth::Unlimited) => Some(self),
            (WireWidth::Bits(s), WireWidth::Bits(t)) =>
                if s == t {
                    Some(self)
                } else {
                    None
                }
        }
    }

    pub fn max(self, other: WireWidth) -> WireWidth {
        match (self, other) {
            (WireWidth::Unlimited, _) => other,
            (_, WireWidth::Unlimited) => self,
            (WireWidth::Bits(s), WireWidth::Bits(t)) =>
                if s > t {
                    self
                } else {
                    other
                }
        }
    }

    pub fn combine_exprs(self, other: WireWidth, left_expr: &Expr, right_expr: &Expr) -> Result<WireWidth, Error> {
        match self.combine(other) {
            Some(width) => Ok(width),
            None => Err(Error::MismatchedExprWidths(left_expr.clone(), right_expr.clone()))
        }
    }

    pub fn combine_expr_and_wire(self, other: WireWidth, wire: &str, right_expr: &Expr) -> Result<WireWidth, Error> {
        match self.combine(other) {
            Some(width) => Ok(width),
            None => Err(Error::MismatchedWireWidths(String::from(wire), right_expr.clone()))
        }
    }

    pub fn mask(self) -> u128 {
        match self {
            WireWidth::Unlimited => !u128::new(0),
            WireWidth::Bits(0) => u128::new(0),
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

impl Display for WireValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.bits)
    }
}

impl LowerHex for WireValue {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:x}", self.bits)
    }
}

impl WireValue {
    pub fn true_value() -> WireValue {
        WireValue { bits: u128::new(1), width: WireWidth::Bits(1) }
    }

    pub fn false_value() -> WireValue {
        WireValue { bits: u128::new(0), width: WireWidth::Bits(1) }
    }

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
}

impl From<u64> for WireValue {
    fn from(x: u64) -> WireValue { WireValue::new(u128::new(x)) }
}

#[derive(Debug,Eq,PartialEq,Clone,Copy)]
enum BinOpKind {
    BooleanCombine,
    BooleanFromEqualWidth,
    EqualWidth,
    EqualWidthWeak,
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
            BinOpCode::LogicalAnd => BinOpKind::BooleanCombine,
            BinOpCode::LogicalOr => BinOpKind::BooleanCombine,
            BinOpCode::Equal => BinOpKind::BooleanFromEqualWidth,
            BinOpCode::LessEqual => BinOpKind::BooleanFromEqualWidth,
            BinOpCode::GreaterEqual => BinOpKind::BooleanFromEqualWidth,
            BinOpCode::Less => BinOpKind::BooleanFromEqualWidth,
            BinOpCode::Greater => BinOpKind::BooleanFromEqualWidth,
            BinOpCode::NotEqual => BinOpKind::BooleanFromEqualWidth,

            BinOpCode::Add => BinOpKind::EqualWidthWeak,
            BinOpCode::Sub => BinOpKind::EqualWidthWeak,
            BinOpCode::Mul => BinOpKind::EqualWidthWeak,
            BinOpCode::Div => BinOpKind::EqualWidthWeak,

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
            BinOpKind::EqualWidth | BinOpKind::BooleanFromEqualWidth =>
                match left.width.combine(right.width) {
                    Some(width) => width,
                    None => return Err(Error::RuntimeMismatchedWidths()),
                },
            BinOpKind::EqualWidthWeak =>
                if STRICT_WIRE_WIDTHS {
                    match left.width.combine(right.width) {
                        Some(width) => width,
                        None => return Err(Error::RuntimeMismatchedWidths()),
                    }
                } else {
                    left.width.max(right.width)
                },
            BinOpKind::BooleanCombine => WireWidth::Bits(1),
        };
        Ok(left.op(right, |l, r| self.apply_raw(l, r), final_width))
    }
}

#[derive(Debug,Eq,PartialEq,Copy,Clone)]
pub enum UnOpCode {
    Negate,
    Complement,
    Not,
}

impl UnOpCode {
    fn apply(self, value: WireValue) -> Result<WireValue, Error> {
        let new_value = match self {
            UnOpCode::Negate => !value.bits + u128::new(1),
            UnOpCode::Complement => !value.bits,
            UnOpCode::Not => if value.bits != u128::new(0) { u128::new(0) } else { u128::new(1) },
        };
        Ok(WireValue { bits: new_value & value.width.mask(),
                       width: if self == UnOpCode::Not { WireWidth::Bits(1) } else { value.width } })
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
    BitSelect { from: Box<Expr>, low: u8, high: u8 },
    Concat(Box<Expr>, Box<Expr>),
    InSet(Box<Expr>, Vec<Expr>),
}

pub type WireValues = HashMap<String, WireValue>;

impl Expr {
    pub fn width<'a>(&self, widths: &'a HashMap<&'a str, WireWidth>) -> Result<WireWidth, Error> {
        match *self {
            Expr::Constant(ref value) => Ok(value.width),
            Expr::BinOp(opcode, ref left, ref right) =>
                match opcode.kind() {
                    BinOpKind::EqualWidth => left.width(widths)?.combine_exprs(right.width(widths)?, left, right),
                    BinOpKind::EqualWidthWeak => {
                        if STRICT_WIRE_WIDTHS {
                            left.width(widths)?.combine_exprs(right.width(widths)?, left, right)
                        } else {
                            Ok((left.width(widths)?).max(right.width(widths)?))
                        }
                    },
                    BinOpKind::BooleanCombine => {
                        if STRICT_WIRE_WIDTHS {
                            if !left.width(widths)?.possibly_boolean() {
                                return Err(Error::NonBooleanWidth((**left).clone()));
                            }
                            if !right.width(widths)?.possibly_boolean() {
                                return Err(Error::NonBooleanWidth((**right).clone()));
                            }
                        } else {
                            left.width(widths)?;
                            right.width(widths)?;
                        }
                        Ok(WireWidth::Bits(1))
                    },
                    BinOpKind::BooleanFromEqualWidth => {
                        // FIXME: consider non-strict wire widths case here
                        left.width(widths)?.combine_exprs(right.width(widths)?, left, right)?;
                        Ok(WireWidth::Bits(1))
                    }
                },
            Expr::Mux(ref options) => {
                let mut maybe_width = Some(WireWidth::Unlimited);
                for option in options {
                    if let Some(cur_width) = maybe_width {
                        maybe_width = cur_width.combine(option.value.width(widths)?);
                    } else {
                        break;
                    }
                }
                match maybe_width {
                    Some(width) => Ok(width),
                    None => Err(Error::MismatchedMuxWidths(options.clone()))
                }
            },
            Expr::UnOp(UnOpCode::Not, ref covered) => {
                covered.width(widths)?;
                Ok(WireWidth::Bits(1))
            },
            Expr::UnOp(_, ref covered) => covered.width(widths),
            Expr::NamedWire(ref name) => match widths.get(name.as_str()) {
                Some(ref width) => Ok(**width),
                None => Err(Error::UndefinedWire(name.clone())),
            },
            Expr::BitSelect { ref from, low, high } => {
                if low > high {
                    return Err(Error::MisorderedBitIndexes(self.clone()));
                }
                match from.width(widths)? {
                    WireWidth::Bits(inner_width) => {
                        if high > inner_width {
                            return Err(Error::InvalidBitIndex(self.clone(), high));
                        }
                    },
                    // FIXME: should we allow this?
                    WireWidth::Unlimited => {},
                }
                Ok(WireWidth::Bits(high - low))
            }
            Expr::Concat(ref left, ref right) => {
                if let WireWidth::Bits(left_width) = left.width(widths)? {
                    if let WireWidth::Bits(right_width) = right.width(widths)? {
                        if left_width + right_width <= 128 {
                            Ok(WireWidth::Bits(left_width + right_width))
                        } else {
                            Err(Error::WireTooWide(self.clone()))
                        }
                    } else {
                        Err(Error::NoBitWidth((**right).clone()))
                    }
                } else {
                    Err(Error::NoBitWidth((**left).clone()))
                }
            },
            Expr::InSet(ref left, ref lst) => {
                let left_width = left.width(widths)?;
                for item in lst {
                    match left_width.combine(item.width(widths)?) {
                        Some(_) => {},
                        None => {
                            return Err(Error::MismatchedExprWidths((**left).clone(), (*item).clone()));
                        },
                    }
                }
                Ok(WireWidth::Bits(1))
            },
        }
    }

    pub fn evaluate_constant(&self) -> Result<WireValue, Error> {
        self.evaluate(&WireValues::new())
    }

    pub fn evaluate<'a>(&self, wires: &'a WireValues) -> Result<WireValue, Error> {
        match *self {
            Expr::Constant(value) => Ok(value),
            Expr::BinOp(opcode, ref left, ref right) => {
                let left_value = left.evaluate(wires)?;
                let right_value = right.evaluate(wires)?;
                match opcode.apply(left_value, right_value) {
                    Err(Error::RuntimeMismatchedWidths()) => 
                        Err(Error::MismatchedExprWidths((**left).clone(), (**right).clone())),
                    Err(x) => Err(x),
                    Ok(x) => Ok(x),
                }
            },
            Expr::UnOp(opcode, ref inner) => {
                let inner_value = inner.evaluate(wires)?;
                opcode.apply(inner_value)
            },
            Expr::Mux(ref options) => {
                let mut result: WireValue = WireValue::new(u128::new(0));
                // FIXME: consider warning for using default?
                for ref option in options {
                    if option.condition.evaluate(wires)?.is_true() {
                        result = try!(option.value.evaluate(wires));
                        break;
                    }
                }
                // FIXME: do we need to adjust widths here?
                Ok(result)
            },
            Expr::NamedWire(ref name) => match wires.get(name) {
                Some(value) => Ok(*value),
                None => Err(Error::UndefinedWire(name.clone())),
            },
            Expr::BitSelect { ref from, low, high } => {
                let inner_value = from.evaluate(wires)?.bits;
                let shifted = inner_value >> low;
                Ok(WireValue::new(shifted).as_width(WireWidth::Bits(high - low)))
            },
            Expr::Concat(ref left, ref right) => {
                let left_value = left.evaluate(wires)?;
                let right_value = right.evaluate(wires)?;
                if let WireWidth::Bits(right_bits) = right_value.width {
                    if let WireWidth::Bits(left_bits) = left_value.width {
                        let shifted_left = left_value.bits << right_bits;
                        Ok(WireValue::new(shifted_left | right_value.bits).as_width(
                            WireWidth::Bits(left_bits + right_bits)))
                    } else {
                        Err(Error::NoBitWidth((**left).clone()))
                    }
                } else {
                    Err(Error::NoBitWidth((**right).clone()))
                }
            },
            Expr::InSet(ref left, ref lst) => {
                let left = left.evaluate(wires)?;
                for item in lst {
                    let right = item.evaluate(wires)?;
                    if left.bits == right.bits {
                        return Ok(WireValue::true_value());
                    }
                }
                Ok(WireValue::false_value())
            }
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
            Expr::BitSelect { ref from, .. } => {
                from.accumulate_referenced_wires(set);
            },
            Expr::Concat(ref left, ref right) => {
                left.accumulate_referenced_wires(set);
                right.accumulate_referenced_wires(set);
            },
            Expr::InSet(ref left, ref lst) => {
                left.accumulate_referenced_wires(set);
                for ref item in lst {
                    item.accumulate_referenced_wires(set);
                }
            }
        }
    }

    pub fn referenced_wires<'a>(&'a self) -> HashSet<&'a str> {
        let mut result = HashSet::new();
        self.accumulate_referenced_wires(&mut result);
        result
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
pub struct RegisterDecl {
    pub name: String,
    pub width: WireWidth,
    pub default: Box<Expr>,
}

#[derive(Debug,Eq,PartialEq)]
pub struct RegisterBankDecl {
    pub name: String,
    pub registers: Vec<RegisterDecl>,
}

#[derive(Debug,Eq,PartialEq)]
pub enum Statement {
    ConstDecls(Vec<ConstDecl>),
    WireDecls(Vec<WireDecl>),
    Assignments(Vec<Assignment>),
    RegisterBankDecl(RegisterBankDecl),
}

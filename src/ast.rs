extern crate extprim;

pub type WireWidth = usize;
pub type WireValue = extprim::u128::u128;

#[derive(Debug)]
pub struct WireDecl {
    pub name: String,
    pub width: WireWidth,
}

#[derive(Debug)]
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
}

#[derive(Debug)]
pub enum UnOpCode {
    Negate,
    Complement,
}

#[derive(Debug)]
pub struct MuxOption {
    condition: Box<Expr>,
    value: Box<Expr>,
}

#[derive(Debug)]
pub enum Expr {
    Constant(WireValue, WireWidth),
    BinOp(BinOpCode, Box<Expr>, Box<Expr>),
    UnOp(UnOpCode, Box<Expr>),
    Mux(Vec<MuxOption>),
}



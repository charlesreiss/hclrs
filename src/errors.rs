use std::convert::From;
use std::error;
use std::fmt;
use std::io;

use ast::{Expr, MuxOption};
use lexer::Loc;

#[derive(Debug)]
pub enum Error {
    MismatchedMuxWidths(Vec<MuxOption>),
    MismatchedExprWidths(Expr, Expr),
    MismatchedWireWidths(String, Expr),
    // register bank, register name, expr
    MismatchedRegisterDefaultWidths(String, String, Expr),
    RuntimeMismatchedWidths(),
    UndefinedWire(String),
    RedefinedWire(String),
    RedefinedBuiltinWire(String),
    UnsetWire(String),
    WireLoop(Vec<String>),
    InvalidWireWidth(Loc, Loc),
    InvalidRegisterBankName(String),
    InvalidBitIndex(Expr, u8),
    NoBitWidth(Expr),
    MisorderedBitIndexes(Expr),
    InvalidConstant(Loc, Loc),
    WireTooWide(Expr),
    UnterminatedComment(Loc),
    LexicalError(Loc),
    IoError(io::Error),
    EmptyFile(),
    UnparseableLine(String),
    MultipleErrors(Vec<Error>),
    // FIXME: multiple errors?
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Error::IoError(io_error)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::MismatchedMuxWidths(_) => "mismatched mux option widths",
            Error::MismatchedExprWidths(_, _) => "mismatched widths in expression",
            Error::MismatchedWireWidths(_, _) => "mismatched width between assignment value and wire",
            Error::MismatchedRegisterDefaultWidths(_, _, _) =>
                "mismatched width in default value for register",
            Error::RuntimeMismatchedWidths() => "mismatched width detected while evaluating expression",
            Error::UndefinedWire(_) => "undefined wire found",
            Error::RedefinedWire(_) => "multiply defined wire found",
            Error::RedefinedBuiltinWire(_) => "redefined wire from fixed functionality",
            Error::UnsetWire(_) => "wire defined but never set",
            Error::WireLoop(_) => "circular dependency between wires found",
            Error::InvalidWireWidth(_, _) => "wire width out of range",
            Error::InvalidRegisterBankName(_) => "invalid register bank name",
            Error::InvalidBitIndex(_, _) => "invalid bit index for bit-slicing",
            Error::InvalidConstant(_, _) => "constant is too big or small",
            Error::WireTooWide(_) => "wire would be wider than 128 bits",
            Error::NoBitWidth(_) => "expression has unknown bit width",
            Error::MisorderedBitIndexes(_) => "misordered bit indexes in bitslice",
            Error::UnterminatedComment(_) => "unterminated /*-style comment",
            Error::LexicalError(_) => "unrecognized token",
            Error::IoError(_) => "an I/O error occurred",
            Error::EmptyFile() => "empty input file",
            Error::UnparseableLine(_) => "unparseable line in input file",
            Error::MultipleErrors(_) => "multiple errors",
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::IoError(ref err) => Some(err as &error::Error),
            _ => None,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)?;
        Ok(())
    }
}

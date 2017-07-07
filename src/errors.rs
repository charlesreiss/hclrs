use std::collections::btree_map::BTreeMap;
use std::convert::From;
use std::error;
use std::fmt;
use std::io::Write;
use std::io;

use lalrpop_util::{ErrorRecovery, ParseError};

use ast::{SpannedExpr, MuxOption, WireWidth};
use lexer::{Tok, Loc, Span};
use io::FileContents;

#[derive(Debug)]
pub enum Error {
    MismatchedMuxWidths(Vec<MuxOption>, Vec<WireWidth>),
    MismatchedExprWidths(SpannedExpr, WireWidth, SpannedExpr, WireWidth),
    MismatchedWireWidths(String, WireWidth, SpannedExpr, WireWidth),
    // register bank, register name, expr
    MismatchedRegisterDefaultWidths {
        bank: String,
        register_name: String,
        register_width: WireWidth,
        default_expression: SpannedExpr,
        expression_width: WireWidth,
    },
    // FIXME: location of definitions?
    DuplicateRegister {
        bank: String,
        register_name: String,
    },
    RuntimeMismatchedWidths(),
    UndefinedWireAssigned(String, Span),
    UndefinedWireRead(String, SpannedExpr),
    NonConstantWireRead(String, SpannedExpr),
    UnsetWire(String, Span),
    UnsetBuiltinWire(String),
    RedeclaredWire(String, Span, Span),
    DoubleAssignedWire(String, Span, Span),
    DoubleAssignedFixedOutWire(String, Span),
    RedeclaredBuiltinWire(String, Span),
    PartialFixedInput {
        found_input: String,
        all_inputs: Vec<Vec<String>>,
    },
    WireLoop(Vec<String>),
    InvalidWireWidth(Span),
    // FIXME: location
    InvalidRegisterBankName(String, Span),
    InvalidBitIndex(SpannedExpr, u8),
    NonBooleanWidth(SpannedExpr),
    NoBitWidth(SpannedExpr),
    MisorderedBitIndexes(SpannedExpr),
    InvalidConstant(Span),
    WireTooWide(SpannedExpr),
    UnterminatedComment(Loc),
    LexicalError(Loc),
    EmptyFile(),
    UnparseableLine(String), // .yo input -- FIXME: rename
    InvalidToken(Loc),
    UnrecognizedToken { location: Span, expected: Vec<String> },
    ExtraToken(Span),
    MultipleErrors(Vec<Error>),
    IoError(io::Error),
    FmtError(fmt::Error),
}

fn error<W: Write>(output: &mut W, message: &str) -> Result<(), io::Error> {
    let mut first_line = true;
    for line in message.lines() {
        if first_line {
            write!(output, "error: {}\n", line)?;
            first_line = false;
        } else {
            write!(output, "       {}\n", line)?;
        }
    }
    Ok(())
}

fn error_continue<W: Write>(output: &mut W, message: &str) -> Result<(), io::Error> {
    for line in message.lines() {
        write!(output, "       {}\n", line)?;
    }
    Ok(())
}

fn s_are(i: usize) -> &'static str {
    if i != 1 {
        "s are"
    } else {
        " is"
    }
}

impl Error {
    pub fn format_for_contents<W: Write>(&self, output: &mut W, contents: &FileContents) -> Result<(), io::Error> {
        match *self {
            Error::MultipleErrors(ref vec) => {
                for item in vec {
                    item.format_for_contents(output, contents)?;
                }
            },
            Error::MismatchedMuxWidths(ref options, ref widths) => {
                error(output, "Mismatched wire widths for mux options.")?;
                let mut by_width = BTreeMap::new();
                for i in 0..options.len() {
                    let width = widths[i];
                    if width == WireWidth::Unlimited {
                        continue;
                    }
                    let ref option = options[i];
                    let lst = by_width.entry(width).or_insert(Vec::new());
                    lst.push(option)
                }
                for (width, lst) in by_width {
                    error_continue(output,
                                   &format!("{} option{} {} bits wide:\n",
                                            lst.len(), s_are(lst.len()), width.bits_or_128()))?;
                    for item in lst {
                        write!(output, "{}",
                               contents.show_region(item.value.span.0, item.value.span.1))?;
                    }
                }
            },
            Error::MismatchedExprWidths(ref first, first_width, ref second, second_width) => {
                error(output, &format!(
                    "Mismatched wire widths.\nOne side is {} bits wide:\n", first_width.bits_or_128()))?;
                write!(output, "{}", contents.show_region(first.span.0, first.span.1))?;
                error_continue(output, &format!(
                    "The other side is {} bits wide:\n", second_width.bits_or_128()))?;
                write!(output, "{}", contents.show_region(second.span.0, second.span.1))?;
            },
            Error::MismatchedWireWidths(ref name, first_width, ref second, second_width) => {
                error(output, &format!(
                    "Mismatched wire widths.\nThe wire '{}' is declared as {} bits wide.\n\
                     But a {} bit wide value is assigned to it:\n",
                    name, first_width.bits_or_128(), second_width.bits_or_128()))?;
                write!(output, "{}", contents.show_region(second.span.0, second.span.1))?;
            },
            Error::MismatchedRegisterDefaultWidths {
                ref bank, ref register_name, ref default_expression,
                ref register_width, ref expression_width
            } => {
                error(output, &format!(
                    "Register '{}' in bank '{}' is {} bits wide, but default value is\
                     {} bits wide:\n",
                     register_name, bank, register_width.bits_or_128(), expression_width.bits_or_128()))?;
                write!(output, "{}", contents.show_region(default_expression.span.0, default_expression.span.1))?;
            },
            // FIXME: show line numbers
            Error::DuplicateRegister { ref bank, ref register_name } => {
                error(output, &format!(
                    "Register '{}' in bank '{}' defined twice.", register_name, bank))?;
            },
            Error::RuntimeMismatchedWidths() => {
                error(output, &format!("Unexpected wire width disagreement."))?;
            },
            Error::UndefinedWireAssigned(ref name, ref span) => {
                // TODO: suggestions for wire meant?
                error(output, &format!(
                            "Undefined wire '{}' assigned value:",
                            name))?;
                write!(output, "{}", contents.show_region(span.0, span.1))?;
            },
            Error::UndefinedWireRead(ref name, ref expr) => {
                // TODO: suggestions for wire meant?
                error(output, &format!(
                            "Usage of undefined value '{}' in expression:",
                            name))?;
                write!(output, "{}", contents.show_region(expr.span.0, expr.span.1))?;
            },
            Error::NonConstantWireRead(ref name, ref expr) => {
                // TODO: suggestions for wire meant?
                error(output, &format!(
                            "Usage of non-constant wire '{}' in initial or constant value:",
                            name))?;
                write!(output, "{}", contents.show_region(expr.span.0, expr.span.1))?;
            },
            Error::UnsetWire(ref name, ref span) => {
                error(output, &format!(
                            "Wire '{}' never assigned but defined here:",
                            name))?;
                write!(output, "{}", contents.show_region(span.0, span.1))?;
            },
            Error::UnsetBuiltinWire(ref name) => {
                error(output, &format!(
                            "Wire '{}' required by fixed functionality but never assigned.",
                            name))?;
            },
            // FIXME: add where this error happens
            Error::RedeclaredWire(ref name, ref new_span, ref old_span) => {
                error(output, &format!("Wire '{}' redeclared. Declared here:", name))?;
                write!(output, "{}", contents.show_region(new_span.0, new_span.1))?;
                error_continue(output, "After being declared here here:")?;
                write!(output, "{}", contents.show_region(old_span.0, old_span.1))?;
            },
            Error::DoubleAssignedWire(ref name, ref new_span, ref old_span) => {
                error(output, &format!("Wire '{}' assigned twice. Assigned here:", name))?;
                write!(output, "{}", contents.show_region(new_span.0, new_span.1))?;
                error_continue(output, "After being assigned here:")?;
                write!(output, "{}", contents.show_region(old_span.0, old_span.1))?;
            },
            Error::DoubleAssignedFixedOutWire(ref name, ref new_span) => {
                error(output, &format!("Wire '{}' is output for fixed functionality but is assigned here:", name))?;
                write!(output, "{}", contents.show_region(new_span.0, new_span.1))?;
            },
            Error::RedeclaredBuiltinWire(ref name, ref new_span) => {
                error(output, &format!("Builtin wire '{}' redeclared here:", name))?;
                write!(output, "{}", contents.show_region(new_span.0, new_span.1))?;
            },
            Error::PartialFixedInput { ref found_input, ref all_inputs } => {
                // FIXME: error should identify missing input
                error(output, &format!("Wire '{}' set, but not the rest of this piece of fixed functionality.", found_input))?;
                for input_set in all_inputs.into_iter() {
                    let mut filtered_set: Vec<String> = input_set.into_iter().filter(|x| *x != found_input).cloned().collect();
                    filtered_set.sort();
                    let mut lst = String::from("");
                    if filtered_set.len() > 2 {
                        lst = filtered_set[0..filtered_set.len() - 1].join(", ");
                        lst.push_str(", and ");
                        lst.push_str(&filtered_set[filtered_set.len() - 1]);
                    } else if filtered_set.len() == 2 {
                        lst.push_str(&filtered_set[0]);
                        lst.push_str(" and ");
                        lst.push_str(&filtered_set[1]);
                    } else if filtered_set.len() == 1 {
                        lst.push_str(&filtered_set[0])
                    } else {
                        continue;
                    }
                    error_continue(output, &format!("(Did you mean to set {}?)", lst))?;
                }
            },
            Error::InvalidWireWidth(ref span) => {
                error(output, &format!("Invalid wire width specified."))?;
                write!(output, "{}", contents.show_region(span.0, span.1))?;
            },
            Error::InvalidRegisterBankName(ref name, ref span) => {
                error(output, &format!("Register bank name '{}' invalid.\nRegister bank names must be two characters.\n\
                                        The first character (input prefix) must be a lowercase letter.\n\
                                        The second character (output prefix) must be an uppercase lettter.",
                                        name))?;
                write!(output, "{}", contents.show_region(span.0, span.1))?;
            },
            // FIXME: expression width
            Error::InvalidBitIndex(ref expr, index) => {
                error(output, &format!("Bit index '{}' out of range for expression:", index))?;
                write!(output, "{}", contents.show_region(expr.span.0, expr.span.1))?;
            },
            Error::NonBooleanWidth(ref expr) => {
                error(output, &format!("Non-boolean value used with boolean operator:"))?;
                write!(output, "{}", contents.show_region(expr.span.0, expr.span.1))?;
            },
            Error::NoBitWidth(ref expr) => {
                error(output, &format!("Expression with unknown width used in bit concatenation:"))?;
                write!(output, "{}", contents.show_region(expr.span.0, expr.span.1))?;
            },
            Error::MisorderedBitIndexes(ref expr) => {
                error(output, "Bit selection expression selects less than 0 bits:")?;
                write!(output, "{}", contents.show_region(expr.span.0, expr.span.1))?;
            },
            Error::InvalidConstant(ref span) => {
                error(output, "Constant value is out of range:")?;
                write!(output, "{}", contents.show_region(span.0, span.1))?;
            },
            Error::WireTooWide(ref expr) => {
                error(output, "Expression would produce a value wider than supported (128 bits):")?;
                write!(output, "{}", contents.show_region(expr.span.0, expr.span.1))?;
            },
            Error::UnterminatedComment(start) => {
                error(output, "Unterminated comment starting here:")?;
                write!(output, "{}", contents.show_region(start, start + 2))?;
            },
            Error::LexicalError(start) | Error::InvalidToken(start) => {
                error(output, "Parse error here:")?;
                write!(output, "{}", contents.show_region(start, start + 1))?;
            },
            Error::EmptyFile() => {
                error(output, &format!("Empty input file."))?;
            },
            Error::UnparseableLine(ref line) => {
                error(output, &format!("Could not parse '{}' in .yo file.", line))?;
            },
            Error::UnrecognizedToken { ref location, ref expected } => {
                let token = &contents.data()[location.0..location.1];
                let mut expected_formatted = String::new();
                for possible_token in expected {
                    match possible_token.as_str() {
                        "ID" => expected_formatted.push_str("<an identifier>"),
                        "CONSTANT" => expected_formatted.push_str("<an integer constant>"),
                        _ => {
                            expected_formatted.push_str(possible_token);
                        }
                    }
                    expected_formatted.push_str(", ");
                }
                expected_formatted.pop();
                expected_formatted.pop();
                error(output, &format!("Unexpected token '{}', expected one of {}:",
                    token, expected_formatted))?;
                // heuristic check for missing semicolon at EOL
                if expected.iter().find(|x| *x == "\";\"").is_some() {
                    debug!("has semicolon");
                    let (number, start, next) = contents.line_number_and_bounds(location.0);
                    let before: &str = &contents.data()[start..location.0];
                    debug!("prefix is {:?}", before);
                    if before.find(|x: char| !x.is_whitespace()).is_none() {
                        error_continue(output, "(Missing semicolon before this?)");
                    }
                }
                write!(output, "{}", contents.show_region(location.0, location.1))?;
                // FIXME: note about missing ; if at beginning of line and ; is one of expected.
            },
            Error::ExtraToken(span) => {
                let token = &contents.data()[span.0..span.1];
                error(output, &format!("Unexpected token '{}':", token))?;
                write!(output, "{}", contents.show_region(span.0, span.1))?;
            },
            Error::WireLoop(ref lst) => {
                error(output, &format!("Circular dependency detected:"))?;
                for i in 0..lst.len() {
                    // FIXME: show code snippets of dependency?
                    error_continue(output, &format!("  '{}' depends on '{}'{}",
                        &lst[(i+1)%lst.len()], &lst[i],
                        if i == lst.len() - 1 { "" } else { " and" }
                        ))?;
                }
            },
            _ => {
                error(output, &format!("{:?}", *self))?;
            }
        }
        Ok(())
    }
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Error::IoError(io_error)
    }
}

impl From<fmt::Error> for Error {
    fn from(io_error: fmt::Error) -> Self {
        Error::FmtError(io_error)
    }
}

type ParseErrorType<'input> = ParseError<usize, Tok<'input>, Error>;
impl<'input> From<ParseErrorType<'input>> for Error {
    fn from(parse_error: ParseErrorType<'input>) -> Self {
        match parse_error {
            ParseError::InvalidToken { location } => Error::InvalidToken(location),
            ParseError::UnrecognizedToken { token, expected } => {
                match token {
                    Some(tuple) => Error::UnrecognizedToken {
                            location: (tuple.0, tuple.2),
                            expected: expected,
                        },
                    None => Error::UnrecognizedToken {
                            location: (usize::max_value(), usize::max_value()),
                            expected: expected
                        },
                }
            },
            ParseError::ExtraToken { token } => Error::ExtraToken((token.0, token.2)),
            ParseError::User { error } => error,
        }
    }
}

type ErrorRecoveryType<'input> = ErrorRecovery<usize, Tok<'input>, Error>;
impl<'input> From<ErrorRecoveryType<'input>> for Error {
    fn from(error_recovery: ErrorRecoveryType<'input>) -> Self {
        Error::from(error_recovery.error)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::MismatchedMuxWidths(_, _) => "mismatched mux option widths",
            Error::MismatchedExprWidths(_, _, _, _) => "mismatched widths in expression",
            Error::MismatchedWireWidths(_, _, _, _) => "mismatched width between assignment value and wire",
            Error::MismatchedRegisterDefaultWidths {..} => "mismatched width in default value for register",
            Error::DuplicateRegister {..} => "duplicate register in register bank",
            Error::RuntimeMismatchedWidths() => "mismatched width detected while evaluating expression",
            Error::UndefinedWireAssigned(_,_) => "undefined wire assigned",
            Error::UndefinedWireRead(_,_) => "undefined wire read",
            Error::NonConstantWireRead(_,_) => "non-constant wire read",
            Error::UnsetWire(_,_) => "wire defined but never assigned",
            Error::UnsetBuiltinWire(_) => "builtin wire required but never assigned",
            Error::DoubleAssignedWire(_,_,_) => "multiply assigned wire found",
            Error::DoubleAssignedFixedOutWire(_,_) => "wire assigned by fixed functionality also assigned manually",
            Error::RedeclaredWire(_,_,_) => "multiply defined wire found",
            Error::RedeclaredBuiltinWire(_,_) => "redefined wire from fixed functionality",
            Error::PartialFixedInput {..} => "part of fixed functionality set, but not all",
            Error::WireLoop(_) => "circular dependency between wires found",
            Error::InvalidWireWidth(_) => "wire width out of range",
            Error::InvalidRegisterBankName(_,_) => "invalid register bank name",
            Error::InvalidBitIndex(_, _) => "invalid bit index for bit-slicing",
            Error::InvalidConstant(_) => "constant is too big or small",
            Error::WireTooWide(_) => "wire would be wider than 128 bits",
            Error::NoBitWidth(_) => "expression has unknown bit width",
            Error::NonBooleanWidth(_) => "non-boolean operand to boolean operation",
            Error::MisorderedBitIndexes(_) => "misordered bit indexes in bitslice",
            Error::UnterminatedComment(_) => "unterminated /*-style comment",
            Error::LexicalError(_) => "unrecognized token",
            Error::EmptyFile() => "empty input file",
            Error::UnparseableLine(_) => "unparseable line in input file",
            Error::InvalidToken(_) => "invalid token", // FIXME: difference between this/unrecognized
            Error::UnrecognizedToken {..} => "unrecognized token",
            Error::ExtraToken(_) => "extra token",
            Error::MultipleErrors(_) => "multiple errors",
            Error::IoError(_) => "an I/O error occurred",
            Error::FmtError(_) => "a formatting error occurred",
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

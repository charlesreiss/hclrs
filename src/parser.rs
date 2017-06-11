use std::str::FromStr;
use ast::{Expr, WireDecl, WireWidth, WireValue, BinOpCode, UnOpCode};
use lalrpop_util::ErrorRecovery;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Expr {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Expr, WireDecl, WireWidth, WireValue, BinOpCode, UnOpCode};
    use lalrpop_util::ErrorRecovery;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_21_3d_22(&'input str),
        Term_22_26_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3c_22(&'input str),
        Term_22_3c_3d_22(&'input str),
        Term_22_3d_3d_22(&'input str),
        Term_22_3e_22(&'input str),
        Term_22_3e_3d_22(&'input str),
        Term_22_5e_22(&'input str),
        Term_22wire_22(&'input str),
        Term_22_7c_22(&'input str),
        Term_22_7e_22(&'input str),
        TermBIN(&'input str),
        TermDEC(&'input str),
        TermHEX(&'input str),
        TermID(&'input str),
        TermOCT(&'input str),
        Nt_28_3cWireDecl_3e_20_22_2c_22_29(WireDecl),
        Nt_28_3cWireDecl_3e_20_22_2c_22_29_2a(::std::vec::Vec<WireDecl>),
        Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(::std::vec::Vec<WireDecl>),
        NtBinOpAddSub(BinOpCode),
        NtBinOpCompare(BinOpCode),
        NtBinOpMulDiv(BinOpCode),
        NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(Box<Expr>),
        NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(Box<Expr>),
        NtBinTier_3cBinOpMulDiv_2c_20Term_3e(Box<Expr>),
        NtComma_3cWireDecl_3e(Vec<WireDecl>),
        NtExpr(Box<Expr>),
        NtExprAddSub(Box<Expr>),
        NtExprCompare(Box<Expr>),
        NtExprMulDiv(Box<Expr>),
        NtTerm(Box<Expr>),
        NtUnOp(UnOpCode),
        NtWireDecl(WireDecl),
        NtWireDecl_3f(::std::option::Option<WireDecl>),
        NtWireDecls(Vec<WireDecl>),
        Nt____Expr(Box<Expr>),
        Nt____Term(Box<Expr>),
        Nt____WireDecls(Vec<WireDecl>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 12, 13, 0, 0,
        // State 1
        -30, 0, 0, 0, 0, 15, 0, 16, 0, 0, 0, -30, -30, -30, -30, -30, 0, 0, 17, 0, 0, 0, 0, 0, 0,
        // State 2
        19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 21, 22, 23, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        -32, 26, 0, 0, 27, -32, 0, -32, 28, 0, 0, -32, -32, -32, -32, -32, 29, 0, -32, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -22, -22, -22, -22, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        -20, 0, 0, 0, 0, -20, 0, -20, 0, 0, 0, -20, -20, -20, -20, -20, 0, 0, -20, 0, 0, 0, 0, 0, 0,
        // State 8
        -24, -24, 0, 0, -24, -24, 0, -24, -24, 0, 0, -24, -24, -24, -24, -24, -24, 0, -24, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 40, 41, 0, 0,
        // State 10
        -35, -35, 0, 0, -35, -35, 0, -35, -35, 0, 0, -35, -35, -35, -35, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0,
        // State 11
        -33, -33, 0, 0, -33, -33, 0, -33, -33, 0, 0, -33, -33, -33, -33, -33, -33, 0, -33, 0, 0, 0, 0, 0, 0,
        // State 12
        -34, -34, 0, 0, -34, -34, 0, -34, -34, 0, 0, -34, -34, -34, -34, -34, -34, 0, -34, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 12, 13, 0, 0,
        // State 14
        0, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, -6, -6, 0, 0,
        // State 15
        0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, -7, -7, 0, 0,
        // State 16
        0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, -8, -8, 0, 0,
        // State 17
        0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 12, 13, 0, 0,
        // State 18
        0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, -10, -10, 0, 0,
        // State 19
        0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, -13, -13, 0, 0,
        // State 20
        0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, -11, -11, 0, 0,
        // State 21
        0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, -9, -9, 0, 0,
        // State 22
        0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, -14, -14, 0, 0,
        // State 23
        0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, -12, -12, 0, 0,
        // State 24
        0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 12, 13, 0, 0,
        // State 25
        0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, -18, -18, 0, 0,
        // State 26
        0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15, -15, 0, 0,
        // State 27
        0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16, -16, 0, 0,
        // State 28
        0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, -17, -17, 0, 0,
        // State 29
        -30, 0, 0, -30, 0, 15, 0, 16, 0, 0, 0, -30, -30, -30, -30, -30, 0, 0, 17, 0, 0, 0, 0, 0, 0,
        // State 30
        19, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 20, 21, 22, 23, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        -32, 26, 0, -32, 27, -32, 0, -32, 28, 0, 0, -32, -32, -32, -32, -32, 29, 0, -32, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        -22, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, -22, -22, -22, -22, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        -20, 0, 0, -20, 0, -20, 0, -20, 0, 0, 0, -20, -20, -20, -20, -20, 0, 0, -20, 0, 0, 0, 0, 0, 0,
        // State 36
        -24, -24, 0, -24, -24, -24, 0, -24, -24, 0, 0, -24, -24, -24, -24, -24, -24, 0, -24, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 40, 41, 0, 0,
        // State 38
        -35, -35, 0, -35, -35, -35, 0, -35, -35, 0, 0, -35, -35, -35, -35, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0,
        // State 39
        -33, -33, 0, -33, -33, -33, 0, -33, -33, 0, 0, -33, -33, -33, -33, -33, -33, 0, -33, 0, 0, 0, 0, 0, 0,
        // State 40
        -34, -34, 0, -34, -34, -34, 0, -34, -34, 0, 0, -34, -34, -34, -34, -34, -34, 0, -34, 0, 0, 0, 0, 0, 0,
        // State 41
        -19, 0, 0, 0, 0, -19, 0, -19, 0, 0, 0, -19, -19, -19, -19, -19, 0, 0, -19, 0, 0, 0, 0, 0, 0,
        // State 42
        -21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -21, -21, -21, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        -23, -23, 0, 0, -23, -23, 0, -23, -23, 0, 0, -23, -23, -23, -23, -23, -23, 0, -23, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 40, 41, 0, 0,
        // State 45
        0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 40, 41, 0, 0,
        // State 46
        0, 0, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 40, 41, 0, 0,
        // State 47
        -36, -36, 0, 0, -36, -36, 0, -36, -36, 0, 0, -36, -36, -36, -36, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        -19, 0, 0, -19, 0, -19, 0, -19, 0, 0, 0, -19, -19, -19, -19, -19, 0, 0, -19, 0, 0, 0, 0, 0, 0,
        // State 50
        -21, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, -21, -21, -21, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        -23, -23, 0, -23, -23, -23, 0, -23, -23, 0, 0, -23, -23, -23, -23, -23, -23, 0, -23, 0, 0, 0, 0, 0, 0,
        // State 52
        -36, -36, 0, -36, -36, -36, 0, -36, -36, 0, 0, -36, -36, -36, -36, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -30,
        -31,
        -32,
        -43,
        -22,
        -29,
        -20,
        -24,
        0,
        -35,
        -33,
        -34,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -19,
        -21,
        -23,
        0,
        0,
        0,
        -36,
        0,
        0,
        0,
        0,
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 2, 3, 4, 0, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 30, 31, 32, 0, 33, 34, 35, 36, 37, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 42, 9, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 2, 0, 4, 0, 0, 43, 0, 8, 9, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 44, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 30, 31, 32, 0, 49, 34, 35, 36, 37, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 50, 37, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 30, 0, 32, 0, 0, 51, 0, 36, 37, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""!=""###,
            r###""&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""^""###,
            r###""wire""###,
            r###""|""###,
            r###""~""###,
            r###"BIN"###,
            r###"DEC"###,
            r###"HEX"###,
            r###"ID"###,
            r###"OCT"###,
        ];
        __ACTION[(__state * 25)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Expr<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
    ) -> Result<Box<Expr>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                (4, _) if true => 4,
                (5, _) if true => 5,
                (6, _) if true => 6,
                (7, _) if true => 7,
                (8, _) if true => 8,
                (9, _) if true => 9,
                (10, _) if true => 10,
                (11, _) if true => 11,
                (12, _) if true => 12,
                (13, _) if true => 13,
                (14, _) if true => 14,
                (15, _) if true => 15,
                (16, _) if true => 16,
                (24, _) if true => 17,
                (17, _) if true => 18,
                (18, _) if true => 19,
                (21, _) if true => 20,
                (20, _) if true => 21,
                (23, _) if true => 22,
                (19, _) if true => 23,
                (22, _) if true => 24,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 25 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_21_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_26_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_2f_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_3b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_3c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_3c_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22_3d_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_3e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22_3e_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22_5e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (24, __tok0) => __Symbol::Term_22wire_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22_7c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22_7e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            (21, __tok0) => __Symbol::TermBIN((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            (20, __tok0) => __Symbol::TermDEC((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            (23, __tok0) => __Symbol::TermHEX((__tok0)),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            (19, __tok0) => __Symbol::TermID((__tok0)),
                            _ => unreachable!(),
                        },
                        24 => match __lookahead.1 {
                            (22, __tok0) => __Symbol::TermOCT((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Box<Expr>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<WireDecl> ",") = WireDecl, "," => ActionFn(39);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtWireDecl(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<WireDecl> ",")* =  => ActionFn(37);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action37::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<WireDecl> ",")* = (<WireDecl> ",")+ => ActionFn(38);
                let __sym0 = __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<WireDecl> ",")+ = WireDecl, "," => ActionFn(42);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtWireDecl(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action42::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<WireDecl> ",")+ = (<WireDecl> ",")+, WireDecl, "," => ActionFn(43);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtWireDecl(__symbols);
                let __sym0 = __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action43::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // BinOpAddSub = "+" => ActionFn(5);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpAddSub(__nt), __end));
                3
            }
            7 => {
                // BinOpAddSub = "-" => ActionFn(6);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpAddSub(__nt), __end));
                3
            }
            8 => {
                // BinOpAddSub = "|" => ActionFn(7);
                let __sym0 = __pop_Term_22_7c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpAddSub(__nt), __end));
                3
            }
            9 => {
                // BinOpCompare = "==" => ActionFn(12);
                let __sym0 = __pop_Term_22_3d_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            10 => {
                // BinOpCompare = "!=" => ActionFn(13);
                let __sym0 = __pop_Term_22_21_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            11 => {
                // BinOpCompare = "<=" => ActionFn(14);
                let __sym0 = __pop_Term_22_3c_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            12 => {
                // BinOpCompare = ">=" => ActionFn(15);
                let __sym0 = __pop_Term_22_3e_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            13 => {
                // BinOpCompare = "<" => ActionFn(16);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            14 => {
                // BinOpCompare = ">" => ActionFn(17);
                let __sym0 = __pop_Term_22_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            15 => {
                // BinOpMulDiv = "*" => ActionFn(8);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpMulDiv(__nt), __end));
                5
            }
            16 => {
                // BinOpMulDiv = "/" => ActionFn(9);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpMulDiv(__nt), __end));
                5
            }
            17 => {
                // BinOpMulDiv = "^" => ActionFn(10);
                let __sym0 = __pop_Term_22_5e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpMulDiv(__nt), __end));
                5
            }
            18 => {
                // BinOpMulDiv = "&" => ActionFn(11);
                let __sym0 = __pop_Term_22_26_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpMulDiv(__nt), __end));
                5
            }
            19 => {
                // BinTier<BinOpAddSub, ExprMulDiv> = BinTier<BinOpAddSub, ExprMulDiv>, BinOpAddSub, ExprMulDiv => ActionFn(30);
                let __sym2 = __pop_NtExprMulDiv(__symbols);
                let __sym1 = __pop_NtBinOpAddSub(__symbols);
                let __sym0 = __pop_NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(__nt), __end));
                6
            }
            20 => {
                // BinTier<BinOpAddSub, ExprMulDiv> = ExprMulDiv => ActionFn(31);
                let __sym0 = __pop_NtExprMulDiv(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(__nt), __end));
                6
            }
            21 => {
                // BinTier<BinOpCompare, ExprAddSub> = BinTier<BinOpCompare, ExprAddSub>, BinOpCompare, ExprAddSub => ActionFn(32);
                let __sym2 = __pop_NtExprAddSub(__symbols);
                let __sym1 = __pop_NtBinOpCompare(__symbols);
                let __sym0 = __pop_NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action32::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(__nt), __end));
                7
            }
            22 => {
                // BinTier<BinOpCompare, ExprAddSub> = ExprAddSub => ActionFn(33);
                let __sym0 = __pop_NtExprAddSub(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(__nt), __end));
                7
            }
            23 => {
                // BinTier<BinOpMulDiv, Term> = BinTier<BinOpMulDiv, Term>, BinOpMulDiv, Term => ActionFn(28);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_NtBinOpMulDiv(__symbols);
                let __sym0 = __pop_NtBinTier_3cBinOpMulDiv_2c_20Term_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action28::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpMulDiv_2c_20Term_3e(__nt), __end));
                8
            }
            24 => {
                // BinTier<BinOpMulDiv, Term> = Term => ActionFn(29);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpMulDiv_2c_20Term_3e(__nt), __end));
                8
            }
            25 => {
                // Comma<WireDecl> = WireDecl => ActionFn(46);
                let __sym0 = __pop_NtWireDecl(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action46::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cWireDecl_3e(__nt), __end));
                9
            }
            26 => {
                // Comma<WireDecl> =  => ActionFn(47);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action47::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cWireDecl_3e(__nt), __end));
                9
            }
            27 => {
                // Comma<WireDecl> = (<WireDecl> ",")+, WireDecl => ActionFn(48);
                let __sym1 = __pop_NtWireDecl(__symbols);
                let __sym0 = __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action48::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cWireDecl_3e(__nt), __end));
                9
            }
            28 => {
                // Comma<WireDecl> = (<WireDecl> ",")+ => ActionFn(49);
                let __sym0 = __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action49::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cWireDecl_3e(__nt), __end));
                9
            }
            29 => {
                // Expr = ExprCompare => ActionFn(27);
                let __sym0 = __pop_NtExprCompare(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                10
            }
            30 => {
                // ExprAddSub = BinTier<BinOpAddSub, ExprMulDiv> => ActionFn(19);
                let __sym0 = __pop_NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprAddSub(__nt), __end));
                11
            }
            31 => {
                // ExprCompare = BinTier<BinOpCompare, ExprAddSub> => ActionFn(18);
                let __sym0 = __pop_NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprCompare(__nt), __end));
                12
            }
            32 => {
                // ExprMulDiv = BinTier<BinOpMulDiv, Term> => ActionFn(20);
                let __sym0 = __pop_NtBinTier_3cBinOpMulDiv_2c_20Term_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprMulDiv(__nt), __end));
                13
            }
            33 => {
                // Term = DEC => ActionFn(23);
                let __sym0 = __pop_TermDEC(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                14
            }
            34 => {
                // Term = HEX => ActionFn(24);
                let __sym0 = __pop_TermHEX(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                14
            }
            35 => {
                // Term = BIN => ActionFn(25);
                let __sym0 = __pop_TermBIN(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                14
            }
            36 => {
                // Term = "(", Expr, ")" => ActionFn(26);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action26::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                14
            }
            37 => {
                // UnOp = "-" => ActionFn(21);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtUnOp(__nt), __end));
                15
            }
            38 => {
                // UnOp = "~" => ActionFn(22);
                let __sym0 = __pop_Term_22_7e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtUnOp(__nt), __end));
                15
            }
            39 => {
                // WireDecl = ID, ":", DEC => ActionFn(3);
                let __sym2 = __pop_TermDEC(__symbols);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_TermID(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtWireDecl(__nt), __end));
                16
            }
            40 => {
                // WireDecl? = WireDecl => ActionFn(35);
                let __sym0 = __pop_NtWireDecl(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtWireDecl_3f(__nt), __end));
                17
            }
            41 => {
                // WireDecl? =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtWireDecl_3f(__nt), __end));
                17
            }
            42 => {
                // WireDecls = "wire", Comma<WireDecl>, ";" => ActionFn(4);
                let __sym2 = __pop_Term_22_3b_22(__symbols);
                let __sym1 = __pop_NtComma_3cWireDecl_3e(__symbols);
                let __sym0 = __pop_Term_22wire_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtWireDecls(__nt), __end));
                18
            }
            43 => {
                // __Expr = Expr => ActionFn(2);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            44 => {
                // __Term = Term => ActionFn(1);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Term(__nt), __end));
                20
            }
            45 => {
                // __WireDecls = WireDecls => ActionFn(0);
                let __sym0 = __pop_NtWireDecls(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____WireDecls(__nt), __end));
                21
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 22 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_21_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_21_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_26_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_26_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22wire_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22wire_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermBIN<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermBIN(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermDEC<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermDEC(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermHEX<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermHEX(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermID<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermID(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermOCT<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermOCT(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, WireDecl, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinOpAddSub<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinOpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinOpAddSub(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinOpCompare<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinOpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinOpCompare(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinOpMulDiv<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinOpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinOpMulDiv(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinTier_3cBinOpMulDiv_2c_20Term_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinTier_3cBinOpMulDiv_2c_20Term_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cWireDecl_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cWireDecl_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprAddSub<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprAddSub(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprCompare<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprCompare(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprMulDiv<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprMulDiv(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtUnOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, UnOpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtUnOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtWireDecl<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, WireDecl, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtWireDecl(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtWireDecl_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtWireDecl_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtWireDecls<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtWireDecls(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Term<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____WireDecls<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____WireDecls(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Expr::parse_Expr;

mod __parse__Term {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Expr, WireDecl, WireWidth, WireValue, BinOpCode, UnOpCode};
    use lalrpop_util::ErrorRecovery;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_21_3d_22(&'input str),
        Term_22_26_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3c_22(&'input str),
        Term_22_3c_3d_22(&'input str),
        Term_22_3d_3d_22(&'input str),
        Term_22_3e_22(&'input str),
        Term_22_3e_3d_22(&'input str),
        Term_22_5e_22(&'input str),
        Term_22wire_22(&'input str),
        Term_22_7c_22(&'input str),
        Term_22_7e_22(&'input str),
        TermBIN(&'input str),
        TermDEC(&'input str),
        TermHEX(&'input str),
        TermID(&'input str),
        TermOCT(&'input str),
        Nt_28_3cWireDecl_3e_20_22_2c_22_29(WireDecl),
        Nt_28_3cWireDecl_3e_20_22_2c_22_29_2a(::std::vec::Vec<WireDecl>),
        Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(::std::vec::Vec<WireDecl>),
        NtBinOpAddSub(BinOpCode),
        NtBinOpCompare(BinOpCode),
        NtBinOpMulDiv(BinOpCode),
        NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(Box<Expr>),
        NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(Box<Expr>),
        NtBinTier_3cBinOpMulDiv_2c_20Term_3e(Box<Expr>),
        NtComma_3cWireDecl_3e(Vec<WireDecl>),
        NtExpr(Box<Expr>),
        NtExprAddSub(Box<Expr>),
        NtExprCompare(Box<Expr>),
        NtExprMulDiv(Box<Expr>),
        NtTerm(Box<Expr>),
        NtUnOp(UnOpCode),
        NtWireDecl(WireDecl),
        NtWireDecl_3f(::std::option::Option<WireDecl>),
        NtWireDecls(Vec<WireDecl>),
        Nt____Expr(Box<Expr>),
        Nt____Term(Box<Expr>),
        Nt____WireDecls(Vec<WireDecl>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 5, 6, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 17, 18, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        -30, 0, 0, -30, 0, 20, 0, 21, 0, 0, 0, -30, -30, -30, -30, -30, 0, 0, 22, 0, 0, 0, 0, 0, 0,
        // State 7
        24, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 25, 26, 27, 28, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        -32, 31, 0, -32, 32, -32, 0, -32, 33, 0, 0, -32, -32, -32, -32, -32, 34, 0, -32, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        -22, 0, 0, -22, 0, 0, 0, 0, 0, 0, 0, -22, -22, -22, -22, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        -20, 0, 0, -20, 0, -20, 0, -20, 0, 0, 0, -20, -20, -20, -20, -20, 0, 0, -20, 0, 0, 0, 0, 0, 0,
        // State 13
        -24, -24, 0, -24, -24, -24, 0, -24, -24, 0, 0, -24, -24, -24, -24, -24, -24, 0, -24, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 17, 18, 0, 0,
        // State 15
        -35, -35, 0, -35, -35, -35, 0, -35, -35, 0, 0, -35, -35, -35, -35, -35, -35, 0, -35, 0, 0, 0, 0, 0, 0,
        // State 16
        -33, -33, 0, -33, -33, -33, 0, -33, -33, 0, 0, -33, -33, -33, -33, -33, -33, 0, -33, 0, 0, 0, 0, 0, 0,
        // State 17
        -34, -34, 0, -34, -34, -34, 0, -34, -34, 0, 0, -34, -34, -34, -34, -34, -34, 0, -34, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 17, 18, 0, 0,
        // State 19
        0, 0, -6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -6, -6, -6, 0, 0,
        // State 20
        0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, -7, -7, 0, 0,
        // State 21
        0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, -8, -8, 0, 0,
        // State 22
        0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 17, 18, 0, 0,
        // State 23
        0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, -10, -10, 0, 0,
        // State 24
        0, 0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, -13, -13, 0, 0,
        // State 25
        0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -11, -11, -11, 0, 0,
        // State 26
        0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, -9, -9, 0, 0,
        // State 27
        0, 0, -14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, -14, -14, 0, 0,
        // State 28
        0, 0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, -12, -12, 0, 0,
        // State 29
        0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 17, 18, 0, 0,
        // State 30
        0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, -18, -18, 0, 0,
        // State 31
        0, 0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, -15, -15, 0, 0,
        // State 32
        0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, -16, -16, 0, 0,
        // State 33
        0, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, -17, -17, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        -19, 0, 0, -19, 0, -19, 0, -19, 0, 0, 0, -19, -19, -19, -19, -19, 0, 0, -19, 0, 0, 0, 0, 0, 0,
        // State 37
        -21, 0, 0, -21, 0, 0, 0, 0, 0, 0, 0, -21, -21, -21, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        -23, -23, 0, -23, -23, -23, 0, -23, -23, 0, 0, -23, -23, -23, -23, -23, -23, 0, -23, 0, 0, 0, 0, 0, 0,
        // State 39
        -36, -36, 0, -36, -36, -36, 0, -36, -36, 0, 0, -36, -36, -36, -36, -36, -36, 0, -36, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -44,
        0,
        -35,
        -33,
        -34,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        -36,
        0,
        0,
        0,
        0,
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 7, 8, 9, 0, 10, 11, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 7, 8, 9, 0, 36, 11, 12, 13, 14, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 37, 14, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 7, 0, 9, 0, 0, 38, 0, 13, 14, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""!=""###,
            r###""&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""^""###,
            r###""wire""###,
            r###""|""###,
            r###""~""###,
            r###"BIN"###,
            r###"DEC"###,
            r###"HEX"###,
            r###"ID"###,
            r###"OCT"###,
        ];
        __ACTION[(__state * 25)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Term<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
    ) -> Result<Box<Expr>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                (4, _) if true => 4,
                (5, _) if true => 5,
                (6, _) if true => 6,
                (7, _) if true => 7,
                (8, _) if true => 8,
                (9, _) if true => 9,
                (10, _) if true => 10,
                (11, _) if true => 11,
                (12, _) if true => 12,
                (13, _) if true => 13,
                (14, _) if true => 14,
                (15, _) if true => 15,
                (16, _) if true => 16,
                (24, _) if true => 17,
                (17, _) if true => 18,
                (18, _) if true => 19,
                (21, _) if true => 20,
                (20, _) if true => 21,
                (23, _) if true => 22,
                (19, _) if true => 23,
                (22, _) if true => 24,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 25 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_21_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_26_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_2f_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_3b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_3c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_3c_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22_3d_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_3e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22_3e_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22_5e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (24, __tok0) => __Symbol::Term_22wire_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22_7c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22_7e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            (21, __tok0) => __Symbol::TermBIN((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            (20, __tok0) => __Symbol::TermDEC((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            (23, __tok0) => __Symbol::TermHEX((__tok0)),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            (19, __tok0) => __Symbol::TermID((__tok0)),
                            _ => unreachable!(),
                        },
                        24 => match __lookahead.1 {
                            (22, __tok0) => __Symbol::TermOCT((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Box<Expr>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<WireDecl> ",") = WireDecl, "," => ActionFn(39);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtWireDecl(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<WireDecl> ",")* =  => ActionFn(37);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action37::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<WireDecl> ",")* = (<WireDecl> ",")+ => ActionFn(38);
                let __sym0 = __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<WireDecl> ",")+ = WireDecl, "," => ActionFn(42);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtWireDecl(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action42::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<WireDecl> ",")+ = (<WireDecl> ",")+, WireDecl, "," => ActionFn(43);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtWireDecl(__symbols);
                let __sym0 = __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action43::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // BinOpAddSub = "+" => ActionFn(5);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpAddSub(__nt), __end));
                3
            }
            7 => {
                // BinOpAddSub = "-" => ActionFn(6);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpAddSub(__nt), __end));
                3
            }
            8 => {
                // BinOpAddSub = "|" => ActionFn(7);
                let __sym0 = __pop_Term_22_7c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpAddSub(__nt), __end));
                3
            }
            9 => {
                // BinOpCompare = "==" => ActionFn(12);
                let __sym0 = __pop_Term_22_3d_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            10 => {
                // BinOpCompare = "!=" => ActionFn(13);
                let __sym0 = __pop_Term_22_21_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            11 => {
                // BinOpCompare = "<=" => ActionFn(14);
                let __sym0 = __pop_Term_22_3c_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            12 => {
                // BinOpCompare = ">=" => ActionFn(15);
                let __sym0 = __pop_Term_22_3e_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            13 => {
                // BinOpCompare = "<" => ActionFn(16);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            14 => {
                // BinOpCompare = ">" => ActionFn(17);
                let __sym0 = __pop_Term_22_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            15 => {
                // BinOpMulDiv = "*" => ActionFn(8);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpMulDiv(__nt), __end));
                5
            }
            16 => {
                // BinOpMulDiv = "/" => ActionFn(9);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpMulDiv(__nt), __end));
                5
            }
            17 => {
                // BinOpMulDiv = "^" => ActionFn(10);
                let __sym0 = __pop_Term_22_5e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpMulDiv(__nt), __end));
                5
            }
            18 => {
                // BinOpMulDiv = "&" => ActionFn(11);
                let __sym0 = __pop_Term_22_26_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpMulDiv(__nt), __end));
                5
            }
            19 => {
                // BinTier<BinOpAddSub, ExprMulDiv> = BinTier<BinOpAddSub, ExprMulDiv>, BinOpAddSub, ExprMulDiv => ActionFn(30);
                let __sym2 = __pop_NtExprMulDiv(__symbols);
                let __sym1 = __pop_NtBinOpAddSub(__symbols);
                let __sym0 = __pop_NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(__nt), __end));
                6
            }
            20 => {
                // BinTier<BinOpAddSub, ExprMulDiv> = ExprMulDiv => ActionFn(31);
                let __sym0 = __pop_NtExprMulDiv(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(__nt), __end));
                6
            }
            21 => {
                // BinTier<BinOpCompare, ExprAddSub> = BinTier<BinOpCompare, ExprAddSub>, BinOpCompare, ExprAddSub => ActionFn(32);
                let __sym2 = __pop_NtExprAddSub(__symbols);
                let __sym1 = __pop_NtBinOpCompare(__symbols);
                let __sym0 = __pop_NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action32::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(__nt), __end));
                7
            }
            22 => {
                // BinTier<BinOpCompare, ExprAddSub> = ExprAddSub => ActionFn(33);
                let __sym0 = __pop_NtExprAddSub(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(__nt), __end));
                7
            }
            23 => {
                // BinTier<BinOpMulDiv, Term> = BinTier<BinOpMulDiv, Term>, BinOpMulDiv, Term => ActionFn(28);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_NtBinOpMulDiv(__symbols);
                let __sym0 = __pop_NtBinTier_3cBinOpMulDiv_2c_20Term_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action28::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpMulDiv_2c_20Term_3e(__nt), __end));
                8
            }
            24 => {
                // BinTier<BinOpMulDiv, Term> = Term => ActionFn(29);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpMulDiv_2c_20Term_3e(__nt), __end));
                8
            }
            25 => {
                // Comma<WireDecl> = WireDecl => ActionFn(46);
                let __sym0 = __pop_NtWireDecl(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action46::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cWireDecl_3e(__nt), __end));
                9
            }
            26 => {
                // Comma<WireDecl> =  => ActionFn(47);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action47::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cWireDecl_3e(__nt), __end));
                9
            }
            27 => {
                // Comma<WireDecl> = (<WireDecl> ",")+, WireDecl => ActionFn(48);
                let __sym1 = __pop_NtWireDecl(__symbols);
                let __sym0 = __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action48::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cWireDecl_3e(__nt), __end));
                9
            }
            28 => {
                // Comma<WireDecl> = (<WireDecl> ",")+ => ActionFn(49);
                let __sym0 = __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action49::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cWireDecl_3e(__nt), __end));
                9
            }
            29 => {
                // Expr = ExprCompare => ActionFn(27);
                let __sym0 = __pop_NtExprCompare(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                10
            }
            30 => {
                // ExprAddSub = BinTier<BinOpAddSub, ExprMulDiv> => ActionFn(19);
                let __sym0 = __pop_NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprAddSub(__nt), __end));
                11
            }
            31 => {
                // ExprCompare = BinTier<BinOpCompare, ExprAddSub> => ActionFn(18);
                let __sym0 = __pop_NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprCompare(__nt), __end));
                12
            }
            32 => {
                // ExprMulDiv = BinTier<BinOpMulDiv, Term> => ActionFn(20);
                let __sym0 = __pop_NtBinTier_3cBinOpMulDiv_2c_20Term_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprMulDiv(__nt), __end));
                13
            }
            33 => {
                // Term = DEC => ActionFn(23);
                let __sym0 = __pop_TermDEC(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                14
            }
            34 => {
                // Term = HEX => ActionFn(24);
                let __sym0 = __pop_TermHEX(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                14
            }
            35 => {
                // Term = BIN => ActionFn(25);
                let __sym0 = __pop_TermBIN(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                14
            }
            36 => {
                // Term = "(", Expr, ")" => ActionFn(26);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action26::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                14
            }
            37 => {
                // UnOp = "-" => ActionFn(21);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtUnOp(__nt), __end));
                15
            }
            38 => {
                // UnOp = "~" => ActionFn(22);
                let __sym0 = __pop_Term_22_7e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtUnOp(__nt), __end));
                15
            }
            39 => {
                // WireDecl = ID, ":", DEC => ActionFn(3);
                let __sym2 = __pop_TermDEC(__symbols);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_TermID(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtWireDecl(__nt), __end));
                16
            }
            40 => {
                // WireDecl? = WireDecl => ActionFn(35);
                let __sym0 = __pop_NtWireDecl(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtWireDecl_3f(__nt), __end));
                17
            }
            41 => {
                // WireDecl? =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtWireDecl_3f(__nt), __end));
                17
            }
            42 => {
                // WireDecls = "wire", Comma<WireDecl>, ";" => ActionFn(4);
                let __sym2 = __pop_Term_22_3b_22(__symbols);
                let __sym1 = __pop_NtComma_3cWireDecl_3e(__symbols);
                let __sym0 = __pop_Term_22wire_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtWireDecls(__nt), __end));
                18
            }
            43 => {
                // __Expr = Expr => ActionFn(2);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                19
            }
            44 => {
                // __Term = Term => ActionFn(1);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            45 => {
                // __WireDecls = WireDecls => ActionFn(0);
                let __sym0 = __pop_NtWireDecls(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____WireDecls(__nt), __end));
                21
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 22 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_21_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_21_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_26_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_26_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22wire_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22wire_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermBIN<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermBIN(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermDEC<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermDEC(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermHEX<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermHEX(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermID<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermID(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermOCT<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermOCT(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, WireDecl, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinOpAddSub<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinOpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinOpAddSub(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinOpCompare<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinOpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinOpCompare(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinOpMulDiv<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinOpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinOpMulDiv(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinTier_3cBinOpMulDiv_2c_20Term_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinTier_3cBinOpMulDiv_2c_20Term_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cWireDecl_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cWireDecl_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprAddSub<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprAddSub(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprCompare<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprCompare(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprMulDiv<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprMulDiv(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtUnOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, UnOpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtUnOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtWireDecl<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, WireDecl, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtWireDecl(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtWireDecl_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtWireDecl_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtWireDecls<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtWireDecls(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Term<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____WireDecls<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____WireDecls(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Term::parse_Term;

mod __parse__WireDecls {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use ast::{Expr, WireDecl, WireWidth, WireValue, BinOpCode, UnOpCode};
    use lalrpop_util::ErrorRecovery;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_21_3d_22(&'input str),
        Term_22_26_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_2f_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3c_22(&'input str),
        Term_22_3c_3d_22(&'input str),
        Term_22_3d_3d_22(&'input str),
        Term_22_3e_22(&'input str),
        Term_22_3e_3d_22(&'input str),
        Term_22_5e_22(&'input str),
        Term_22wire_22(&'input str),
        Term_22_7c_22(&'input str),
        Term_22_7e_22(&'input str),
        TermBIN(&'input str),
        TermDEC(&'input str),
        TermHEX(&'input str),
        TermID(&'input str),
        TermOCT(&'input str),
        Nt_28_3cWireDecl_3e_20_22_2c_22_29(WireDecl),
        Nt_28_3cWireDecl_3e_20_22_2c_22_29_2a(::std::vec::Vec<WireDecl>),
        Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(::std::vec::Vec<WireDecl>),
        NtBinOpAddSub(BinOpCode),
        NtBinOpCompare(BinOpCode),
        NtBinOpMulDiv(BinOpCode),
        NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(Box<Expr>),
        NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(Box<Expr>),
        NtBinTier_3cBinOpMulDiv_2c_20Term_3e(Box<Expr>),
        NtComma_3cWireDecl_3e(Vec<WireDecl>),
        NtExpr(Box<Expr>),
        NtExprAddSub(Box<Expr>),
        NtExprCompare(Box<Expr>),
        NtExprMulDiv(Box<Expr>),
        NtTerm(Box<Expr>),
        NtUnOp(UnOpCode),
        NtWireDecl(WireDecl),
        NtWireDecl_3f(::std::option::Option<WireDecl>),
        NtWireDecls(Vec<WireDecl>),
        Nt____Expr(Box<Expr>),
        Nt____Term(Box<Expr>),
        Nt____WireDecls(Vec<WireDecl>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 10, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 12, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 13, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -5, 0,
        // State 12
        0, 0, 0, 0, 0, 0, -39, 0, 0, 0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -45,
        0,
        0,
        0,
        0,
        0,
        0,
        -42,
        0,
        0,
        0,
        0,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 4, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 6, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 8, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""!=""###,
            r###""&""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""^""###,
            r###""wire""###,
            r###""|""###,
            r###""~""###,
            r###"BIN"###,
            r###"DEC"###,
            r###"HEX"###,
            r###"ID"###,
            r###"OCT"###,
        ];
        __ACTION[(__state * 25)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_WireDecls<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
    ) -> Result<Vec<WireDecl>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let mut __last_location = Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            __last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                (0, _) if true => 0,
                (1, _) if true => 1,
                (2, _) if true => 2,
                (3, _) if true => 3,
                (4, _) if true => 4,
                (5, _) if true => 5,
                (6, _) if true => 6,
                (7, _) if true => 7,
                (8, _) if true => 8,
                (9, _) if true => 9,
                (10, _) if true => 10,
                (11, _) if true => 11,
                (12, _) if true => 12,
                (13, _) if true => 13,
                (14, _) if true => 14,
                (15, _) if true => 15,
                (16, _) if true => 16,
                (24, _) if true => 17,
                (17, _) if true => 18,
                (18, _) if true => 19,
                (21, _) if true => 20,
                (20, _) if true => 21,
                (23, _) if true => 22,
                (19, _) if true => 23,
                (22, _) if true => 24,
                _ => {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error);
                }
            };
            '__inner: loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __ACTION[__state * 25 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Term_22_21_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_26_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            (3, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            (4, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            (5, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            (6, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            (7, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            (8, __tok0) => __Symbol::Term_22_2f_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            (9, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            (10, __tok0) => __Symbol::Term_22_3b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            (11, __tok0) => __Symbol::Term_22_3c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            (12, __tok0) => __Symbol::Term_22_3c_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            (13, __tok0) => __Symbol::Term_22_3d_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            (14, __tok0) => __Symbol::Term_22_3e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            (15, __tok0) => __Symbol::Term_22_3e_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            (16, __tok0) => __Symbol::Term_22_5e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            (24, __tok0) => __Symbol::Term_22wire_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            (17, __tok0) => __Symbol::Term_22_7c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            (18, __tok0) => __Symbol::Term_22_7e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            (21, __tok0) => __Symbol::TermBIN((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            (20, __tok0) => __Symbol::TermDEC((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            (23, __tok0) => __Symbol::TermHEX((__tok0)),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            (19, __tok0) => __Symbol::TermID((__tok0)),
                            _ => unreachable!(),
                        },
                        24 => match __lookahead.1 {
                            (22, __tok0) => __Symbol::TermOCT((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(errors, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: Some(__lookahead),
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
        loop {
            let __state = *__states.last().unwrap() as usize;
            let __action = __EOF_ACTION[__state];
            if __action < 0 {
                if let Some(r) = __reduce(errors, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: None,
                    expected: __expected_tokens(__state),
                };
                return Err(__error);
            }
        }
    }
    pub fn __reduce<
        'input,
        'err,
    >(
        errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<WireDecl>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<WireDecl> ",") = WireDecl, "," => ActionFn(39);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtWireDecl(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<WireDecl> ",")* =  => ActionFn(37);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action37::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<WireDecl> ",")* = (<WireDecl> ",")+ => ActionFn(38);
                let __sym0 = __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<WireDecl> ",")+ = WireDecl, "," => ActionFn(42);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtWireDecl(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action42::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<WireDecl> ",")+ = (<WireDecl> ",")+, WireDecl, "," => ActionFn(43);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtWireDecl(__symbols);
                let __sym0 = __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action43::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // BinOpAddSub = "+" => ActionFn(5);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpAddSub(__nt), __end));
                3
            }
            7 => {
                // BinOpAddSub = "-" => ActionFn(6);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpAddSub(__nt), __end));
                3
            }
            8 => {
                // BinOpAddSub = "|" => ActionFn(7);
                let __sym0 = __pop_Term_22_7c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpAddSub(__nt), __end));
                3
            }
            9 => {
                // BinOpCompare = "==" => ActionFn(12);
                let __sym0 = __pop_Term_22_3d_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            10 => {
                // BinOpCompare = "!=" => ActionFn(13);
                let __sym0 = __pop_Term_22_21_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            11 => {
                // BinOpCompare = "<=" => ActionFn(14);
                let __sym0 = __pop_Term_22_3c_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            12 => {
                // BinOpCompare = ">=" => ActionFn(15);
                let __sym0 = __pop_Term_22_3e_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            13 => {
                // BinOpCompare = "<" => ActionFn(16);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            14 => {
                // BinOpCompare = ">" => ActionFn(17);
                let __sym0 = __pop_Term_22_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpCompare(__nt), __end));
                4
            }
            15 => {
                // BinOpMulDiv = "*" => ActionFn(8);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpMulDiv(__nt), __end));
                5
            }
            16 => {
                // BinOpMulDiv = "/" => ActionFn(9);
                let __sym0 = __pop_Term_22_2f_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpMulDiv(__nt), __end));
                5
            }
            17 => {
                // BinOpMulDiv = "^" => ActionFn(10);
                let __sym0 = __pop_Term_22_5e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpMulDiv(__nt), __end));
                5
            }
            18 => {
                // BinOpMulDiv = "&" => ActionFn(11);
                let __sym0 = __pop_Term_22_26_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinOpMulDiv(__nt), __end));
                5
            }
            19 => {
                // BinTier<BinOpAddSub, ExprMulDiv> = BinTier<BinOpAddSub, ExprMulDiv>, BinOpAddSub, ExprMulDiv => ActionFn(30);
                let __sym2 = __pop_NtExprMulDiv(__symbols);
                let __sym1 = __pop_NtBinOpAddSub(__symbols);
                let __sym0 = __pop_NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action30::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(__nt), __end));
                6
            }
            20 => {
                // BinTier<BinOpAddSub, ExprMulDiv> = ExprMulDiv => ActionFn(31);
                let __sym0 = __pop_NtExprMulDiv(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(__nt), __end));
                6
            }
            21 => {
                // BinTier<BinOpCompare, ExprAddSub> = BinTier<BinOpCompare, ExprAddSub>, BinOpCompare, ExprAddSub => ActionFn(32);
                let __sym2 = __pop_NtExprAddSub(__symbols);
                let __sym1 = __pop_NtBinOpCompare(__symbols);
                let __sym0 = __pop_NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action32::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(__nt), __end));
                7
            }
            22 => {
                // BinTier<BinOpCompare, ExprAddSub> = ExprAddSub => ActionFn(33);
                let __sym0 = __pop_NtExprAddSub(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(__nt), __end));
                7
            }
            23 => {
                // BinTier<BinOpMulDiv, Term> = BinTier<BinOpMulDiv, Term>, BinOpMulDiv, Term => ActionFn(28);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_NtBinOpMulDiv(__symbols);
                let __sym0 = __pop_NtBinTier_3cBinOpMulDiv_2c_20Term_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action28::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpMulDiv_2c_20Term_3e(__nt), __end));
                8
            }
            24 => {
                // BinTier<BinOpMulDiv, Term> = Term => ActionFn(29);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBinTier_3cBinOpMulDiv_2c_20Term_3e(__nt), __end));
                8
            }
            25 => {
                // Comma<WireDecl> = WireDecl => ActionFn(46);
                let __sym0 = __pop_NtWireDecl(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action46::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cWireDecl_3e(__nt), __end));
                9
            }
            26 => {
                // Comma<WireDecl> =  => ActionFn(47);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action47::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cWireDecl_3e(__nt), __end));
                9
            }
            27 => {
                // Comma<WireDecl> = (<WireDecl> ",")+, WireDecl => ActionFn(48);
                let __sym1 = __pop_NtWireDecl(__symbols);
                let __sym0 = __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action48::<>(errors, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cWireDecl_3e(__nt), __end));
                9
            }
            28 => {
                // Comma<WireDecl> = (<WireDecl> ",")+ => ActionFn(49);
                let __sym0 = __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action49::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cWireDecl_3e(__nt), __end));
                9
            }
            29 => {
                // Expr = ExprCompare => ActionFn(27);
                let __sym0 = __pop_NtExprCompare(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                10
            }
            30 => {
                // ExprAddSub = BinTier<BinOpAddSub, ExprMulDiv> => ActionFn(19);
                let __sym0 = __pop_NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprAddSub(__nt), __end));
                11
            }
            31 => {
                // ExprCompare = BinTier<BinOpCompare, ExprAddSub> => ActionFn(18);
                let __sym0 = __pop_NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprCompare(__nt), __end));
                12
            }
            32 => {
                // ExprMulDiv = BinTier<BinOpMulDiv, Term> => ActionFn(20);
                let __sym0 = __pop_NtBinTier_3cBinOpMulDiv_2c_20Term_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprMulDiv(__nt), __end));
                13
            }
            33 => {
                // Term = DEC => ActionFn(23);
                let __sym0 = __pop_TermDEC(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                14
            }
            34 => {
                // Term = HEX => ActionFn(24);
                let __sym0 = __pop_TermHEX(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                14
            }
            35 => {
                // Term = BIN => ActionFn(25);
                let __sym0 = __pop_TermBIN(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                14
            }
            36 => {
                // Term = "(", Expr, ")" => ActionFn(26);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action26::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                14
            }
            37 => {
                // UnOp = "-" => ActionFn(21);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtUnOp(__nt), __end));
                15
            }
            38 => {
                // UnOp = "~" => ActionFn(22);
                let __sym0 = __pop_Term_22_7e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtUnOp(__nt), __end));
                15
            }
            39 => {
                // WireDecl = ID, ":", DEC => ActionFn(3);
                let __sym2 = __pop_TermDEC(__symbols);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_TermID(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action3::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtWireDecl(__nt), __end));
                16
            }
            40 => {
                // WireDecl? = WireDecl => ActionFn(35);
                let __sym0 = __pop_NtWireDecl(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtWireDecl_3f(__nt), __end));
                17
            }
            41 => {
                // WireDecl? =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(errors, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtWireDecl_3f(__nt), __end));
                17
            }
            42 => {
                // WireDecls = "wire", Comma<WireDecl>, ";" => ActionFn(4);
                let __sym2 = __pop_Term_22_3b_22(__symbols);
                let __sym1 = __pop_NtComma_3cWireDecl_3e(__symbols);
                let __sym0 = __pop_Term_22wire_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4::<>(errors, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtWireDecls(__nt), __end));
                18
            }
            43 => {
                // __Expr = Expr => ActionFn(2);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Expr(__nt), __end));
                19
            }
            44 => {
                // __Term = Term => ActionFn(1);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(errors, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Term(__nt), __end));
                20
            }
            45 => {
                // __WireDecls = WireDecls => ActionFn(0);
                let __sym0 = __pop_NtWireDecls(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(errors, input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 22 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_21_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_21_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_26_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_26_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2f_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2f_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22wire_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22wire_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermBIN<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermBIN(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermDEC<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermDEC(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermHEX<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermHEX(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermID<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermID(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_TermOCT<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::TermOCT(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, WireDecl, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cWireDecl_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinOpAddSub<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinOpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinOpAddSub(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinOpCompare<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinOpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinOpCompare(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinOpMulDiv<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinOpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinOpMulDiv(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinTier_3cBinOpAddSub_2c_20ExprMulDiv_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinTier_3cBinOpCompare_2c_20ExprAddSub_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBinTier_3cBinOpMulDiv_2c_20Term_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBinTier_3cBinOpMulDiv_2c_20Term_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cWireDecl_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cWireDecl_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprAddSub<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprAddSub(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprCompare<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprCompare(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprMulDiv<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprMulDiv(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtUnOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, UnOpCode, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtUnOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtWireDecl<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, WireDecl, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtWireDecl(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtWireDecl_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtWireDecl_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtWireDecls<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtWireDecls(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Expr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Expr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Term<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Term(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____WireDecls<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<WireDecl>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____WireDecls(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__WireDecls::parse_WireDecls;
mod __intern_token {
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:!=)",
                "^(?u:\\&)",
                "^(?u:\\()",
                "^(?u:\\))",
                "^(?u:\\*)",
                "^(?u:\\+)",
                "^(?u:,)",
                "^(?u:\\-)",
                "^(?u:/)",
                "^(?u::)",
                "^(?u:;)",
                "^(?u:<)",
                "^(?u:<=)",
                "^(?u:==)",
                "^(?u:>)",
                "^(?u:>=)",
                "^(?u:\\^)",
                "^(?u:\\|)",
                "^(?u:\\~)",
                "^(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࢠ-ࢴࣣ-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-ૹଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಁ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഁ-ഃഅ-ഌഎ-ഐഒ-ഺഽ-ൄെ-ൈൊ-ൎൗ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽ᳐-᳔᳒-ᳶ᳸-᳹ᴀ-᷵᷼-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-꣄꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌰-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑫀-𑫸𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞠀-𞣄𞣐-𞣖𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀󠄀-󠇯])+",
                "^(?u:[0-9])+",
                "^(?u:0b)(?u:[0-1])+",
                "^(?u:0o)(?u:[0-7])",
                "^(?u:0x)(?u:[0-9A-Fa-f])",
                "^(?u:wire)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:!=)").unwrap(),
                __regex::Regex::new("^(?u:\\&)").unwrap(),
                __regex::Regex::new("^(?u:\\()").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
                __regex::Regex::new("^(?u:\\*)").unwrap(),
                __regex::Regex::new("^(?u:\\+)").unwrap(),
                __regex::Regex::new("^(?u:,)").unwrap(),
                __regex::Regex::new("^(?u:\\-)").unwrap(),
                __regex::Regex::new("^(?u:/)").unwrap(),
                __regex::Regex::new("^(?u::)").unwrap(),
                __regex::Regex::new("^(?u:;)").unwrap(),
                __regex::Regex::new("^(?u:<)").unwrap(),
                __regex::Regex::new("^(?u:<=)").unwrap(),
                __regex::Regex::new("^(?u:==)").unwrap(),
                __regex::Regex::new("^(?u:>)").unwrap(),
                __regex::Regex::new("^(?u:>=)").unwrap(),
                __regex::Regex::new("^(?u:\\^)").unwrap(),
                __regex::Regex::new("^(?u:\\|)").unwrap(),
                __regex::Regex::new("^(?u:\\~)").unwrap(),
                __regex::Regex::new("^(?u:[0-9A-Z_-_a-zª-ªµ-µº-ºÀ-ÖØ-öø-ˁˆ-ˑˠ-ˤˬ-ˬˮ-ˮ̀-ʹͶ-ͷͺ-ͽͿ-ͿΆ-ΆΈ-ΊΌ-ΌΎ-ΡΣ-ϵϷ-ҁ҃-ԯԱ-Ֆՙ-ՙա-և֑-ֽֿ-ֿׁ-ׂׄ-ׇׅ-ׇא-תװ-ײؐ-ؚؠ-٩ٮ-ۓە-ۜ۟-۪ۨ-ۼۿ-ۿܐ-݊ݍ-ޱ߀-ߵߺ-ߺࠀ-࠭ࡀ-࡛ࢠ-ࢴࣣ-ॣ०-९ॱ-ঃঅ-ঌএ-ঐও-নপ-রল-লশ-হ়-ৄে-ৈো-ৎৗ-ৗড়-ঢ়য়-ৣ০-ৱਁ-ਃਅ-ਊਏ-ਐਓ-ਨਪ-ਰਲ-ਲ਼ਵ-ਸ਼ਸ-ਹ਼-਼ਾ-ੂੇ-ੈੋ-੍ੑ-ੑਖ਼-ੜਫ਼-ਫ਼੦-ੵઁ-ઃઅ-ઍએ-ઑઓ-નપ-રલ-ળવ-હ઼-ૅે-ૉો-્ૐ-ૐૠ-ૣ૦-૯ૹ-ૹଁ-ଃଅ-ଌଏ-ଐଓ-ନପ-ରଲ-ଳଵ-ହ଼-ୄେ-ୈୋ-୍ୖ-ୗଡ଼-ଢ଼ୟ-ୣ୦-୯ୱ-ୱஂ-ஃஅ-ஊஎ-ஐஒ-கங-சஜ-ஜஞ-டண-தந-பம-ஹா-ூெ-ைொ-்ௐ-ௐௗ-ௗ௦-௯ఀ-ఃఅ-ఌఎ-ఐఒ-నప-హఽ-ౄె-ైొ-్ౕ-ౖౘ-ౚౠ-ౣ౦-౯ಁ-ಃಅ-ಌಎ-ಐಒ-ನಪ-ಳವ-ಹ಼-ೄೆ-ೈೊ-್ೕ-ೖೞ-ೞೠ-ೣ೦-೯ೱ-ೲഁ-ഃഅ-ഌഎ-ഐഒ-ഺഽ-ൄെ-ൈൊ-ൎൗ-ൗൟ-ൣ൦-൯ൺ-ൿං-ඃඅ-ඖක-නඳ-රල-ලව-ෆ්-්ා-ුූ-ූෘ-ෟ෦-෯ෲ-ෳก-ฺเ-๎๐-๙ກ-ຂຄ-ຄງ-ຈຊ-ຊຍ-ຍດ-ທນ-ຟມ-ຣລ-ລວ-ວສ-ຫອ-ູົ-ຽເ-ໄໆ-ໆ່-ໍ໐-໙ໜ-ໟༀ-ༀ༘-༙༠-༩༵-༵༷-༹༷-༹༾-ཇཉ-ཬཱ-྄྆-ྗྙ-ྼ࿆-࿆က-၉ၐ-ႝႠ-ჅჇ-ჇჍ-Ⴭა-ჺჼ-ቈቊ-ቍቐ-ቖቘ-ቘቚ-ቝበ-ኈኊ-ኍነ-ኰኲ-ኵኸ-ኾዀ-ዀዂ-ዅወ-ዖዘ-ጐጒ-ጕጘ-ፚ፝-፟ᎀ-ᎏᎠ-Ᏽᏸ-ᏽᐁ-ᙬᙯ-ᙿᚁ-ᚚᚠ-ᛪᛮ-ᛸᜀ-ᜌᜎ-᜔ᜠ-᜴ᝀ-ᝓᝠ-ᝬᝮ-ᝰᝲ-ᝳក-៓ៗ-ៗៜ-៝០-៩᠋-᠍᠐-᠙ᠠ-ᡷᢀ-ᢪᢰ-ᣵᤀ-ᤞᤠ-ᤫᤰ-᤻᥆-ᥭᥰ-ᥴᦀ-ᦫᦰ-ᧉ᧐-᧙ᨀ-ᨛᨠ-ᩞ᩠-᩿᩼-᪉᪐-᪙ᪧ-ᪧ᪰-᪾ᬀ-ᭋ᭐-᭙᭫-᭳ᮀ-᯳ᰀ-᰷᱀-᱉ᱍ-ᱽ᳐-᳔᳒-ᳶ᳸-᳹ᴀ-᷵᷼-ἕἘ-Ἕἠ-ὅὈ-Ὅὐ-ὗὙ-ὙὛ-ὛὝ-ὝὟ-ώᾀ-ᾴᾶ-ᾼι-ιῂ-ῄῆ-ῌῐ-ΐῖ-Ίῠ-Ῥῲ-ῴῶ-ῼ\u{200c}-\u{200d}‿-⁀⁔-⁔ⁱ-ⁱⁿ-ⁿₐ-ₜ⃐-⃰ℂ-ℂℇ-ℇℊ-ℓℕ-ℕℙ-ℝℤ-ℤΩ-Ωℨ-ℨK-ℭℯ-ℹℼ-ℿⅅ-ⅉⅎ-ⅎⅠ-ↈⒶ-ⓩⰀ-Ⱞⰰ-ⱞⱠ-ⳤⳫ-ⳳⴀ-ⴥⴧ-ⴧⴭ-ⴭⴰ-ⵧⵯ-ⵯ⵿-ⶖⶠ-ⶦⶨ-ⶮⶰ-ⶶⶸ-ⶾⷀ-ⷆⷈ-ⷎⷐ-ⷖⷘ-ⷞⷠ-ⷿⸯ-ⸯ々-〇〡-〯〱-〵〸-〼ぁ-ゖ゙-゚ゝ-ゟァ-ヺー-ヿㄅ-ㄭㄱ-ㆎㆠ-ㆺㇰ-ㇿ㐀-䶵一-鿕ꀀ-ꒌꓐ-ꓽꔀ-ꘌꘐ-ꘫꙀ-꙲ꙴ-꙽ꙿ-꛱ꜗ-ꜟꜢ-ꞈꞋ-ꞭꞰ-ꞷꟷ-ꠧꡀ-ꡳꢀ-꣄꣐-꣙꣠-ꣷꣻ-ꣻꣽ-ꣽ꤀-꤭ꤰ-꥓ꥠ-ꥼꦀ-꧀ꧏ-꧙ꧠ-ꧾꨀ-ꨶꩀ-ꩍ꩐-꩙ꩠ-ꩶꩺ-ꫂꫛ-ꫝꫠ-ꫯꫲ-꫶ꬁ-ꬆꬉ-ꬎꬑ-ꬖꬠ-ꬦꬨ-ꬮꬰ-ꭚꭜ-ꭥꭰ-ꯪ꯬-꯭꯰-꯹가-힣ힰ-ퟆퟋ-ퟻ豈-舘並-龎ﬀ-ﬆﬓ-ﬗיִ-ﬨשׁ-זּטּ-לּמּ-מּנּ-סּףּ-פּצּ-ﮱﯓ-ﴽﵐ-ﶏﶒ-ﷇﷰ-ﷻ︀-️︠-︯︳-︴﹍-﹏ﹰ-ﹴﹶ-ﻼ０-９Ａ-Ｚ＿-＿ａ-ｚｦ-ﾾￂ-ￇￊ-ￏￒ-ￗￚ-ￜ𐀀-𐀋𐀍-𐀦𐀨-𐀺𐀼-𐀽𐀿-𐁍𐁐-𐁝𐂀-𐃺𐅀-𐅴𐇽-𐇽𐊀-𐊜𐊠-𐋐𐋠-𐋠𐌀-𐌟𐌰-𐍊𐍐-𐍺𐎀-𐎝𐎠-𐏃𐏈-𐏏𐏑-𐏕𐐀-𐒝𐒠-𐒩𐔀-𐔧𐔰-𐕣𐘀-𐜶𐝀-𐝕𐝠-𐝧𐠀-𐠅𐠈-𐠈𐠊-𐠵𐠷-𐠸𐠼-𐠼𐠿-𐡕𐡠-𐡶𐢀-𐢞𐣠-𐣲𐣴-𐣵𐤀-𐤕𐤠-𐤹𐦀-𐦷𐦾-𐦿𐨀-𐨃𐨅-𐨆𐨌-𐨓𐨕-𐨗𐨙-𐨳𐨸-𐨿𐨺-𐨿𐩠-𐩼𐪀-𐪜𐫀-𐫇𐫉-𐫦𐬀-𐬵𐭀-𐭕𐭠-𐭲𐮀-𐮑𐰀-𐱈𐲀-𐲲𐳀-𐳲𑀀-𑁆𑁦-𑁯𑁿-𑂺𑃐-𑃨𑃰-𑃹𑄀-𑄴𑄶-𑄿𑅐-𑅳𑅶-𑅶𑆀-𑇄𑇊-𑇌𑇐-𑇚𑇜-𑇜𑈀-𑈑𑈓-𑈷𑊀-𑊆𑊈-𑊈𑊊-𑊍𑊏-𑊝𑊟-𑊨𑊰-𑋪𑋰-𑋹𑌀-𑌃𑌅-𑌌𑌏-𑌐𑌓-𑌨𑌪-𑌰𑌲-𑌳𑌵-𑌹𑌼-𑍄𑍇-𑍈𑍋-𑍍𑍐-𑍐𑍗-𑍗𑍝-𑍣𑍦-𑍬𑍰-𑍴𑒀-𑓅𑓇-𑓇𑓐-𑓙𑖀-𑖵𑖸-𑗀𑗘-𑗝𑘀-𑙀𑙄-𑙄𑙐-𑙙𑚀-𑚷𑛀-𑛉𑜀-𑜙𑜝-𑜫𑜰-𑜹𑢠-𑣩𑣿-𑣿𑫀-𑫸𒀀-𒎙𒐀-𒑮𒒀-𒕃𓀀-𓐮𔐀-𔙆𖠀-𖨸𖩀-𖩞𖩠-𖩩𖫐-𖫭𖫰-𖫴𖬀-𖬶𖭀-𖭃𖭐-𖭙𖭣-𖭷𖭽-𖮏𖼀-𖽄𖽐-𖽾𖾏-𖾟𛀀-𛀁𛰀-𛱪𛱰-𛱼𛲀-𛲈𛲐-𛲙𛲝-𛲞𝅥-𝅩𝅭-𝅲𝅻-𝆂𝆅-𝆋𝆪-𝆭𝉂-𝉄𝐀-𝑔𝑖-𝒜𝒞-𝒟𝒢-𝒢𝒥-𝒦𝒩-𝒬𝒮-𝒹𝒻-𝒻𝒽-𝓃𝓅-𝔅𝔇-𝔊𝔍-𝔔𝔖-𝔜𝔞-𝔹𝔻-𝔾𝕀-𝕄𝕆-𝕆𝕊-𝕐𝕒-𝚥𝚨-𝛀𝛂-𝛚𝛜-𝛺𝛼-𝜔𝜖-𝜴𝜶-𝝎𝝐-𝝮𝝰-𝞈𝞊-𝞨𝞪-𝟂𝟄-𝟋𝟎-𝟿𝨀-𝨶𝨻-𝩬𝩵-𝩵𝪄-𝪄𝪛-𝪟𝪡-𝪯𞠀-𞣄𞣐-𞣖𞸀-𞸃𞸅-𞸟𞸡-𞸢𞸤-𞸤𞸧-𞸧𞸩-𞸲𞸴-𞸷𞸹-𞸹𞸻-𞸻𞹂-𞹂𞹇-𞹇𞹉-𞹉𞹋-𞹋𞹍-𞹏𞹑-𞹒𞹔-𞹔𞹗-𞹗𞹙-𞹙𞹛-𞹛𞹝-𞹝𞹟-𞹟𞹡-𞹢𞹤-𞹤𞹧-𞹪𞹬-𞹲𞹴-𞹷𞹹-𞹼𞹾-𞹾𞺀-𞺉𞺋-𞺛𞺡-𞺣𞺥-𞺩𞺫-𞺻🄰-🅉🅐-🅩🅰-🆉𠀀-𪛖𪜀-𫜴𫝀-𫠝𫠠-𬺡丽-𪘀󠄀-󠇯])+").unwrap(),
                __regex::Regex::new("^(?u:[0-9])+").unwrap(),
                __regex::Regex::new("^(?u:0b)(?u:[0-1])+").unwrap(),
                __regex::Regex::new("^(?u:0o)(?u:[0-7])").unwrap(),
                __regex::Regex::new("^(?u:0x)(?u:[0-9A-Fa-f])").unwrap(),
                __regex::Regex::new("^(?u:wire)").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
        type Item = Result<(usize, (usize, &'input str), usize), __lalrpop_util::ParseError<usize,(usize, &'input str),()>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 25 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, (__index, __result), __end_offset)))
                }
            }
        }
    }
}

#[allow(unused_variables)]
pub fn __action0<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Vec<WireDecl>, usize),
) -> Vec<WireDecl>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, name, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, w, _): (usize, &'input str, usize),
) -> WireDecl
{
    match WireWidth::from_str(w) {
        Ok(width) => WireDecl { name: String::from(name), width: width },
        Err(_) => {
            panic!() /* FIXME */
        }
    }
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, c, _): (usize, Vec<WireDecl>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Vec<WireDecl>
{
    c
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpCode
{
    BinOpCode::Add
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpCode
{
    BinOpCode::Sub
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpCode
{
    BinOpCode::Or
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpCode
{
    BinOpCode::Mul
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpCode
{
    BinOpCode::Div
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpCode
{
    BinOpCode::Xor
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpCode
{
    BinOpCode::And
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpCode
{
    BinOpCode::Equal
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpCode
{
    BinOpCode::NotEqual
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpCode
{
    BinOpCode::LessEqual
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpCode
{
    BinOpCode::GreaterEqual
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpCode
{
    BinOpCode::Less
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinOpCode
{
    BinOpCode::Greater
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action20<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action21<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> UnOpCode
{
    UnOpCode::Negate
}

#[allow(unused_variables)]
pub fn __action22<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> UnOpCode
{
    UnOpCode::Complement
}

#[allow(unused_variables)]
pub fn __action23<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    Box::new(Expr::Constant(WireValue::from_str(__0).unwrap(), 0))
}

#[allow(unused_variables)]
pub fn __action24<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    Box::new(Expr::Constant(WireValue::from_str_radix(&__0[2..], 16).unwrap(), 0))
}

#[allow(unused_variables)]
pub fn __action25<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    Box::new(Expr::Constant(WireValue::from_str_radix(&__0[2..], 2).unwrap(), __0.len() - 2))
}

#[allow(unused_variables)]
pub fn __action26<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, e, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    e
}

#[allow(unused_variables)]
pub fn __action27<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action28<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, l, _): (usize, Box<Expr>, usize),
    (_, o, _): (usize, BinOpCode, usize),
    (_, r, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::BinOp(o, l, r))
}

#[allow(unused_variables)]
pub fn __action29<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action30<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, l, _): (usize, Box<Expr>, usize),
    (_, o, _): (usize, BinOpCode, usize),
    (_, r, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::BinOp(o, l, r))
}

#[allow(unused_variables)]
pub fn __action31<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action32<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, l, _): (usize, Box<Expr>, usize),
    (_, o, _): (usize, BinOpCode, usize),
    (_, r, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::BinOp(o, l, r))
}

#[allow(unused_variables)]
pub fn __action33<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action34<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<WireDecl>, usize),
    (_, e, _): (usize, ::std::option::Option<WireDecl>, usize),
) -> Vec<WireDecl>
{
    match e {
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
pub fn __action35<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, WireDecl, usize),
) -> ::std::option::Option<WireDecl>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action36<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<WireDecl>
{
    None
}

#[allow(unused_variables)]
pub fn __action37<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<WireDecl>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action38<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<WireDecl>, usize),
) -> ::std::vec::Vec<WireDecl>
{
    v
}

#[allow(unused_variables)]
pub fn __action39<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, WireDecl, usize),
    (_, _, _): (usize, &'input str, usize),
) -> WireDecl
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action40<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, __0, _): (usize, WireDecl, usize),
) -> ::std::vec::Vec<WireDecl>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action41<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<WireDecl>, usize),
    (_, e, _): (usize, WireDecl, usize),
) -> ::std::vec::Vec<WireDecl>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action42<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, WireDecl, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<WireDecl>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action39(
        errors,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action43<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<WireDecl>, usize),
    __1: (usize, WireDecl, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<WireDecl>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action39(
        errors,
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action44<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::option::Option<WireDecl>, usize),
) -> Vec<WireDecl>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action37(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        errors,
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
pub fn __action45<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<WireDecl>, usize),
    __1: (usize, ::std::option::Option<WireDecl>, usize),
) -> Vec<WireDecl>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action38(
        errors,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action34(
        errors,
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action46<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, WireDecl, usize),
) -> Vec<WireDecl>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action35(
        errors,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action47<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<WireDecl>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action36(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action44(
        errors,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action48<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<WireDecl>, usize),
    __1: (usize, WireDecl, usize),
) -> Vec<WireDecl>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action35(
        errors,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        errors,
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action49<
    'input,
    'err,
>(
    errors: &'err mut Vec<ErrorRecovery<usize, (usize, &'input str), ()>>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<WireDecl>, usize),
) -> Vec<WireDecl>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action36(
        errors,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        errors,
        input,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, 'err, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, 'err, > __ToTriple<'input, 'err, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, 'err, > __ToTriple<'input, 'err, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}

// auto-generated: "lalrpop 0.14.0"
use std::str::FromStr;
use ast::{BinaryOperator, Expr, Statement};
use value::Value;
use std::collections::HashMap;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use ast::{BinaryOperator, Expr, Statement};
    use value::Value;
    use std::collections::HashMap;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Term_22_21_3d_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3c_22(&'input str),
        Term_22_3c_3d_22(&'input str),
        Term_22_3d_22(&'input str),
        Term_22_3d_3d_22(&'input str),
        Term_22_3e_22(&'input str),
        Term_22_3e_3d_22(&'input str),
        Term_22else_22(&'input str),
        Term_22end_22(&'input str),
        Term_22false_22(&'input str),
        Term_22if_22(&'input str),
        Term_22print_22(&'input str),
        Term_22true_22(&'input str),
        Term_22var_22(&'input str),
        Term_22_7b_22(&'input str),
        Term_22_7d_22(&'input str),
        Termr_23_22_5c_22_5b_5e_5c_22_5d_2a_5c_22_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(&'input str),
        Nt_28_22_3d_22_20_3cEquality_3e_29(Box<Expr>),
        Nt_28_22_3d_22_20_3cEquality_3e_29_3f(::std::option::Option<Box<Expr>>),
        Nt_28_22else_22_20_3cStatement_3e_29(Statement),
        Nt_28_22else_22_20_3cStatement_3e_29_3f(::std::option::Option<Statement>),
        NtAddition(Box<Expr>),
        NtAdditionOp(BinaryOperator),
        NtAssignment(Box<Expr>),
        NtBlock(Statement),
        NtComparison(Box<Expr>),
        NtComparisonOp(BinaryOperator),
        NtDeclaration(Statement),
        NtDeclaration_2a(::std::vec::Vec<Statement>),
        NtDeclaration_2b(::std::vec::Vec<Statement>),
        NtEquality(Box<Expr>),
        NtEqualityOp(BinaryOperator),
        NtExprStatement(Statement),
        NtExpression(Box<Expr>),
        NtIdentifier(&'input str),
        NtIfStatement(Statement),
        NtLiteral(usize),
        NtMultiplication(Box<Expr>),
        NtMultiplicationOp(BinaryOperator),
        NtPrintStatement(Statement),
        NtProgram(::std::vec::Vec<Statement>),
        NtStatement(Statement),
        NtTerm(Box<Expr>),
        NtTier_3cAdditionOp_2c_20Multiplication_3e(Box<Expr>),
        NtTier_3cComparisonOp_2c_20Addition_3e(Box<Expr>),
        NtTier_3cEqualityOp_2c_20Comparison_3e(Box<Expr>),
        NtTier_3cMultiplicationOp_2c_20Term_3e(Box<Expr>),
        NtVariableDeclaration(Statement),
        Nt____Program(::std::vec::Vec<Statement>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 26, 27, 28, 29, 30, 0, 31, 32, 33,
        // State 1
        -52, 0, -52, 0, 0, 0, -52, -52, -52, 0, -52, -52, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, -29, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45, -45,
        // State 4
        -54, 0, -54, 0, 0, 0, -54, 0, 0, 0, -54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, -23, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23,
        // State 6
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 26, 27, 28, 29, 30, 0, 31, 32, 33,
        // State 7
        0, 0, -11, 0, 0, 0, -11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, -43, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43, -43,
        // State 9
        0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        -47, 0, -47, -47, -47, -47, -47, -47, -47, 36, -47, -47, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44, -44,
        // State 12
        -46, 0, -46, -46, -46, -46, -46, -46, -46, 0, -46, -46, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        -50, 0, -50, 0, -50, -50, -50, -50, -50, 0, -50, -50, -50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, -42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42, -42,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20,
        // State 17
        -56, 0, -56, -56, -56, -56, -56, -56, -56, 0, -56, -56, -56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        -7, 0, -7, 0, 38, 39, -7, -7, -7, 0, -7, -7, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        -14, 0, -14, 0, 0, 0, -14, 41, 42, 0, -14, 43, 44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        46, 0, -25, 0, 0, 0, -25, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        -37, 0, -37, 49, -37, -37, -37, -37, -37, 0, -37, -37, -37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, -19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19,
        // State 23
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 28, 0, 0, 0, 31, 32, 33,
        // State 24
        -36, 0, -36, -36, -36, -36, -36, -36, -36, 0, -36, -36, -36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 28, 0, 0, 0, 31, 32, 33,
        // State 27
        -35, 0, -35, -35, -35, -35, -35, -35, -35, 0, -35, -35, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 33,
        // State 29
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 26, 27, 28, 29, 30, 56, 31, 32, 33,
        // State 30
        -34, 0, -34, -34, -34, -34, -34, -34, -34, 0, -34, -34, -34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        -33, 0, -33, -33, -33, -33, -33, -33, -33, 0, -33, -33, -33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        -30, 0, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24,
        // State 34
        0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 35
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 28, 0, 0, 0, 31, 32, 33,
        // State 36
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 28, 0, 0, 0, 31, 32, 33,
        // State 37
        0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, -8, 0, 0, 0, -8, -8, -8,
        // State 38
        0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0, 0, -9, 0, 0, 0, -9, -9, -9,
        // State 39
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 28, 0, 0, 0, 31, 32, 33,
        // State 40
        0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -17, 0, 0, -17, 0, 0, 0, -17, -17, -17,
        // State 41
        0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -18, 0, 0, -18, 0, 0, 0, -18, -18, -18,
        // State 42
        0, -15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, -15, 0, 0, 0, -15, -15, -15,
        // State 43
        0, -16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -16, 0, 0, -16, 0, 0, 0, -16, -16, -16,
        // State 44
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 28, 0, 0, 0, 31, 32, 33,
        // State 45
        0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0, 0, -26, 0, 0, 0, -26, -26, -26,
        // State 46
        0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0, 0, -27, 0, 0, 0, -27, -27, -27,
        // State 47
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 28, 0, 0, 0, 31, 32, 33,
        // State 48
        0, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -38, 0, 0, -38, 0, 0, 0, -38, -38, -38,
        // State 49
        0, 0, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        -47, 0, -47, -47, -47, -47, -47, -47, -47, 0, -47, -47, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 28, 0, 0, 0, 31, 32, 33,
        // State 52
        0, 0, 0, 0, 0, 0, 64, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 65, 0, 0, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 26, 27, 28, 29, 30, 67, 31, 32, 33,
        // State 55
        0, -12, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 56
        0, 0, -10, 0, 0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        -49, 0, -49, 0, -49, -49, -49, -49, -49, 0, -49, -49, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        -51, 0, -51, 0, 0, 0, -51, -51, -51, 0, -51, -51, -51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        -53, 0, -53, 0, 0, 0, -53, 0, 0, 0, -53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        -55, 0, -55, -55, -55, -55, -55, -55, -55, 0, -55, -55, -55, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        -48, 0, -48, -48, -48, -48, -48, -48, -48, 0, -48, -48, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 68, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39, -39,
        // State 64
        0, -58, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -58, -58, -58, -58, -58, -58, -58, -58, -58, -58,
        // State 65
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 0, 0, 28, 0, 0, 0, 31, 32, 33,
        // State 66
        0, -13, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 67
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 26, 27, 28, 0, 30, 0, 31, 32, 33,
        // State 68
        0, 0, 0, 0, 0, 0, 71, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 72, 73, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, -57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -57, -57, -57, -57, -57, -57, -57, -57, -57, -57,
        // State 71
        0, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25, 26, 27, 28, 0, 30, 0, 31, 32, 33,
        // State 72
        0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 75, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        -40,
        // State 1
        0,
        // State 2
        0,
        // State 3
        -45,
        // State 4
        0,
        // State 5
        -23,
        // State 6
        -41,
        // State 7
        0,
        // State 8
        -43,
        // State 9
        0,
        // State 10
        0,
        // State 11
        -44,
        // State 12
        0,
        // State 13
        0,
        // State 14
        -42,
        // State 15
        -59,
        // State 16
        -20,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        -19,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        -24,
        // State 34
        -28,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        0,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        -12,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        0,
        // State 61
        0,
        // State 62
        0,
        // State 63
        -39,
        // State 64
        -58,
        // State 65
        0,
        // State 66
        -13,
        // State 67
        0,
        // State 68
        0,
        // State 69
        0,
        // State 70
        -57,
        // State 71
        0,
        // State 72
        -32,
        // State 73
        0,
        // State 74
        -31,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 2, 0, 3, 4, 5, 0, 6, 0, 7, 8, 0, 9, 10, 11, 12, 13, 14, 0, 15, 16, 17, 18, 19, 20, 21, 22, 23, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 2, 0, 3, 4, 5, 0, 34, 0, 0, 8, 0, 9, 10, 11, 12, 13, 14, 0, 15, 0, 17, 18, 19, 20, 21, 22, 23, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 2, 0, 0, 0, 5, 0, 0, 0, 0, 50, 0, 0, 0, 51, 0, 13, 14, 0, 0, 0, 0, 18, 19, 20, 21, 22, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 2, 0, 3, 0, 5, 0, 0, 0, 0, 8, 0, 0, 53, 11, 0, 13, 14, 0, 0, 0, 0, 18, 19, 20, 21, 22, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 2, 0, 3, 4, 5, 0, 6, 0, 55, 8, 0, 9, 10, 11, 12, 13, 14, 0, 15, 0, 17, 18, 19, 20, 21, 22, 23, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 2, 0, 57, 0, 5, 0, 0, 0, 0, 8, 0, 0, 0, 11, 0, 13, 14, 0, 0, 0, 0, 18, 19, 20, 21, 22, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 13, 58, 0, 0, 0, 0, 18, 0, 0, 0, 22, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 13, 14, 0, 0, 0, 0, 18, 19, 0, 0, 22, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 2, 0, 0, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 13, 14, 0, 0, 0, 0, 18, 19, 20, 0, 22, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 13, 0, 0, 0, 0, 0, 61, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 2, 0, 3, 0, 5, 0, 0, 0, 0, 8, 0, 0, 63, 11, 0, 13, 14, 0, 0, 0, 0, 18, 19, 20, 21, 22, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 2, 0, 3, 4, 5, 0, 34, 0, 0, 8, 0, 9, 10, 11, 12, 13, 14, 0, 15, 0, 17, 18, 19, 20, 21, 22, 23, 0,
        // State 55
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 61
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 62
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 63
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 64
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 65
        0, 0, 0, 0, 2, 0, 0, 0, 5, 0, 0, 0, 0, 69, 0, 0, 0, 51, 0, 13, 14, 0, 0, 0, 0, 18, 19, 20, 21, 22, 0, 0,
        // State 66
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 67
        0, 0, 0, 0, 2, 0, 3, 4, 5, 0, 0, 0, 0, 8, 0, 9, 10, 11, 12, 13, 14, 0, 15, 0, 70, 18, 19, 20, 21, 22, 0, 0,
        // State 68
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 69
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 70
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 71
        0, 0, 0, 0, 2, 0, 3, 4, 5, 0, 0, 0, 0, 8, 0, 9, 10, 11, 12, 13, 14, 0, 15, 0, 74, 18, 19, 20, 21, 22, 0, 0,
        // State 72
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 73
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 74
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""!=""###,
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###""-""###,
            r###"";""###,
            r###""<""###,
            r###""<=""###,
            r###""=""###,
            r###""==""###,
            r###"">""###,
            r###"">=""###,
            r###""else""###,
            r###""end""###,
            r###""false""###,
            r###""if""###,
            r###""print""###,
            r###""true""###,
            r###""var""###,
            r###""{""###,
            r###""}""###,
            r###"r#"\"[^\"]*\""#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[a-zA-Z][a-zA-Z0-9_]*"#"###,
        ];
        __ACTION[(__state * 25)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    #[allow(dead_code)]
    pub fn parse_Program<
        'input,
        'c,
    >(
        constants: &'c mut HashMap<String, usize>,
        constant_pool: &'c mut Vec<Value>,
        input: &'input str,
    ) -> Result<::std::vec::Vec<Statement>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
        let mut __symbols = vec![];
        let mut __integer;
        let mut __lookahead;
        let __last_location = &mut Default::default();
        '__shift: loop {
            __lookahead = match __tokens.next() {
                Some(Ok(v)) => v,
                None => break '__shift,
                Some(Err(e)) => return Err(e),
            };
            *__last_location = __lookahead.2.clone();
            __integer = match __lookahead.1 {
                Token(3, _) if true => 0,
                Token(4, _) if true => 1,
                Token(5, _) if true => 2,
                Token(6, _) if true => 3,
                Token(7, _) if true => 4,
                Token(8, _) if true => 5,
                Token(9, _) if true => 6,
                Token(10, _) if true => 7,
                Token(11, _) if true => 8,
                Token(12, _) if true => 9,
                Token(13, _) if true => 10,
                Token(14, _) if true => 11,
                Token(15, _) if true => 12,
                Token(16, _) if true => 13,
                Token(17, _) if true => 14,
                Token(18, _) if true => 15,
                Token(19, _) if true => 16,
                Token(20, _) if true => 17,
                Token(21, _) if true => 18,
                Token(22, _) if true => 19,
                Token(23, _) if true => 20,
                Token(24, _) if true => 21,
                Token(0, _) if true => 22,
                Token(1, _) if true => 23,
                Token(2, _) if true => 24,
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
                            Token(3, __tok0) => __Symbol::Term_22_21_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            Token(4, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            Token(5, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            Token(6, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            Token(7, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            Token(8, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            Token(9, __tok0) => __Symbol::Term_22_3b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            Token(10, __tok0) => __Symbol::Term_22_3c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            Token(11, __tok0) => __Symbol::Term_22_3c_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            Token(12, __tok0) => __Symbol::Term_22_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token(13, __tok0) => __Symbol::Term_22_3d_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            Token(14, __tok0) => __Symbol::Term_22_3e_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token(15, __tok0) => __Symbol::Term_22_3e_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            Token(16, __tok0) => __Symbol::Term_22else_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            Token(17, __tok0) => __Symbol::Term_22end_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            Token(18, __tok0) => __Symbol::Term_22false_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            Token(19, __tok0) => __Symbol::Term_22if_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            Token(20, __tok0) => __Symbol::Term_22print_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            Token(21, __tok0) => __Symbol::Term_22true_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            Token(22, __tok0) => __Symbol::Term_22var_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            Token(23, __tok0) => __Symbol::Term_22_7b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            Token(24, __tok0) => __Symbol::Term_22_7d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            Token(0, __tok0) => __Symbol::Termr_23_22_5c_22_5b_5e_5c_22_5d_2a_5c_22_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            Token(1, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        24 => match __lookahead.1 {
                            Token(2, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(constants, constant_pool, input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        if r.is_err() {
                            return r;
                        }
                        return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                    }
                } else {
                    let mut __err_lookahead = Some(__lookahead);
                    let mut __err_integer: Option<usize> = Some(__integer);
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
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
                if let Some(r) = __reduce(constants, constant_pool, input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                    return r;
                }
            } else {
                let mut __err_lookahead = None;
                let mut __err_integer: Option<usize> = None;
                let __state = *__states.last().unwrap() as usize;
                let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                    token: __err_lookahead,
                    expected: __expected_tokens(__state),
                };
                return Err(__error)
            }
        }
    }
    pub fn __reduce<
        'input,
        'c,
    >(
        constants: &'c mut HashMap<String, usize>,
        constant_pool: &'c mut Vec<Value>,
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<::std::vec::Vec<Statement>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // ("=" <Equality>) = "=", Equality => ActionFn(50);
                let __sym1 = __pop_NtEquality(__symbols);
                let __sym0 = __pop_Term_22_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action50::<>(constants, constant_pool, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_3d_22_20_3cEquality_3e_29(__nt), __end));
                0
            }
            2 => {
                // ("=" <Equality>)? = "=", Equality => ActionFn(55);
                let __sym1 = __pop_NtEquality(__symbols);
                let __sym0 = __pop_Term_22_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action55::<>(constants, constant_pool, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_3d_22_20_3cEquality_3e_29_3f(__nt), __end));
                1
            }
            3 => {
                // ("=" <Equality>)? =  => ActionFn(49);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action49::<>(constants, constant_pool, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22_3d_22_20_3cEquality_3e_29_3f(__nt), __end));
                1
            }
            4 => {
                // ("else" <Statement>) = "else", Statement => ActionFn(47);
                let __sym1 = __pop_NtStatement(__symbols);
                let __sym0 = __pop_Term_22else_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action47::<>(constants, constant_pool, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22else_22_20_3cStatement_3e_29(__nt), __end));
                2
            }
            5 => {
                // ("else" <Statement>)? = "else", Statement => ActionFn(58);
                let __sym1 = __pop_NtStatement(__symbols);
                let __sym0 = __pop_Term_22else_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action58::<>(constants, constant_pool, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22else_22_20_3cStatement_3e_29_3f(__nt), __end));
                3
            }
            6 => {
                // ("else" <Statement>)? =  => ActionFn(46);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action46::<>(constants, constant_pool, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22else_22_20_3cStatement_3e_29_3f(__nt), __end));
                3
            }
            7 => {
                // Addition = Tier<AdditionOp, Multiplication> => ActionFn(18);
                let __sym0 = __pop_NtTier_3cAdditionOp_2c_20Multiplication_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAddition(__nt), __end));
                4
            }
            8 => {
                // AdditionOp = "+" => ActionFn(26);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAdditionOp(__nt), __end));
                5
            }
            9 => {
                // AdditionOp = "-" => ActionFn(27);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAdditionOp(__nt), __end));
                5
            }
            10 => {
                // Assignment = Identifier, "=", Assignment => ActionFn(14);
                let __sym2 = __pop_NtAssignment(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action14::<>(constants, constant_pool, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAssignment(__nt), __end));
                6
            }
            11 => {
                // Assignment = Equality => ActionFn(15);
                let __sym0 = __pop_NtEquality(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAssignment(__nt), __end));
                6
            }
            12 => {
                // Block = "{", "}" => ActionFn(61);
                let __sym1 = __pop_Term_22_7d_22(__symbols);
                let __sym0 = __pop_Term_22_7b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action61::<>(constants, constant_pool, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                7
            }
            13 => {
                // Block = "{", Declaration+, "}" => ActionFn(62);
                let __sym2 = __pop_Term_22_7d_22(__symbols);
                let __sym1 = __pop_NtDeclaration_2b(__symbols);
                let __sym0 = __pop_Term_22_7b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action62::<>(constants, constant_pool, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtBlock(__nt), __end));
                7
            }
            14 => {
                // Comparison = Tier<ComparisonOp, Addition> => ActionFn(17);
                let __sym0 = __pop_NtTier_3cComparisonOp_2c_20Addition_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparison(__nt), __end));
                8
            }
            15 => {
                // ComparisonOp = ">" => ActionFn(22);
                let __sym0 = __pop_Term_22_3e_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOp(__nt), __end));
                9
            }
            16 => {
                // ComparisonOp = ">=" => ActionFn(23);
                let __sym0 = __pop_Term_22_3e_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOp(__nt), __end));
                9
            }
            17 => {
                // ComparisonOp = "<" => ActionFn(24);
                let __sym0 = __pop_Term_22_3c_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOp(__nt), __end));
                9
            }
            18 => {
                // ComparisonOp = "<=" => ActionFn(25);
                let __sym0 = __pop_Term_22_3c_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComparisonOp(__nt), __end));
                9
            }
            19 => {
                // Declaration = VariableDeclaration => ActionFn(2);
                let __sym0 = __pop_NtVariableDeclaration(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDeclaration(__nt), __end));
                10
            }
            20 => {
                // Declaration = Statement => ActionFn(3);
                let __sym0 = __pop_NtStatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDeclaration(__nt), __end));
                10
            }
            21 => {
                // Declaration* =  => ActionFn(51);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action51::<>(constants, constant_pool, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDeclaration_2a(__nt), __end));
                11
            }
            22 => {
                // Declaration* = Declaration+ => ActionFn(52);
                let __sym0 = __pop_NtDeclaration_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action52::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDeclaration_2a(__nt), __end));
                11
            }
            23 => {
                // Declaration+ = Declaration => ActionFn(53);
                let __sym0 = __pop_NtDeclaration(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action53::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDeclaration_2b(__nt), __end));
                12
            }
            24 => {
                // Declaration+ = Declaration+, Declaration => ActionFn(54);
                let __sym1 = __pop_NtDeclaration(__symbols);
                let __sym0 = __pop_NtDeclaration_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action54::<>(constants, constant_pool, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDeclaration_2b(__nt), __end));
                12
            }
            25 => {
                // Equality = Tier<EqualityOp, Comparison> => ActionFn(16);
                let __sym0 = __pop_NtTier_3cEqualityOp_2c_20Comparison_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtEquality(__nt), __end));
                13
            }
            26 => {
                // EqualityOp = "!=" => ActionFn(20);
                let __sym0 = __pop_Term_22_21_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtEqualityOp(__nt), __end));
                14
            }
            27 => {
                // EqualityOp = "==" => ActionFn(21);
                let __sym0 = __pop_Term_22_3d_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtEqualityOp(__nt), __end));
                14
            }
            28 => {
                // ExprStatement = Expression, ";" => ActionFn(9);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(constants, constant_pool, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExprStatement(__nt), __end));
                15
            }
            29 => {
                // Expression = Assignment => ActionFn(13);
                let __sym0 = __pop_NtAssignment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpression(__nt), __end));
                16
            }
            30 => {
                // Identifier = r#"[a-zA-Z][a-zA-Z0-9_]*"# => ActionFn(36);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action36::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdentifier(__nt), __end));
                17
            }
            31 => {
                // IfStatement = "if", "(", Expression, ")", Statement, "else", Statement, "end" => ActionFn(59);
                let __sym7 = __pop_Term_22end_22(__symbols);
                let __sym6 = __pop_NtStatement(__symbols);
                let __sym5 = __pop_Term_22else_22(__symbols);
                let __sym4 = __pop_NtStatement(__symbols);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtExpression(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22if_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym7.2.clone();
                let __nt = super::__action59::<>(constants, constant_pool, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7);
                let __states_len = __states.len();
                __states.truncate(__states_len - 8);
                __symbols.push((__start, __Symbol::NtIfStatement(__nt), __end));
                18
            }
            32 => {
                // IfStatement = "if", "(", Expression, ")", Statement, "end" => ActionFn(60);
                let __sym5 = __pop_Term_22end_22(__symbols);
                let __sym4 = __pop_NtStatement(__symbols);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtExpression(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_Term_22if_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action60::<>(constants, constant_pool, input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtIfStatement(__nt), __end));
                18
            }
            33 => {
                // Literal = r#"[0-9]+"# => ActionFn(32);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                19
            }
            34 => {
                // Literal = r#"\"[^\"]*\""# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5c_22_5b_5e_5c_22_5d_2a_5c_22_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                19
            }
            35 => {
                // Literal = "true" => ActionFn(34);
                let __sym0 = __pop_Term_22true_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                19
            }
            36 => {
                // Literal = "false" => ActionFn(35);
                let __sym0 = __pop_Term_22false_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                19
            }
            37 => {
                // Multiplication = Tier<MultiplicationOp, Term> => ActionFn(19);
                let __sym0 = __pop_NtTier_3cMultiplicationOp_2c_20Term_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMultiplication(__nt), __end));
                20
            }
            38 => {
                // MultiplicationOp = "*" => ActionFn(28);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtMultiplicationOp(__nt), __end));
                21
            }
            39 => {
                // PrintStatement = "print", Expression, ";" => ActionFn(11);
                let __sym2 = __pop_Term_22_3b_22(__symbols);
                let __sym1 = __pop_NtExpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action11::<>(constants, constant_pool, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPrintStatement(__nt), __end));
                22
            }
            40 => {
                // Program =  => ActionFn(63);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action63::<>(constants, constant_pool, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                23
            }
            41 => {
                // Program = Declaration+ => ActionFn(64);
                let __sym0 = __pop_NtDeclaration_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action64::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                23
            }
            42 => {
                // Statement = PrintStatement => ActionFn(5);
                let __sym0 = __pop_NtPrintStatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStatement(__nt), __end));
                24
            }
            43 => {
                // Statement = ExprStatement => ActionFn(6);
                let __sym0 = __pop_NtExprStatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStatement(__nt), __end));
                24
            }
            44 => {
                // Statement = IfStatement => ActionFn(7);
                let __sym0 = __pop_NtIfStatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStatement(__nt), __end));
                24
            }
            45 => {
                // Statement = Block => ActionFn(8);
                let __sym0 = __pop_NtBlock(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStatement(__nt), __end));
                24
            }
            46 => {
                // Term = Literal => ActionFn(29);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action29::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                25
            }
            47 => {
                // Term = Identifier => ActionFn(30);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                25
            }
            48 => {
                // Term = "(", Equality, ")" => ActionFn(31);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtEquality(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action31::<>(constants, constant_pool, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                25
            }
            49 => {
                // Tier<AdditionOp, Multiplication> = Tier<AdditionOp, Multiplication>, AdditionOp, Multiplication => ActionFn(39);
                let __sym2 = __pop_NtMultiplication(__symbols);
                let __sym1 = __pop_NtAdditionOp(__symbols);
                let __sym0 = __pop_NtTier_3cAdditionOp_2c_20Multiplication_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action39::<>(constants, constant_pool, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTier_3cAdditionOp_2c_20Multiplication_3e(__nt), __end));
                26
            }
            50 => {
                // Tier<AdditionOp, Multiplication> = Multiplication => ActionFn(40);
                let __sym0 = __pop_NtMultiplication(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action40::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTier_3cAdditionOp_2c_20Multiplication_3e(__nt), __end));
                26
            }
            51 => {
                // Tier<ComparisonOp, Addition> = Tier<ComparisonOp, Addition>, ComparisonOp, Addition => ActionFn(41);
                let __sym2 = __pop_NtAddition(__symbols);
                let __sym1 = __pop_NtComparisonOp(__symbols);
                let __sym0 = __pop_NtTier_3cComparisonOp_2c_20Addition_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action41::<>(constants, constant_pool, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTier_3cComparisonOp_2c_20Addition_3e(__nt), __end));
                27
            }
            52 => {
                // Tier<ComparisonOp, Addition> = Addition => ActionFn(42);
                let __sym0 = __pop_NtAddition(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action42::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTier_3cComparisonOp_2c_20Addition_3e(__nt), __end));
                27
            }
            53 => {
                // Tier<EqualityOp, Comparison> = Tier<EqualityOp, Comparison>, EqualityOp, Comparison => ActionFn(43);
                let __sym2 = __pop_NtComparison(__symbols);
                let __sym1 = __pop_NtEqualityOp(__symbols);
                let __sym0 = __pop_NtTier_3cEqualityOp_2c_20Comparison_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action43::<>(constants, constant_pool, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTier_3cEqualityOp_2c_20Comparison_3e(__nt), __end));
                28
            }
            54 => {
                // Tier<EqualityOp, Comparison> = Comparison => ActionFn(44);
                let __sym0 = __pop_NtComparison(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action44::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTier_3cEqualityOp_2c_20Comparison_3e(__nt), __end));
                28
            }
            55 => {
                // Tier<MultiplicationOp, Term> = Tier<MultiplicationOp, Term>, MultiplicationOp, Term => ActionFn(37);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_NtMultiplicationOp(__symbols);
                let __sym0 = __pop_NtTier_3cMultiplicationOp_2c_20Term_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action37::<>(constants, constant_pool, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTier_3cMultiplicationOp_2c_20Term_3e(__nt), __end));
                29
            }
            56 => {
                // Tier<MultiplicationOp, Term> = Term => ActionFn(38);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTier_3cMultiplicationOp_2c_20Term_3e(__nt), __end));
                29
            }
            57 => {
                // VariableDeclaration = "var", Identifier, "=", Equality, ";" => ActionFn(56);
                let __sym4 = __pop_Term_22_3b_22(__symbols);
                let __sym3 = __pop_NtEquality(__symbols);
                let __sym2 = __pop_Term_22_3d_22(__symbols);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22var_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action56::<>(constants, constant_pool, input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtVariableDeclaration(__nt), __end));
                30
            }
            58 => {
                // VariableDeclaration = "var", Identifier, ";" => ActionFn(57);
                let __sym2 = __pop_Term_22_3b_22(__symbols);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22var_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action57::<>(constants, constant_pool, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtVariableDeclaration(__nt), __end));
                30
            }
            59 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(constants, constant_pool, input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 32 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_21_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_21_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3c_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3c_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3e_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3e_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22else_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22else_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22end_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22end_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22false_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22false_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22if_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22if_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22print_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22print_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22true_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22true_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22var_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22var_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_7d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_7d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_22_5b_5e_5c_22_5d_2a_5c_22_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_22_5b_5e_5c_22_5d_2a_5c_22_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_3d_22_20_3cEquality_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_3d_22_20_3cEquality_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_3d_22_20_3cEquality_3e_29_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Box<Expr>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_3d_22_20_3cEquality_3e_29_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22else_22_20_3cStatement_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22else_22_20_3cStatement_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22else_22_20_3cStatement_3e_29_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Statement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22else_22_20_3cStatement_3e_29_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAddition<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAddition(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAdditionOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinaryOperator, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAdditionOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtAssignment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAssignment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBlock<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBlock(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComparison<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComparison(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComparisonOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinaryOperator, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComparisonOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDeclaration<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDeclaration(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDeclaration_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDeclaration_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDeclaration_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDeclaration_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtEquality<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtEquality(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtEqualityOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinaryOperator, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtEqualityOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprStatement<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprStatement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExpression<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpression(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdentifier<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdentifier(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIfStatement<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIfStatement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLiteral<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLiteral(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMultiplication<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMultiplication(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtMultiplicationOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, BinaryOperator, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtMultiplicationOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtPrintStatement<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtPrintStatement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtStatement<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtStatement(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTerm<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTerm(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTier_3cAdditionOp_2c_20Multiplication_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTier_3cAdditionOp_2c_20Multiplication_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTier_3cComparisonOp_2c_20Addition_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTier_3cComparisonOp_2c_20Addition_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTier_3cEqualityOp_2c_20Comparison_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTier_3cEqualityOp_2c_20Comparison_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTier_3cMultiplicationOp_2c_20Term_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTier_3cMultiplicationOp_2c_20Term_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVariableDeclaration<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Statement, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVariableDeclaration(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Statement>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Program::parse_Program;
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use ast::{BinaryOperator, Expr, Statement};
    use value::Value;
    use std::collections::HashMap;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    use std::fmt as __fmt;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Token<'input>(pub usize, pub &'input str);
    impl<'a> __fmt::Display for Token<'a> {
        fn fmt(&self, formatter: &mut __fmt::Formatter) -> Result<(), __fmt::Error> {
            __fmt::Display::fmt(self.1, formatter)
        }
    }

    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:\")(?u:[\u{0}-!\\#-\u{10ffff}])*(?u:\")",
                "^(?u:[0-9])+",
                "^(?u:[A-Za-z])(?u:[0-9A-Z_-_a-z])*",
                "^(?u:!=)",
                "^(?u:\\()",
                "^(?u:\\))",
                "^(?u:\\*)",
                "^(?u:\\+)",
                "^(?u:\\-)",
                "^(?u:;)",
                "^(?u:<)",
                "^(?u:<=)",
                "^(?u:=)",
                "^(?u:==)",
                "^(?u:>)",
                "^(?u:>=)",
                "^(?u:else)",
                "^(?u:end)",
                "^(?u:false)",
                "^(?u:if)",
                "^(?u:print)",
                "^(?u:true)",
                "^(?u:var)",
                "^(?u:\\{)",
                "^(?u:\\})",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:\")(?u:[\u{0}-!\\#-\u{10ffff}])*(?u:\")").unwrap(),
                __regex::Regex::new("^(?u:[0-9])+").unwrap(),
                __regex::Regex::new("^(?u:[A-Za-z])(?u:[0-9A-Z_-_a-z])*").unwrap(),
                __regex::Regex::new("^(?u:!=)").unwrap(),
                __regex::Regex::new("^(?u:\\()").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
                __regex::Regex::new("^(?u:\\*)").unwrap(),
                __regex::Regex::new("^(?u:\\+)").unwrap(),
                __regex::Regex::new("^(?u:\\-)").unwrap(),
                __regex::Regex::new("^(?u:;)").unwrap(),
                __regex::Regex::new("^(?u:<)").unwrap(),
                __regex::Regex::new("^(?u:<=)").unwrap(),
                __regex::Regex::new("^(?u:=)").unwrap(),
                __regex::Regex::new("^(?u:==)").unwrap(),
                __regex::Regex::new("^(?u:>)").unwrap(),
                __regex::Regex::new("^(?u:>=)").unwrap(),
                __regex::Regex::new("^(?u:else)").unwrap(),
                __regex::Regex::new("^(?u:end)").unwrap(),
                __regex::Regex::new("^(?u:false)").unwrap(),
                __regex::Regex::new("^(?u:if)").unwrap(),
                __regex::Regex::new("^(?u:print)").unwrap(),
                __regex::Regex::new("^(?u:true)").unwrap(),
                __regex::Regex::new("^(?u:var)").unwrap(),
                __regex::Regex::new("^(?u:\\{)").unwrap(),
                __regex::Regex::new("^(?u:\\})").unwrap(),
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
        type Item = Result<(usize, Token<'input>, usize), __lalrpop_util::ParseError<usize,Token<'input>,&'static str>>;

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
                    Some(Ok((__start_offset, Token(__index, __result), __end_offset)))
                }
            }
        }
    }
}
pub use self::__intern_token::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<Statement>, usize),
) -> ::std::vec::Vec<Statement>
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<Statement>, usize),
) -> ::std::vec::Vec<Statement>
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Statement, usize),
) -> Statement
{
    (__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Statement, usize),
) -> Statement
{
    (__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, i, _): (usize, &'input str, usize),
    (_, initializer, _): (usize, ::std::option::Option<Box<Expr>>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Statement
{
    {
        let name = i.to_string();
        Statement::Var { name, initializer }
    }
}

#[allow(unused_variables)]
fn __action5<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Statement, usize),
) -> Statement
{
    (__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Statement, usize),
) -> Statement
{
    (__0)
}

#[allow(unused_variables)]
fn __action7<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Statement, usize),
) -> Statement
{
    (__0)
}

#[allow(unused_variables)]
fn __action8<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Statement, usize),
) -> Statement
{
    (__0)
}

#[allow(unused_variables)]
fn __action9<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, expression, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Statement
{
    Statement::Expression{expression:expression}
}

#[allow(unused_variables)]
fn __action10<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, i, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, t, _): (usize, Statement, usize),
    (_, e, _): (usize, ::std::option::Option<Statement>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Statement
{
    {
        let eb = match e {
            Some(s) => Some(Box::new(s)),
            None => None,
        };
        Statement::If {
            condition: i,
            then_branch: Box::new(t),
            else_branch: eb,
        }
    }
}

#[allow(unused_variables)]
fn __action11<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, expression, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Statement
{
    Statement::Print{expression:expression}
}

#[allow(unused_variables)]
fn __action12<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, statements, _): (usize, ::std::vec::Vec<Statement>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Statement
{
    Statement::Block{statements:statements}
}

#[allow(unused_variables)]
fn __action13<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action14<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, name, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, v, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    {
        Box::new(Expr::Assign {name: name.to_string(), value: v})
    }
}

#[allow(unused_variables)]
fn __action15<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action16<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action17<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action18<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action19<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action20<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::BangEq
}

#[allow(unused_variables)]
fn __action21<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::Eq
}

#[allow(unused_variables)]
fn __action22<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::Gt
}

#[allow(unused_variables)]
fn __action23<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::GtEq
}

#[allow(unused_variables)]
fn __action24<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::Lt
}

#[allow(unused_variables)]
fn __action25<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::LtEq
}

#[allow(unused_variables)]
fn __action26<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::Plus
}

#[allow(unused_variables)]
fn __action27<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::Minus
}

#[allow(unused_variables)]
fn __action28<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> BinaryOperator
{
    BinaryOperator::Star
}

#[allow(unused_variables)]
fn __action29<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, usize, usize),
) -> Box<Expr>
{
    Box::new(Expr::Literal{value: __0})
}

#[allow(unused_variables)]
fn __action30<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    Box::new(Expr::Variable{name: i.to_string()})
}

#[allow(unused_variables)]
fn __action31<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action32<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> usize
{
    {
        let idx = constants.entry(s.to_string()).or_insert_with(|| {
            let v = Value::Number(i32::from_str(s).unwrap());
            let i = constant_pool.len();
            constant_pool.push(v);
            i
        });
        *idx
    }
}

#[allow(unused_variables)]
fn __action33<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> usize
{
    {
        let literal = s[1..s.len()-1].to_string();
        let idx = constants.entry(literal.clone()).or_insert_with(|| {
            let v = Value::String(literal);
            let i = constant_pool.len();
            constant_pool.push(v);
            i
        });
        *idx
    }
}

#[allow(unused_variables)]
fn __action34<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> usize
{
    {
        let idx = constants.entry("true".to_string()).or_insert_with(|| {
            let v = Value::Bool(true);
            let i = constant_pool.len();
            constant_pool.push(v);
            i
        });
        *idx
    }
}

#[allow(unused_variables)]
fn __action35<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> usize
{
    {
        let idx = constants.entry("false".to_string()).or_insert_with(|| {
            let v = Value::Bool(false);
            let i = constant_pool.len();
            constant_pool.push(v);
            i
        });
        *idx
    }
}

#[allow(unused_variables)]
fn __action36<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action37<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, left, _): (usize, Box<Expr>, usize),
    (_, operator, _): (usize, BinaryOperator, usize),
    (_, right, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Binary{left:left, operator:operator, right:right})
}

#[allow(unused_variables)]
fn __action38<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action39<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, left, _): (usize, Box<Expr>, usize),
    (_, operator, _): (usize, BinaryOperator, usize),
    (_, right, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Binary{left:left, operator:operator, right:right})
}

#[allow(unused_variables)]
fn __action40<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action41<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, left, _): (usize, Box<Expr>, usize),
    (_, operator, _): (usize, BinaryOperator, usize),
    (_, right, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Binary{left:left, operator:operator, right:right})
}

#[allow(unused_variables)]
fn __action42<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action43<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, left, _): (usize, Box<Expr>, usize),
    (_, operator, _): (usize, BinaryOperator, usize),
    (_, right, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Binary{left:left, operator:operator, right:right})
}

#[allow(unused_variables)]
fn __action44<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action45<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Statement, usize),
) -> ::std::option::Option<Statement>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action46<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Statement>
{
    None
}

#[allow(unused_variables)]
fn __action47<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Statement, usize),
) -> Statement
{
    (__0)
}

#[allow(unused_variables)]
fn __action48<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Box<Expr>, usize),
) -> ::std::option::Option<Box<Expr>>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action49<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Box<Expr>>
{
    None
}

#[allow(unused_variables)]
fn __action50<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    (__0)
}

#[allow(unused_variables)]
fn __action51<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Statement>
{
    vec![]
}

#[allow(unused_variables)]
fn __action52<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Statement>, usize),
) -> ::std::vec::Vec<Statement>
{
    v
}

#[allow(unused_variables)]
fn __action53<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, Statement, usize),
) -> ::std::vec::Vec<Statement>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action54<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Statement>, usize),
    (_, e, _): (usize, Statement, usize),
) -> ::std::vec::Vec<Statement>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action55<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Box<Expr>, usize),
) -> ::std::option::Option<Box<Expr>>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action50(
        constants,
        constant_pool,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action48(
        constants,
        constant_pool,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action56<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
    __3: (usize, Box<Expr>, usize),
    __4: (usize, &'input str, usize),
) -> Statement
{
    let __start0 = __2.0.clone();
    let __end0 = __3.2.clone();
    let __temp0 = __action55(
        constants,
        constant_pool,
        input,
        __2,
        __3,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        constants,
        constant_pool,
        input,
        __0,
        __1,
        __temp0,
        __4,
    )
}

#[allow(unused_variables)]
fn __action57<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
) -> Statement
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action49(
        constants,
        constant_pool,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action4(
        constants,
        constant_pool,
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action58<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, Statement, usize),
) -> ::std::option::Option<Statement>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action47(
        constants,
        constant_pool,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action45(
        constants,
        constant_pool,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action59<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Box<Expr>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, Statement, usize),
    __5: (usize, &'input str, usize),
    __6: (usize, Statement, usize),
    __7: (usize, &'input str, usize),
) -> Statement
{
    let __start0 = __5.0.clone();
    let __end0 = __6.2.clone();
    let __temp0 = __action58(
        constants,
        constant_pool,
        input,
        __5,
        __6,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        constants,
        constant_pool,
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __7,
    )
}

#[allow(unused_variables)]
fn __action60<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, Box<Expr>, usize),
    __3: (usize, &'input str, usize),
    __4: (usize, Statement, usize),
    __5: (usize, &'input str, usize),
) -> Statement
{
    let __start0 = __4.2.clone();
    let __end0 = __5.0.clone();
    let __temp0 = __action46(
        constants,
        constant_pool,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        constants,
        constant_pool,
        input,
        __0,
        __1,
        __2,
        __3,
        __4,
        __temp0,
        __5,
    )
}

#[allow(unused_variables)]
fn __action61<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
) -> Statement
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action51(
        constants,
        constant_pool,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        constants,
        constant_pool,
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action62<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<Statement>, usize),
    __2: (usize, &'input str, usize),
) -> Statement
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action52(
        constants,
        constant_pool,
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action12(
        constants,
        constant_pool,
        input,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action63<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Statement>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action51(
        constants,
        constant_pool,
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        constants,
        constant_pool,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action64<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Statement>, usize),
) -> ::std::vec::Vec<Statement>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action52(
        constants,
        constant_pool,
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        constants,
        constant_pool,
        input,
        __temp0,
    )
}

pub trait __ToTriple<'input, 'c, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),Self::Error>;
}

impl<'input, 'c, > __ToTriple<'input, 'c, > for (usize, Token<'input>, usize) {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        Ok(value)
    }
}
impl<'input, 'c, > __ToTriple<'input, 'c, > for Result<(usize, Token<'input>, usize),&'static str> {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        value
    }
}

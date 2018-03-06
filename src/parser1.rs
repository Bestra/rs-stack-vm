// auto-generated: "lalrpop 0.14.0"
use std::str::FromStr;
use ast::{Expr, Statement};
use value::Value;
use std::collections::HashMap;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use ast::{Expr, Statement};
    use value::Value;
    use std::collections::HashMap;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2a_22(&'input str),
        Term_22_2b_22(&'input str),
        Term_22_2d_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3d_22(&'input str),
        Term_22false_22(&'input str),
        Term_22print_22(&'input str),
        Term_22true_22(&'input str),
        Term_22var_22(&'input str),
        Termr_23_22_5c_22_5b_5e_5c_22_5d_2a_5c_22_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(&'input str),
        Nt_28_22_3d_22_20_3cExpr_3e_29(Box<Expr>),
        Nt_28_22_3d_22_20_3cExpr_3e_29_3f(::std::option::Option<Box<Expr>>),
        NtAssignment(Box<Expr>),
        NtDeclaration(Statement),
        NtDeclaration_2a(::std::vec::Vec<Statement>),
        NtDeclaration_2b(::std::vec::Vec<Statement>),
        NtExpr(Box<Expr>),
        NtExprOp(String),
        NtExprStatement(Statement),
        NtExpression(Box<Expr>),
        NtFactor(Box<Expr>),
        NtFactorOp(String),
        NtIdentifier(&'input str),
        NtLiteral(usize),
        NtPrintStatement(Statement),
        NtProgram(::std::vec::Vec<Statement>),
        NtStatement(Statement),
        NtTerm(Box<Expr>),
        NtTier_3cExprOp_2c_20Factor_3e(Box<Expr>),
        NtTier_3cFactorOp_2c_20Term_3e(Box<Expr>),
        NtVariableDeclaration(Statement),
        Nt____Program(::std::vec::Vec<Statement>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        18, 0, 0, 0, 0, 0, 0, 19, 20, 21, 22, 23, 24, 25,
        // State 1
        0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        -10, 0, 0, 0, 0, 0, 0, -10, -10, -10, -10, -10, -10, -10,
        // State 3
        18, 0, 0, 0, 0, 0, 0, 19, 20, 21, 22, 23, 24, 25,
        // State 4
        0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        -27, 0, 0, 0, 0, 0, 0, -27, -27, -27, -27, -27, -27, -27,
        // State 6
        0, 0, 0, 0, 0, 27, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, -33, 0, -33, -33, -33, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, -30, -30, -30, -30, 28, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, -29, -29, -29, -29, -29, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        -28, 0, 0, 0, 0, 0, 0, -28, -28, -28, -28, -28, -28, -28,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        -7, 0, 0, 0, 0, 0, 0, -7, -7, -7, -7, -7, -7, -7,
        // State 13
        0, -35, -35, -35, -35, -35, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, -12, 0, 30, 31, -12, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, -17, 33, -17, -17, -17, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        -6, 0, 0, 0, 0, 0, 0, -6, -6, -6, -6, -6, -6, -6,
        // State 17
        18, 0, 0, 0, 0, 0, 0, 19, 0, 21, 0, 23, 24, 25,
        // State 18
        0, -23, -23, -23, -23, -23, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        18, 0, 0, 0, 0, 0, 0, 19, 0, 21, 0, 23, 24, 25,
        // State 20
        0, -22, -22, -22, -22, -22, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 25,
        // State 22
        0, -21, -21, -21, -21, -21, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, -20, -20, -20, -20, -20, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, -19, -19, -19, -19, -19, -19, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        -11, 0, 0, 0, 0, 0, 0, -11, -11, -11, -11, -11, -11, -11,
        // State 26
        -15, 0, 0, 0, 0, 0, 0, -15, -15, -15, -15, -15, -15, -15,
        // State 27
        18, 0, 0, 0, 0, 0, 0, 19, 0, 21, 0, 23, 24, 25,
        // State 28
        18, 0, 0, 0, 0, 0, 0, 19, 0, 21, 0, 23, 24, 25,
        // State 29
        -13, 0, 0, 0, 0, 0, 0, -13, 0, -13, 0, -13, -13, -13,
        // State 30
        -14, 0, 0, 0, 0, 0, 0, -14, 0, -14, 0, -14, -14, -14,
        // State 31
        18, 0, 0, 0, 0, 0, 0, 19, 0, 21, 0, 23, 24, 25,
        // State 32
        -18, 0, 0, 0, 0, 0, 0, -18, 0, -18, 0, -18, -18, -18,
        // State 33
        0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, -30, -30, -30, -30, -30, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 42, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 43, 44, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, -32, 0, -32, -32, -32, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, -34, -34, -34, -34, -34, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, -31, -31, -31, -31, -31, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        -24, 0, 0, 0, 0, 0, 0, -24, -24, -24, -24, -24, -24, -24,
        // State 42
        -37, 0, 0, 0, 0, 0, 0, -37, -37, -37, -37, -37, -37, -37,
        // State 43
        18, 0, 0, 0, 0, 0, 0, 19, 0, 21, 0, 23, 24, 25,
        // State 44
        0, 0, 0, 0, 0, 46, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        -36, 0, 0, 0, 0, 0, 0, -36, -36, -36, -36, -36, -36, -36,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        -25,
        // State 1
        0,
        // State 2
        -10,
        // State 3
        -26,
        // State 4
        0,
        // State 5
        -27,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        -28,
        // State 11
        -38,
        // State 12
        -7,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        -6,
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
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        -11,
        // State 26
        -15,
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
        0,
        // State 34
        0,
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
        -24,
        // State 42
        -37,
        // State 43
        0,
        // State 44
        0,
        // State 45
        -36,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 2, 3, 0, 4, 5, 0, 6, 7, 8, 0, 9, 10, 11, 12, 13, 14, 15, 16, 17, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 2, 26, 0, 0, 5, 0, 6, 7, 8, 0, 9, 10, 11, 0, 13, 14, 15, 16, 17, 0,
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
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 34, 0, 0, 0, 8, 0, 35, 10, 0, 0, 0, 14, 15, 16, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 2, 0, 0, 0, 5, 0, 0, 36, 8, 0, 9, 10, 0, 0, 0, 14, 15, 16, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 38, 0, 0, 0, 5, 0, 0, 0, 8, 0, 9, 10, 0, 0, 0, 14, 15, 16, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 35, 10, 0, 0, 0, 14, 0, 16, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 10, 0, 0, 0, 40, 0, 0, 0, 0,
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
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 45, 0, 0, 0, 8, 0, 35, 10, 0, 0, 0, 14, 15, 16, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###""-""###,
            r###"";""###,
            r###""=""###,
            r###""false""###,
            r###""print""###,
            r###""true""###,
            r###""var""###,
            r###"r#"\"[^\"]*\""#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[a-zA-Z][a-zA-Z0-9_]*"#"###,
        ];
        __ACTION[(__state * 14)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
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
                Token(0, _) if true => 11,
                Token(1, _) if true => 12,
                Token(2, _) if true => 13,
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
                let __action = __ACTION[__state * 14 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            Token(3, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            Token(4, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            Token(5, __tok0) => __Symbol::Term_22_2a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            Token(6, __tok0) => __Symbol::Term_22_2b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            Token(7, __tok0) => __Symbol::Term_22_2d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            Token(8, __tok0) => __Symbol::Term_22_3b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            Token(9, __tok0) => __Symbol::Term_22_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            Token(10, __tok0) => __Symbol::Term_22false_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            Token(11, __tok0) => __Symbol::Term_22print_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            Token(12, __tok0) => __Symbol::Term_22true_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token(13, __tok0) => __Symbol::Term_22var_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            Token(0, __tok0) => __Symbol::Termr_23_22_5c_22_5b_5e_5c_22_5d_2a_5c_22_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token(1, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
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
                // ("=" <Expr>) = "=", Expr => ActionFn(31);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action31::<>(constants, constant_pool, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_3d_22_20_3cExpr_3e_29(__nt), __end));
                0
            }
            2 => {
                // ("=" <Expr>)? = "=", Expr => ActionFn(36);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22_3d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action36::<>(constants, constant_pool, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_22_3d_22_20_3cExpr_3e_29_3f(__nt), __end));
                1
            }
            3 => {
                // ("=" <Expr>)? =  => ActionFn(30);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action30::<>(constants, constant_pool, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_22_3d_22_20_3cExpr_3e_29_3f(__nt), __end));
                1
            }
            4 => {
                // Assignment = Identifier, "=", Assignment => ActionFn(10);
                let __sym2 = __pop_NtAssignment(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action10::<>(constants, constant_pool, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtAssignment(__nt), __end));
                2
            }
            5 => {
                // Assignment = Expr => ActionFn(11);
                let __sym0 = __pop_NtExpr(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAssignment(__nt), __end));
                2
            }
            6 => {
                // Declaration = VariableDeclaration => ActionFn(2);
                let __sym0 = __pop_NtVariableDeclaration(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDeclaration(__nt), __end));
                3
            }
            7 => {
                // Declaration = Statement => ActionFn(3);
                let __sym0 = __pop_NtStatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDeclaration(__nt), __end));
                3
            }
            8 => {
                // Declaration* =  => ActionFn(32);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action32::<>(constants, constant_pool, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDeclaration_2a(__nt), __end));
                4
            }
            9 => {
                // Declaration* = Declaration+ => ActionFn(33);
                let __sym0 = __pop_NtDeclaration_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDeclaration_2a(__nt), __end));
                4
            }
            10 => {
                // Declaration+ = Declaration => ActionFn(34);
                let __sym0 = __pop_NtDeclaration(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDeclaration_2b(__nt), __end));
                5
            }
            11 => {
                // Declaration+ = Declaration+, Declaration => ActionFn(35);
                let __sym1 = __pop_NtDeclaration(__symbols);
                let __sym0 = __pop_NtDeclaration_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action35::<>(constants, constant_pool, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtDeclaration_2b(__nt), __end));
                5
            }
            12 => {
                // Expr = Tier<ExprOp, Factor> => ActionFn(12);
                let __sym0 = __pop_NtTier_3cExprOp_2c_20Factor_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpr(__nt), __end));
                6
            }
            13 => {
                // ExprOp = "+" => ActionFn(14);
                let __sym0 = __pop_Term_22_2b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprOp(__nt), __end));
                7
            }
            14 => {
                // ExprOp = "-" => ActionFn(15);
                let __sym0 = __pop_Term_22_2d_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExprOp(__nt), __end));
                7
            }
            15 => {
                // ExprStatement = Expression, ";" => ActionFn(7);
                let __sym1 = __pop_Term_22_3b_22(__symbols);
                let __sym0 = __pop_NtExpression(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action7::<>(constants, constant_pool, input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtExprStatement(__nt), __end));
                8
            }
            16 => {
                // Expression = Assignment => ActionFn(9);
                let __sym0 = __pop_NtAssignment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtExpression(__nt), __end));
                9
            }
            17 => {
                // Factor = Tier<FactorOp, Term> => ActionFn(13);
                let __sym0 = __pop_NtTier_3cFactorOp_2c_20Term_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactor(__nt), __end));
                10
            }
            18 => {
                // FactorOp = "*" => ActionFn(16);
                let __sym0 = __pop_Term_22_2a_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFactorOp(__nt), __end));
                11
            }
            19 => {
                // Identifier = r#"[a-zA-Z][a-zA-Z0-9_]*"# => ActionFn(24);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdentifier(__nt), __end));
                12
            }
            20 => {
                // Literal = r#"[0-9]+"# => ActionFn(20);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                13
            }
            21 => {
                // Literal = r#"\"[^\"]*\""# => ActionFn(21);
                let __sym0 = __pop_Termr_23_22_5c_22_5b_5e_5c_22_5d_2a_5c_22_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                13
            }
            22 => {
                // Literal = "true" => ActionFn(22);
                let __sym0 = __pop_Term_22true_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                13
            }
            23 => {
                // Literal = "false" => ActionFn(23);
                let __sym0 = __pop_Term_22false_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLiteral(__nt), __end));
                13
            }
            24 => {
                // PrintStatement = "print", Expression, ";" => ActionFn(8);
                let __sym2 = __pop_Term_22_3b_22(__symbols);
                let __sym1 = __pop_NtExpression(__symbols);
                let __sym0 = __pop_Term_22print_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action8::<>(constants, constant_pool, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtPrintStatement(__nt), __end));
                14
            }
            25 => {
                // Program =  => ActionFn(39);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action39::<>(constants, constant_pool, input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                15
            }
            26 => {
                // Program = Declaration+ => ActionFn(40);
                let __sym0 = __pop_NtDeclaration_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action40::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                15
            }
            27 => {
                // Statement = ExprStatement => ActionFn(5);
                let __sym0 = __pop_NtExprStatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStatement(__nt), __end));
                16
            }
            28 => {
                // Statement = PrintStatement => ActionFn(6);
                let __sym0 = __pop_NtPrintStatement(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtStatement(__nt), __end));
                16
            }
            29 => {
                // Term = Literal => ActionFn(17);
                let __sym0 = __pop_NtLiteral(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                17
            }
            30 => {
                // Term = Identifier => ActionFn(18);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                17
            }
            31 => {
                // Term = "(", Expr, ")" => ActionFn(19);
                let __sym2 = __pop_Term_22_29_22(__symbols);
                let __sym1 = __pop_NtExpr(__symbols);
                let __sym0 = __pop_Term_22_28_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action19::<>(constants, constant_pool, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTerm(__nt), __end));
                17
            }
            32 => {
                // Tier<ExprOp, Factor> = Tier<ExprOp, Factor>, ExprOp, Factor => ActionFn(27);
                let __sym2 = __pop_NtFactor(__symbols);
                let __sym1 = __pop_NtExprOp(__symbols);
                let __sym0 = __pop_NtTier_3cExprOp_2c_20Factor_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action27::<>(constants, constant_pool, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTier_3cExprOp_2c_20Factor_3e(__nt), __end));
                18
            }
            33 => {
                // Tier<ExprOp, Factor> = Factor => ActionFn(28);
                let __sym0 = __pop_NtFactor(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTier_3cExprOp_2c_20Factor_3e(__nt), __end));
                18
            }
            34 => {
                // Tier<FactorOp, Term> = Tier<FactorOp, Term>, FactorOp, Term => ActionFn(25);
                let __sym2 = __pop_NtTerm(__symbols);
                let __sym1 = __pop_NtFactorOp(__symbols);
                let __sym0 = __pop_NtTier_3cFactorOp_2c_20Term_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action25::<>(constants, constant_pool, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtTier_3cFactorOp_2c_20Term_3e(__nt), __end));
                19
            }
            35 => {
                // Tier<FactorOp, Term> = Term => ActionFn(26);
                let __sym0 = __pop_NtTerm(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(constants, constant_pool, input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTier_3cFactorOp_2c_20Term_3e(__nt), __end));
                19
            }
            36 => {
                // VariableDeclaration = "var", Identifier, "=", Expr, ";" => ActionFn(37);
                let __sym4 = __pop_Term_22_3b_22(__symbols);
                let __sym3 = __pop_NtExpr(__symbols);
                let __sym2 = __pop_Term_22_3d_22(__symbols);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22var_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action37::<>(constants, constant_pool, input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtVariableDeclaration(__nt), __end));
                20
            }
            37 => {
                // VariableDeclaration = "var", Identifier, ";" => ActionFn(38);
                let __sym2 = __pop_Term_22_3b_22(__symbols);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22var_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action38::<>(constants, constant_pool, input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtVariableDeclaration(__nt), __end));
                20
            }
            38 => {
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
        let __next_state = __GOTO[__state * 22 + __nonterminal] - 1;
        __states.push(__next_state);
        None
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
    fn __pop_Nt_28_22_3d_22_20_3cExpr_3e_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_3d_22_20_3cExpr_3e_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_22_3d_22_20_3cExpr_3e_29_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Box<Expr>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_22_3d_22_20_3cExpr_3e_29_3f(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtExpr<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExpr(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtExprOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtExprOp(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtFactor<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactor(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFactorOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFactorOp(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtTier_3cExprOp_2c_20Factor_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTier_3cExprOp_2c_20Factor_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTier_3cFactorOp_2c_20Term_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expr>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTier_3cFactorOp_2c_20Term_3e(__v), __r) => (__l, __v, __r),
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
    use ast::{Expr, Statement};
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
                "^(?u:\\()",
                "^(?u:\\))",
                "^(?u:\\*)",
                "^(?u:\\+)",
                "^(?u:\\-)",
                "^(?u:;)",
                "^(?u:=)",
                "^(?u:false)",
                "^(?u:print)",
                "^(?u:true)",
                "^(?u:var)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:\")(?u:[\u{0}-!\\#-\u{10ffff}])*(?u:\")").unwrap(),
                __regex::Regex::new("^(?u:[0-9])+").unwrap(),
                __regex::Regex::new("^(?u:[A-Za-z])(?u:[0-9A-Z_-_a-z])*").unwrap(),
                __regex::Regex::new("^(?u:\\()").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
                __regex::Regex::new("^(?u:\\*)").unwrap(),
                __regex::Regex::new("^(?u:\\+)").unwrap(),
                __regex::Regex::new("^(?u:\\-)").unwrap(),
                __regex::Regex::new("^(?u:;)").unwrap(),
                __regex::Regex::new("^(?u:=)").unwrap(),
                __regex::Regex::new("^(?u:false)").unwrap(),
                __regex::Regex::new("^(?u:print)").unwrap(),
                __regex::Regex::new("^(?u:true)").unwrap(),
                __regex::Regex::new("^(?u:var)").unwrap(),
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
                    for __i in 0 .. 14 {
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
    (_, expression, _): (usize, Box<Expr>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Statement
{
    Statement::Expression{expression:expression}
}

#[allow(unused_variables)]
fn __action8<
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
fn __action9<
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
fn __action10<
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
fn __action11<
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
fn __action12<
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
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    "+".to_string()
}

#[allow(unused_variables)]
fn __action15<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    "-".to_string()
}

#[allow(unused_variables)]
fn __action16<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    "*".to_string()
}

#[allow(unused_variables)]
fn __action17<
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
fn __action18<
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
fn __action19<
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
fn __action20<
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
fn __action21<
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
fn __action22<
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
fn __action23<
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
fn __action24<
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
fn __action25<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, left, _): (usize, Box<Expr>, usize),
    (_, operator, _): (usize, String, usize),
    (_, right, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Binary{left:left, operator:operator, right:right})
}

#[allow(unused_variables)]
fn __action26<
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
fn __action27<
    'input,
    'c,
>(
    constants: &'c mut HashMap<String, usize>,
    constant_pool: &'c mut Vec<Value>,
    input: &'input str,
    (_, left, _): (usize, Box<Expr>, usize),
    (_, operator, _): (usize, String, usize),
    (_, right, _): (usize, Box<Expr>, usize),
) -> Box<Expr>
{
    Box::new(Expr::Binary{left:left, operator:operator, right:right})
}

#[allow(unused_variables)]
fn __action28<
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
fn __action29<
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
fn __action30<
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
fn __action31<
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
fn __action32<
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
fn __action33<
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
fn __action34<
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
fn __action35<
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
fn __action36<
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
    let __temp0 = __action31(
        constants,
        constant_pool,
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        constants,
        constant_pool,
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action37<
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
    let __temp0 = __action36(
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
fn __action38<
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
    let __temp0 = __action30(
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
fn __action39<
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
    let __temp0 = __action32(
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
fn __action40<
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
    let __temp0 = __action33(
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

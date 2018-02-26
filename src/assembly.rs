// auto-generated: "lalrpop 0.14.0"
use std::str::FromStr;
use instruction::{Instruction, Ref, OpCode};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Comment {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use instruction::{Instruction, Ref, OpCode};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Term_22_3a_22(&'input str),
        Term_22ADD_22(&'input str),
        Term_22AND_22(&'input str),
        Term_22CALL_22(&'input str),
        Term_22DUP_22(&'input str),
        Term_22HALT_22(&'input str),
        Term_22ISEQ_22(&'input str),
        Term_22ISGE_22(&'input str),
        Term_22ISGT_22(&'input str),
        Term_22ISLT_22(&'input str),
        Term_22JIF_22(&'input str),
        Term_22JMP_22(&'input str),
        Term_22LOAD_22(&'input str),
        Term_22MULT_22(&'input str),
        Term_22NOT_22(&'input str),
        Term_22OR_22(&'input str),
        Term_22POP_22(&'input str),
        Term_22PRINT_22(&'input str),
        Term_22PUSH_22(&'input str),
        Term_22RET_22(&'input str),
        Term_22STORE_22(&'input str),
        Term_22SUB_22(&'input str),
        Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(&'input str),
        NtAddress(usize),
        NtComment(Instruction),
        NtIdentifier(&'input str),
        NtInteger(i32),
        NtLabel(Instruction),
        NtLine(Instruction),
        NtLine_2a(::std::vec::Vec<Instruction>),
        NtLine_2b(::std::vec::Vec<Instruction>),
        NtOp(Instruction),
        NtProgram(Vec<Instruction>),
        NtRef(Instruction),
        Nt____Comment(Instruction),
        Nt____Label(Instruction),
        Nt____Line(Instruction),
        Nt____Program(Vec<Instruction>),
        Nt____Ref(Instruction),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        0,
        // State 1
        -37,
        // State 2
        -2,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"":""###,
            r###""ADD""###,
            r###""AND""###,
            r###""CALL""###,
            r###""DUP""###,
            r###""HALT""###,
            r###""ISEQ""###,
            r###""ISGE""###,
            r###""ISGT""###,
            r###""ISLT""###,
            r###""JIF""###,
            r###""JMP""###,
            r###""LOAD""###,
            r###""MULT""###,
            r###""NOT""###,
            r###""OR""###,
            r###""POP""###,
            r###""PRINT""###,
            r###""PUSH""###,
            r###""RET""###,
            r###""STORE""###,
            r###""SUB""###,
            r###"r#"\'[a-z0-9]+\'"#"###,
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
    pub fn parse_Comment<
        'input,
    >(
        input: &'input str,
    ) -> Result<Instruction, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
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
                            Token(3, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            Token(4, __tok0) => __Symbol::Term_22ADD_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            Token(5, __tok0) => __Symbol::Term_22AND_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            Token(6, __tok0) => __Symbol::Term_22CALL_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            Token(7, __tok0) => __Symbol::Term_22DUP_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            Token(8, __tok0) => __Symbol::Term_22HALT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            Token(9, __tok0) => __Symbol::Term_22ISEQ_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            Token(10, __tok0) => __Symbol::Term_22ISGE_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            Token(11, __tok0) => __Symbol::Term_22ISGT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            Token(12, __tok0) => __Symbol::Term_22ISLT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token(13, __tok0) => __Symbol::Term_22JIF_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            Token(14, __tok0) => __Symbol::Term_22JMP_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token(15, __tok0) => __Symbol::Term_22LOAD_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            Token(16, __tok0) => __Symbol::Term_22MULT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            Token(17, __tok0) => __Symbol::Term_22NOT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            Token(18, __tok0) => __Symbol::Term_22OR_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            Token(19, __tok0) => __Symbol::Term_22POP_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            Token(20, __tok0) => __Symbol::Term_22PRINT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            Token(21, __tok0) => __Symbol::Term_22PUSH_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            Token(22, __tok0) => __Symbol::Term_22RET_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            Token(23, __tok0) => __Symbol::Term_22STORE_22((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            Token(24, __tok0) => __Symbol::Term_22SUB_22((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            Token(0, __tok0) => __Symbol::Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23((__tok0)),
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
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Instruction,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Address = r#"[0-9]+"# => ActionFn(34);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAddress(__nt), __end));
                0
            }
            2 => {
                // Comment = r#"\'[a-z0-9]+\'"# => ActionFn(11);
                let __sym0 = __pop_Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComment(__nt), __end));
                1
            }
            3 => {
                // Identifier = r#"[a-zA-Z][a-zA-Z0-9_]*"# => ActionFn(35);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdentifier(__nt), __end));
                2
            }
            4 => {
                // Integer = r#"[0-9]+"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInteger(__nt), __end));
                3
            }
            5 => {
                // Label = Identifier, ":" => ActionFn(10);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLabel(__nt), __end));
                4
            }
            6 => {
                // Line = Label => ActionFn(6);
                let __sym0 = __pop_NtLabel(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            7 => {
                // Line = Op => ActionFn(7);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            8 => {
                // Line = Ref => ActionFn(8);
                let __sym0 = __pop_NtRef(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            9 => {
                // Line = Comment => ActionFn(9);
                let __sym0 = __pop_NtComment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            10 => {
                // Line* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtLine_2a(__nt), __end));
                6
            }
            11 => {
                // Line* = Line+ => ActionFn(37);
                let __sym0 = __pop_NtLine_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine_2a(__nt), __end));
                6
            }
            12 => {
                // Line+ = Line => ActionFn(38);
                let __sym0 = __pop_NtLine(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine_2b(__nt), __end));
                7
            }
            13 => {
                // Line+ = Line+, Line => ActionFn(39);
                let __sym1 = __pop_NtLine(__symbols);
                let __sym0 = __pop_NtLine_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLine_2b(__nt), __end));
                7
            }
            14 => {
                // Op = "ADD" => ActionFn(15);
                let __sym0 = __pop_Term_22ADD_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            15 => {
                // Op = "HALT" => ActionFn(16);
                let __sym0 = __pop_Term_22HALT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            16 => {
                // Op = "PUSH", Integer => ActionFn(17);
                let __sym1 = __pop_NtInteger(__symbols);
                let __sym0 = __pop_Term_22PUSH_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            17 => {
                // Op = "SUB" => ActionFn(18);
                let __sym0 = __pop_Term_22SUB_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            18 => {
                // Op = "MULT" => ActionFn(19);
                let __sym0 = __pop_Term_22MULT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            19 => {
                // Op = "NOT" => ActionFn(20);
                let __sym0 = __pop_Term_22NOT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            20 => {
                // Op = "AND" => ActionFn(21);
                let __sym0 = __pop_Term_22AND_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            21 => {
                // Op = "OR" => ActionFn(22);
                let __sym0 = __pop_Term_22OR_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            22 => {
                // Op = "POP" => ActionFn(23);
                let __sym0 = __pop_Term_22POP_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            23 => {
                // Op = "DUP" => ActionFn(24);
                let __sym0 = __pop_Term_22DUP_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            24 => {
                // Op = "ISEQ" => ActionFn(25);
                let __sym0 = __pop_Term_22ISEQ_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            25 => {
                // Op = "ISGT" => ActionFn(26);
                let __sym0 = __pop_Term_22ISGT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            26 => {
                // Op = "ISGE" => ActionFn(27);
                let __sym0 = __pop_Term_22ISGE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            27 => {
                // Op = "ISLT" => ActionFn(28);
                let __sym0 = __pop_Term_22ISLT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            28 => {
                // Op = "LOAD", Address => ActionFn(29);
                let __sym1 = __pop_NtAddress(__symbols);
                let __sym0 = __pop_Term_22LOAD_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            29 => {
                // Op = "STORE", Address => ActionFn(30);
                let __sym1 = __pop_NtAddress(__symbols);
                let __sym0 = __pop_Term_22STORE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action30::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            30 => {
                // Op = "PRINT" => ActionFn(31);
                let __sym0 = __pop_Term_22PRINT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            31 => {
                // Op = "RET" => ActionFn(32);
                let __sym0 = __pop_Term_22RET_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            32 => {
                // Program =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                9
            }
            33 => {
                // Program = Line+ => ActionFn(41);
                let __sym0 = __pop_NtLine_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                9
            }
            34 => {
                // Ref = "CALL", Identifier => ActionFn(12);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22CALL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRef(__nt), __end));
                10
            }
            35 => {
                // Ref = "JMP", Identifier => ActionFn(13);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22JMP_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRef(__nt), __end));
                10
            }
            36 => {
                // Ref = "JIF", Identifier => ActionFn(14);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22JIF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRef(__nt), __end));
                10
            }
            37 => {
                // __Comment = Comment => ActionFn(3);
                let __sym0 = __pop_NtComment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            38 => {
                // __Label = Label => ActionFn(2);
                let __sym0 = __pop_NtLabel(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Label(__nt), __end));
                12
            }
            39 => {
                // __Line = Line => ActionFn(1);
                let __sym0 = __pop_NtLine(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Line(__nt), __end));
                13
            }
            40 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Program(__nt), __end));
                14
            }
            41 => {
                // __Ref = Ref => ActionFn(4);
                let __sym0 = __pop_NtRef(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Ref(__nt), __end));
                15
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 16 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ADD_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ADD_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22AND_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22AND_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22CALL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22CALL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22DUP_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22DUP_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22HALT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22HALT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISEQ_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISEQ_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISGE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISGE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISGT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISGT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISLT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISLT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22JIF_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22JIF_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22JMP_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22JMP_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22LOAD_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22LOAD_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22MULT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22MULT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22NOT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22NOT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22OR_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22OR_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22POP_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22POP_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22PRINT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22PRINT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22PUSH_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22PUSH_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22RET_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22RET_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22STORE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22STORE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22SUB_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22SUB_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtAddress<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAddress(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComment(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtInteger<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInteger(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLabel<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLabel(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRef<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRef(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Comment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Comment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Label<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Label(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Ref<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Ref(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Comment::parse_Comment;

mod __parse__Label {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use instruction::{Instruction, Ref, OpCode};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Term_22_3a_22(&'input str),
        Term_22ADD_22(&'input str),
        Term_22AND_22(&'input str),
        Term_22CALL_22(&'input str),
        Term_22DUP_22(&'input str),
        Term_22HALT_22(&'input str),
        Term_22ISEQ_22(&'input str),
        Term_22ISGE_22(&'input str),
        Term_22ISGT_22(&'input str),
        Term_22ISLT_22(&'input str),
        Term_22JIF_22(&'input str),
        Term_22JMP_22(&'input str),
        Term_22LOAD_22(&'input str),
        Term_22MULT_22(&'input str),
        Term_22NOT_22(&'input str),
        Term_22OR_22(&'input str),
        Term_22POP_22(&'input str),
        Term_22PRINT_22(&'input str),
        Term_22PUSH_22(&'input str),
        Term_22RET_22(&'input str),
        Term_22STORE_22(&'input str),
        Term_22SUB_22(&'input str),
        Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(&'input str),
        NtAddress(usize),
        NtComment(Instruction),
        NtIdentifier(&'input str),
        NtInteger(i32),
        NtLabel(Instruction),
        NtLine(Instruction),
        NtLine_2a(::std::vec::Vec<Instruction>),
        NtLine_2b(::std::vec::Vec<Instruction>),
        NtOp(Instruction),
        NtProgram(Vec<Instruction>),
        NtRef(Instruction),
        Nt____Comment(Instruction),
        Nt____Label(Instruction),
        Nt____Line(Instruction),
        Nt____Program(Vec<Instruction>),
        Nt____Ref(Instruction),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4,
        // State 1
        5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        0,
        // State 1
        0,
        // State 2
        -38,
        // State 3
        0,
        // State 4
        -5,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 2, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"":""###,
            r###""ADD""###,
            r###""AND""###,
            r###""CALL""###,
            r###""DUP""###,
            r###""HALT""###,
            r###""ISEQ""###,
            r###""ISGE""###,
            r###""ISGT""###,
            r###""ISLT""###,
            r###""JIF""###,
            r###""JMP""###,
            r###""LOAD""###,
            r###""MULT""###,
            r###""NOT""###,
            r###""OR""###,
            r###""POP""###,
            r###""PRINT""###,
            r###""PUSH""###,
            r###""RET""###,
            r###""STORE""###,
            r###""SUB""###,
            r###"r#"\'[a-z0-9]+\'"#"###,
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
    pub fn parse_Label<
        'input,
    >(
        input: &'input str,
    ) -> Result<Instruction, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
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
                            Token(3, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            Token(4, __tok0) => __Symbol::Term_22ADD_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            Token(5, __tok0) => __Symbol::Term_22AND_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            Token(6, __tok0) => __Symbol::Term_22CALL_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            Token(7, __tok0) => __Symbol::Term_22DUP_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            Token(8, __tok0) => __Symbol::Term_22HALT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            Token(9, __tok0) => __Symbol::Term_22ISEQ_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            Token(10, __tok0) => __Symbol::Term_22ISGE_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            Token(11, __tok0) => __Symbol::Term_22ISGT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            Token(12, __tok0) => __Symbol::Term_22ISLT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token(13, __tok0) => __Symbol::Term_22JIF_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            Token(14, __tok0) => __Symbol::Term_22JMP_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token(15, __tok0) => __Symbol::Term_22LOAD_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            Token(16, __tok0) => __Symbol::Term_22MULT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            Token(17, __tok0) => __Symbol::Term_22NOT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            Token(18, __tok0) => __Symbol::Term_22OR_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            Token(19, __tok0) => __Symbol::Term_22POP_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            Token(20, __tok0) => __Symbol::Term_22PRINT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            Token(21, __tok0) => __Symbol::Term_22PUSH_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            Token(22, __tok0) => __Symbol::Term_22RET_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            Token(23, __tok0) => __Symbol::Term_22STORE_22((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            Token(24, __tok0) => __Symbol::Term_22SUB_22((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            Token(0, __tok0) => __Symbol::Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23((__tok0)),
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
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Instruction,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Address = r#"[0-9]+"# => ActionFn(34);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAddress(__nt), __end));
                0
            }
            2 => {
                // Comment = r#"\'[a-z0-9]+\'"# => ActionFn(11);
                let __sym0 = __pop_Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComment(__nt), __end));
                1
            }
            3 => {
                // Identifier = r#"[a-zA-Z][a-zA-Z0-9_]*"# => ActionFn(35);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdentifier(__nt), __end));
                2
            }
            4 => {
                // Integer = r#"[0-9]+"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInteger(__nt), __end));
                3
            }
            5 => {
                // Label = Identifier, ":" => ActionFn(10);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLabel(__nt), __end));
                4
            }
            6 => {
                // Line = Label => ActionFn(6);
                let __sym0 = __pop_NtLabel(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            7 => {
                // Line = Op => ActionFn(7);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            8 => {
                // Line = Ref => ActionFn(8);
                let __sym0 = __pop_NtRef(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            9 => {
                // Line = Comment => ActionFn(9);
                let __sym0 = __pop_NtComment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            10 => {
                // Line* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtLine_2a(__nt), __end));
                6
            }
            11 => {
                // Line* = Line+ => ActionFn(37);
                let __sym0 = __pop_NtLine_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine_2a(__nt), __end));
                6
            }
            12 => {
                // Line+ = Line => ActionFn(38);
                let __sym0 = __pop_NtLine(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine_2b(__nt), __end));
                7
            }
            13 => {
                // Line+ = Line+, Line => ActionFn(39);
                let __sym1 = __pop_NtLine(__symbols);
                let __sym0 = __pop_NtLine_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLine_2b(__nt), __end));
                7
            }
            14 => {
                // Op = "ADD" => ActionFn(15);
                let __sym0 = __pop_Term_22ADD_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            15 => {
                // Op = "HALT" => ActionFn(16);
                let __sym0 = __pop_Term_22HALT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            16 => {
                // Op = "PUSH", Integer => ActionFn(17);
                let __sym1 = __pop_NtInteger(__symbols);
                let __sym0 = __pop_Term_22PUSH_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            17 => {
                // Op = "SUB" => ActionFn(18);
                let __sym0 = __pop_Term_22SUB_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            18 => {
                // Op = "MULT" => ActionFn(19);
                let __sym0 = __pop_Term_22MULT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            19 => {
                // Op = "NOT" => ActionFn(20);
                let __sym0 = __pop_Term_22NOT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            20 => {
                // Op = "AND" => ActionFn(21);
                let __sym0 = __pop_Term_22AND_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            21 => {
                // Op = "OR" => ActionFn(22);
                let __sym0 = __pop_Term_22OR_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            22 => {
                // Op = "POP" => ActionFn(23);
                let __sym0 = __pop_Term_22POP_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            23 => {
                // Op = "DUP" => ActionFn(24);
                let __sym0 = __pop_Term_22DUP_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            24 => {
                // Op = "ISEQ" => ActionFn(25);
                let __sym0 = __pop_Term_22ISEQ_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            25 => {
                // Op = "ISGT" => ActionFn(26);
                let __sym0 = __pop_Term_22ISGT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            26 => {
                // Op = "ISGE" => ActionFn(27);
                let __sym0 = __pop_Term_22ISGE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            27 => {
                // Op = "ISLT" => ActionFn(28);
                let __sym0 = __pop_Term_22ISLT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            28 => {
                // Op = "LOAD", Address => ActionFn(29);
                let __sym1 = __pop_NtAddress(__symbols);
                let __sym0 = __pop_Term_22LOAD_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            29 => {
                // Op = "STORE", Address => ActionFn(30);
                let __sym1 = __pop_NtAddress(__symbols);
                let __sym0 = __pop_Term_22STORE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action30::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            30 => {
                // Op = "PRINT" => ActionFn(31);
                let __sym0 = __pop_Term_22PRINT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            31 => {
                // Op = "RET" => ActionFn(32);
                let __sym0 = __pop_Term_22RET_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            32 => {
                // Program =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                9
            }
            33 => {
                // Program = Line+ => ActionFn(41);
                let __sym0 = __pop_NtLine_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                9
            }
            34 => {
                // Ref = "CALL", Identifier => ActionFn(12);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22CALL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRef(__nt), __end));
                10
            }
            35 => {
                // Ref = "JMP", Identifier => ActionFn(13);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22JMP_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRef(__nt), __end));
                10
            }
            36 => {
                // Ref = "JIF", Identifier => ActionFn(14);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22JIF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRef(__nt), __end));
                10
            }
            37 => {
                // __Comment = Comment => ActionFn(3);
                let __sym0 = __pop_NtComment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Comment(__nt), __end));
                11
            }
            38 => {
                // __Label = Label => ActionFn(2);
                let __sym0 = __pop_NtLabel(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            39 => {
                // __Line = Line => ActionFn(1);
                let __sym0 = __pop_NtLine(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Line(__nt), __end));
                13
            }
            40 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Program(__nt), __end));
                14
            }
            41 => {
                // __Ref = Ref => ActionFn(4);
                let __sym0 = __pop_NtRef(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Ref(__nt), __end));
                15
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 16 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ADD_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ADD_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22AND_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22AND_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22CALL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22CALL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22DUP_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22DUP_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22HALT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22HALT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISEQ_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISEQ_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISGE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISGE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISGT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISGT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISLT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISLT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22JIF_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22JIF_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22JMP_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22JMP_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22LOAD_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22LOAD_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22MULT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22MULT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22NOT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22NOT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22OR_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22OR_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22POP_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22POP_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22PRINT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22PRINT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22PUSH_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22PUSH_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22RET_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22RET_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22STORE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22STORE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22SUB_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22SUB_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtAddress<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAddress(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComment(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtInteger<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInteger(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLabel<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLabel(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRef<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRef(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Comment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Comment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Label<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Label(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Ref<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Ref(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Label::parse_Label;

mod __parse__Line {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use instruction::{Instruction, Ref, OpCode};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Term_22_3a_22(&'input str),
        Term_22ADD_22(&'input str),
        Term_22AND_22(&'input str),
        Term_22CALL_22(&'input str),
        Term_22DUP_22(&'input str),
        Term_22HALT_22(&'input str),
        Term_22ISEQ_22(&'input str),
        Term_22ISGE_22(&'input str),
        Term_22ISGT_22(&'input str),
        Term_22ISLT_22(&'input str),
        Term_22JIF_22(&'input str),
        Term_22JMP_22(&'input str),
        Term_22LOAD_22(&'input str),
        Term_22MULT_22(&'input str),
        Term_22NOT_22(&'input str),
        Term_22OR_22(&'input str),
        Term_22POP_22(&'input str),
        Term_22PRINT_22(&'input str),
        Term_22PUSH_22(&'input str),
        Term_22RET_22(&'input str),
        Term_22STORE_22(&'input str),
        Term_22SUB_22(&'input str),
        Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(&'input str),
        NtAddress(usize),
        NtComment(Instruction),
        NtIdentifier(&'input str),
        NtInteger(i32),
        NtLabel(Instruction),
        NtLine(Instruction),
        NtLine_2a(::std::vec::Vec<Instruction>),
        NtLine_2b(::std::vec::Vec<Instruction>),
        NtOp(Instruction),
        NtProgram(Vec<Instruction>),
        NtRef(Instruction),
        Nt____Comment(Instruction),
        Nt____Label(Instruction),
        Nt____Line(Instruction),
        Nt____Program(Vec<Instruction>),
        Nt____Ref(Instruction),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 0, 30,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 30,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        -3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        0,
        // State 1
        -9,
        // State 2
        0,
        // State 3
        -6,
        // State 4
        -39,
        // State 5
        -7,
        // State 6
        -8,
        // State 7
        -14,
        // State 8
        -20,
        // State 9
        0,
        // State 10
        -23,
        // State 11
        -15,
        // State 12
        -24,
        // State 13
        -26,
        // State 14
        -25,
        // State 15
        -27,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        -18,
        // State 20
        -19,
        // State 21
        -21,
        // State 22
        -22,
        // State 23
        -30,
        // State 24
        0,
        // State 25
        -31,
        // State 26
        0,
        // State 27
        -17,
        // State 28
        -2,
        // State 29
        -3,
        // State 30
        -5,
        // State 31
        -34,
        // State 32
        -36,
        // State 33
        -35,
        // State 34
        -28,
        // State 35
        -1,
        // State 36
        -16,
        // State 37
        -4,
        // State 38
        -29,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 2, 3, 0, 4, 5, 0, 0, 6, 0, 7, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 34, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"":""###,
            r###""ADD""###,
            r###""AND""###,
            r###""CALL""###,
            r###""DUP""###,
            r###""HALT""###,
            r###""ISEQ""###,
            r###""ISGE""###,
            r###""ISGT""###,
            r###""ISLT""###,
            r###""JIF""###,
            r###""JMP""###,
            r###""LOAD""###,
            r###""MULT""###,
            r###""NOT""###,
            r###""OR""###,
            r###""POP""###,
            r###""PRINT""###,
            r###""PUSH""###,
            r###""RET""###,
            r###""STORE""###,
            r###""SUB""###,
            r###"r#"\'[a-z0-9]+\'"#"###,
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
    pub fn parse_Line<
        'input,
    >(
        input: &'input str,
    ) -> Result<Instruction, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
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
                            Token(3, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            Token(4, __tok0) => __Symbol::Term_22ADD_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            Token(5, __tok0) => __Symbol::Term_22AND_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            Token(6, __tok0) => __Symbol::Term_22CALL_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            Token(7, __tok0) => __Symbol::Term_22DUP_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            Token(8, __tok0) => __Symbol::Term_22HALT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            Token(9, __tok0) => __Symbol::Term_22ISEQ_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            Token(10, __tok0) => __Symbol::Term_22ISGE_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            Token(11, __tok0) => __Symbol::Term_22ISGT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            Token(12, __tok0) => __Symbol::Term_22ISLT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token(13, __tok0) => __Symbol::Term_22JIF_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            Token(14, __tok0) => __Symbol::Term_22JMP_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token(15, __tok0) => __Symbol::Term_22LOAD_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            Token(16, __tok0) => __Symbol::Term_22MULT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            Token(17, __tok0) => __Symbol::Term_22NOT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            Token(18, __tok0) => __Symbol::Term_22OR_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            Token(19, __tok0) => __Symbol::Term_22POP_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            Token(20, __tok0) => __Symbol::Term_22PRINT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            Token(21, __tok0) => __Symbol::Term_22PUSH_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            Token(22, __tok0) => __Symbol::Term_22RET_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            Token(23, __tok0) => __Symbol::Term_22STORE_22((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            Token(24, __tok0) => __Symbol::Term_22SUB_22((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            Token(0, __tok0) => __Symbol::Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23((__tok0)),
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
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Instruction,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Address = r#"[0-9]+"# => ActionFn(34);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAddress(__nt), __end));
                0
            }
            2 => {
                // Comment = r#"\'[a-z0-9]+\'"# => ActionFn(11);
                let __sym0 = __pop_Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComment(__nt), __end));
                1
            }
            3 => {
                // Identifier = r#"[a-zA-Z][a-zA-Z0-9_]*"# => ActionFn(35);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdentifier(__nt), __end));
                2
            }
            4 => {
                // Integer = r#"[0-9]+"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInteger(__nt), __end));
                3
            }
            5 => {
                // Label = Identifier, ":" => ActionFn(10);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLabel(__nt), __end));
                4
            }
            6 => {
                // Line = Label => ActionFn(6);
                let __sym0 = __pop_NtLabel(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            7 => {
                // Line = Op => ActionFn(7);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            8 => {
                // Line = Ref => ActionFn(8);
                let __sym0 = __pop_NtRef(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            9 => {
                // Line = Comment => ActionFn(9);
                let __sym0 = __pop_NtComment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            10 => {
                // Line* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtLine_2a(__nt), __end));
                6
            }
            11 => {
                // Line* = Line+ => ActionFn(37);
                let __sym0 = __pop_NtLine_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine_2a(__nt), __end));
                6
            }
            12 => {
                // Line+ = Line => ActionFn(38);
                let __sym0 = __pop_NtLine(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine_2b(__nt), __end));
                7
            }
            13 => {
                // Line+ = Line+, Line => ActionFn(39);
                let __sym1 = __pop_NtLine(__symbols);
                let __sym0 = __pop_NtLine_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLine_2b(__nt), __end));
                7
            }
            14 => {
                // Op = "ADD" => ActionFn(15);
                let __sym0 = __pop_Term_22ADD_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            15 => {
                // Op = "HALT" => ActionFn(16);
                let __sym0 = __pop_Term_22HALT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            16 => {
                // Op = "PUSH", Integer => ActionFn(17);
                let __sym1 = __pop_NtInteger(__symbols);
                let __sym0 = __pop_Term_22PUSH_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            17 => {
                // Op = "SUB" => ActionFn(18);
                let __sym0 = __pop_Term_22SUB_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            18 => {
                // Op = "MULT" => ActionFn(19);
                let __sym0 = __pop_Term_22MULT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            19 => {
                // Op = "NOT" => ActionFn(20);
                let __sym0 = __pop_Term_22NOT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            20 => {
                // Op = "AND" => ActionFn(21);
                let __sym0 = __pop_Term_22AND_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            21 => {
                // Op = "OR" => ActionFn(22);
                let __sym0 = __pop_Term_22OR_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            22 => {
                // Op = "POP" => ActionFn(23);
                let __sym0 = __pop_Term_22POP_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            23 => {
                // Op = "DUP" => ActionFn(24);
                let __sym0 = __pop_Term_22DUP_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            24 => {
                // Op = "ISEQ" => ActionFn(25);
                let __sym0 = __pop_Term_22ISEQ_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            25 => {
                // Op = "ISGT" => ActionFn(26);
                let __sym0 = __pop_Term_22ISGT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            26 => {
                // Op = "ISGE" => ActionFn(27);
                let __sym0 = __pop_Term_22ISGE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            27 => {
                // Op = "ISLT" => ActionFn(28);
                let __sym0 = __pop_Term_22ISLT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            28 => {
                // Op = "LOAD", Address => ActionFn(29);
                let __sym1 = __pop_NtAddress(__symbols);
                let __sym0 = __pop_Term_22LOAD_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            29 => {
                // Op = "STORE", Address => ActionFn(30);
                let __sym1 = __pop_NtAddress(__symbols);
                let __sym0 = __pop_Term_22STORE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action30::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            30 => {
                // Op = "PRINT" => ActionFn(31);
                let __sym0 = __pop_Term_22PRINT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            31 => {
                // Op = "RET" => ActionFn(32);
                let __sym0 = __pop_Term_22RET_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            32 => {
                // Program =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                9
            }
            33 => {
                // Program = Line+ => ActionFn(41);
                let __sym0 = __pop_NtLine_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                9
            }
            34 => {
                // Ref = "CALL", Identifier => ActionFn(12);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22CALL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRef(__nt), __end));
                10
            }
            35 => {
                // Ref = "JMP", Identifier => ActionFn(13);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22JMP_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRef(__nt), __end));
                10
            }
            36 => {
                // Ref = "JIF", Identifier => ActionFn(14);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22JIF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRef(__nt), __end));
                10
            }
            37 => {
                // __Comment = Comment => ActionFn(3);
                let __sym0 = __pop_NtComment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Comment(__nt), __end));
                11
            }
            38 => {
                // __Label = Label => ActionFn(2);
                let __sym0 = __pop_NtLabel(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Label(__nt), __end));
                12
            }
            39 => {
                // __Line = Line => ActionFn(1);
                let __sym0 = __pop_NtLine(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            40 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Program(__nt), __end));
                14
            }
            41 => {
                // __Ref = Ref => ActionFn(4);
                let __sym0 = __pop_NtRef(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Ref(__nt), __end));
                15
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 16 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ADD_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ADD_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22AND_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22AND_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22CALL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22CALL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22DUP_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22DUP_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22HALT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22HALT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISEQ_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISEQ_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISGE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISGE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISGT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISGT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISLT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISLT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22JIF_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22JIF_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22JMP_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22JMP_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22LOAD_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22LOAD_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22MULT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22MULT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22NOT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22NOT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22OR_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22OR_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22POP_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22POP_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22PRINT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22PRINT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22PUSH_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22PUSH_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22RET_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22RET_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22STORE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22STORE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22SUB_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22SUB_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtAddress<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAddress(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComment(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtInteger<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInteger(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLabel<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLabel(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRef<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRef(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Comment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Comment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Label<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Label(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Ref<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Ref(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Line::parse_Line;

mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use instruction::{Instruction, Ref, OpCode};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Term_22_3a_22(&'input str),
        Term_22ADD_22(&'input str),
        Term_22AND_22(&'input str),
        Term_22CALL_22(&'input str),
        Term_22DUP_22(&'input str),
        Term_22HALT_22(&'input str),
        Term_22ISEQ_22(&'input str),
        Term_22ISGE_22(&'input str),
        Term_22ISGT_22(&'input str),
        Term_22ISLT_22(&'input str),
        Term_22JIF_22(&'input str),
        Term_22JMP_22(&'input str),
        Term_22LOAD_22(&'input str),
        Term_22MULT_22(&'input str),
        Term_22NOT_22(&'input str),
        Term_22OR_22(&'input str),
        Term_22POP_22(&'input str),
        Term_22PRINT_22(&'input str),
        Term_22PUSH_22(&'input str),
        Term_22RET_22(&'input str),
        Term_22STORE_22(&'input str),
        Term_22SUB_22(&'input str),
        Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(&'input str),
        NtAddress(usize),
        NtComment(Instruction),
        NtIdentifier(&'input str),
        NtInteger(i32),
        NtLabel(Instruction),
        NtLine(Instruction),
        NtLine_2a(::std::vec::Vec<Instruction>),
        NtLine_2b(::std::vec::Vec<Instruction>),
        NtOp(Instruction),
        NtProgram(Vec<Instruction>),
        NtRef(Instruction),
        Nt____Comment(Instruction),
        Nt____Label(Instruction),
        Nt____Line(Instruction),
        Nt____Program(Vec<Instruction>),
        Nt____Ref(Instruction),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 32,
        // State 1
        0, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, 0, -9,
        // State 2
        33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, -6, 0, -6,
        // State 4
        0, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, 0, -12,
        // State 5
        0, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 0, 32,
        // State 6
        0, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, 0, -7,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, 0, -8,
        // State 9
        0, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, 0, -14,
        // State 10
        0, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20, 0, -20,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32,
        // State 12
        0, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, -23, 0, -23,
        // State 13
        0, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, 0, -15,
        // State 14
        0, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, 0, -24,
        // State 15
        0, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, 0, -26,
        // State 16
        0, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, 0, -25,
        // State 17
        0, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, 0, -27,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0,
        // State 21
        0, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, -18, 0, -18,
        // State 22
        0, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, -19, 0, -19,
        // State 23
        0, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21, 0, -21,
        // State 24
        0, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, -22, 0, -22,
        // State 25
        0, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, 0, -30,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 0,
        // State 27
        0, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, 0, -31,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0,
        // State 29
        0, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, 0, -17,
        // State 30
        0, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, 0, -2,
        // State 31
        -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, 0, -3,
        // State 32
        0, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, -5, 0, -5,
        // State 33
        0, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, 0, -13,
        // State 34
        0, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, 0, -34,
        // State 35
        0, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, 0, -36,
        // State 36
        0, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, 0, -35,
        // State 37
        0, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, 0, -28,
        // State 38
        0, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, 0, -1,
        // State 39
        0, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, 0, -16,
        // State 40
        0, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, 0, -4,
        // State 41
        0, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, 0, -29,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        -32,
        // State 1
        -9,
        // State 2
        0,
        // State 3
        -6,
        // State 4
        -12,
        // State 5
        -33,
        // State 6
        -7,
        // State 7
        -40,
        // State 8
        -8,
        // State 9
        -14,
        // State 10
        -20,
        // State 11
        0,
        // State 12
        -23,
        // State 13
        -15,
        // State 14
        -24,
        // State 15
        -26,
        // State 16
        -25,
        // State 17
        -27,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        -18,
        // State 22
        -19,
        // State 23
        -21,
        // State 24
        -22,
        // State 25
        -30,
        // State 26
        0,
        // State 27
        -31,
        // State 28
        0,
        // State 29
        -17,
        // State 30
        -2,
        // State 31
        -3,
        // State 32
        -5,
        // State 33
        -13,
        // State 34
        -34,
        // State 35
        -36,
        // State 36
        -35,
        // State 37
        -28,
        // State 38
        -1,
        // State 39
        -16,
        // State 40
        -4,
        // State 41
        -29,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 2, 3, 0, 4, 5, 0, 6, 7, 8, 9, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 2, 3, 0, 4, 34, 0, 0, 7, 0, 9, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        42, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"":""###,
            r###""ADD""###,
            r###""AND""###,
            r###""CALL""###,
            r###""DUP""###,
            r###""HALT""###,
            r###""ISEQ""###,
            r###""ISGE""###,
            r###""ISGT""###,
            r###""ISLT""###,
            r###""JIF""###,
            r###""JMP""###,
            r###""LOAD""###,
            r###""MULT""###,
            r###""NOT""###,
            r###""OR""###,
            r###""POP""###,
            r###""PRINT""###,
            r###""PUSH""###,
            r###""RET""###,
            r###""STORE""###,
            r###""SUB""###,
            r###"r#"\'[a-z0-9]+\'"#"###,
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
    >(
        input: &'input str,
    ) -> Result<Vec<Instruction>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
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
                            Token(3, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            Token(4, __tok0) => __Symbol::Term_22ADD_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            Token(5, __tok0) => __Symbol::Term_22AND_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            Token(6, __tok0) => __Symbol::Term_22CALL_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            Token(7, __tok0) => __Symbol::Term_22DUP_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            Token(8, __tok0) => __Symbol::Term_22HALT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            Token(9, __tok0) => __Symbol::Term_22ISEQ_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            Token(10, __tok0) => __Symbol::Term_22ISGE_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            Token(11, __tok0) => __Symbol::Term_22ISGT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            Token(12, __tok0) => __Symbol::Term_22ISLT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token(13, __tok0) => __Symbol::Term_22JIF_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            Token(14, __tok0) => __Symbol::Term_22JMP_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token(15, __tok0) => __Symbol::Term_22LOAD_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            Token(16, __tok0) => __Symbol::Term_22MULT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            Token(17, __tok0) => __Symbol::Term_22NOT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            Token(18, __tok0) => __Symbol::Term_22OR_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            Token(19, __tok0) => __Symbol::Term_22POP_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            Token(20, __tok0) => __Symbol::Term_22PRINT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            Token(21, __tok0) => __Symbol::Term_22PUSH_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            Token(22, __tok0) => __Symbol::Term_22RET_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            Token(23, __tok0) => __Symbol::Term_22STORE_22((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            Token(24, __tok0) => __Symbol::Term_22SUB_22((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            Token(0, __tok0) => __Symbol::Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23((__tok0)),
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
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<Instruction>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Address = r#"[0-9]+"# => ActionFn(34);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAddress(__nt), __end));
                0
            }
            2 => {
                // Comment = r#"\'[a-z0-9]+\'"# => ActionFn(11);
                let __sym0 = __pop_Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComment(__nt), __end));
                1
            }
            3 => {
                // Identifier = r#"[a-zA-Z][a-zA-Z0-9_]*"# => ActionFn(35);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdentifier(__nt), __end));
                2
            }
            4 => {
                // Integer = r#"[0-9]+"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInteger(__nt), __end));
                3
            }
            5 => {
                // Label = Identifier, ":" => ActionFn(10);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLabel(__nt), __end));
                4
            }
            6 => {
                // Line = Label => ActionFn(6);
                let __sym0 = __pop_NtLabel(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            7 => {
                // Line = Op => ActionFn(7);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            8 => {
                // Line = Ref => ActionFn(8);
                let __sym0 = __pop_NtRef(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            9 => {
                // Line = Comment => ActionFn(9);
                let __sym0 = __pop_NtComment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            10 => {
                // Line* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtLine_2a(__nt), __end));
                6
            }
            11 => {
                // Line* = Line+ => ActionFn(37);
                let __sym0 = __pop_NtLine_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine_2a(__nt), __end));
                6
            }
            12 => {
                // Line+ = Line => ActionFn(38);
                let __sym0 = __pop_NtLine(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine_2b(__nt), __end));
                7
            }
            13 => {
                // Line+ = Line+, Line => ActionFn(39);
                let __sym1 = __pop_NtLine(__symbols);
                let __sym0 = __pop_NtLine_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLine_2b(__nt), __end));
                7
            }
            14 => {
                // Op = "ADD" => ActionFn(15);
                let __sym0 = __pop_Term_22ADD_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            15 => {
                // Op = "HALT" => ActionFn(16);
                let __sym0 = __pop_Term_22HALT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            16 => {
                // Op = "PUSH", Integer => ActionFn(17);
                let __sym1 = __pop_NtInteger(__symbols);
                let __sym0 = __pop_Term_22PUSH_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            17 => {
                // Op = "SUB" => ActionFn(18);
                let __sym0 = __pop_Term_22SUB_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            18 => {
                // Op = "MULT" => ActionFn(19);
                let __sym0 = __pop_Term_22MULT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            19 => {
                // Op = "NOT" => ActionFn(20);
                let __sym0 = __pop_Term_22NOT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            20 => {
                // Op = "AND" => ActionFn(21);
                let __sym0 = __pop_Term_22AND_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            21 => {
                // Op = "OR" => ActionFn(22);
                let __sym0 = __pop_Term_22OR_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            22 => {
                // Op = "POP" => ActionFn(23);
                let __sym0 = __pop_Term_22POP_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            23 => {
                // Op = "DUP" => ActionFn(24);
                let __sym0 = __pop_Term_22DUP_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            24 => {
                // Op = "ISEQ" => ActionFn(25);
                let __sym0 = __pop_Term_22ISEQ_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            25 => {
                // Op = "ISGT" => ActionFn(26);
                let __sym0 = __pop_Term_22ISGT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            26 => {
                // Op = "ISGE" => ActionFn(27);
                let __sym0 = __pop_Term_22ISGE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            27 => {
                // Op = "ISLT" => ActionFn(28);
                let __sym0 = __pop_Term_22ISLT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            28 => {
                // Op = "LOAD", Address => ActionFn(29);
                let __sym1 = __pop_NtAddress(__symbols);
                let __sym0 = __pop_Term_22LOAD_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            29 => {
                // Op = "STORE", Address => ActionFn(30);
                let __sym1 = __pop_NtAddress(__symbols);
                let __sym0 = __pop_Term_22STORE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action30::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            30 => {
                // Op = "PRINT" => ActionFn(31);
                let __sym0 = __pop_Term_22PRINT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            31 => {
                // Op = "RET" => ActionFn(32);
                let __sym0 = __pop_Term_22RET_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            32 => {
                // Program =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                9
            }
            33 => {
                // Program = Line+ => ActionFn(41);
                let __sym0 = __pop_NtLine_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                9
            }
            34 => {
                // Ref = "CALL", Identifier => ActionFn(12);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22CALL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRef(__nt), __end));
                10
            }
            35 => {
                // Ref = "JMP", Identifier => ActionFn(13);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22JMP_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRef(__nt), __end));
                10
            }
            36 => {
                // Ref = "JIF", Identifier => ActionFn(14);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22JIF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRef(__nt), __end));
                10
            }
            37 => {
                // __Comment = Comment => ActionFn(3);
                let __sym0 = __pop_NtComment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Comment(__nt), __end));
                11
            }
            38 => {
                // __Label = Label => ActionFn(2);
                let __sym0 = __pop_NtLabel(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Label(__nt), __end));
                12
            }
            39 => {
                // __Line = Line => ActionFn(1);
                let __sym0 = __pop_NtLine(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Line(__nt), __end));
                13
            }
            40 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            41 => {
                // __Ref = Ref => ActionFn(4);
                let __sym0 = __pop_NtRef(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Ref(__nt), __end));
                15
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 16 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ADD_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ADD_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22AND_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22AND_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22CALL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22CALL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22DUP_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22DUP_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22HALT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22HALT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISEQ_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISEQ_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISGE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISGE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISGT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISGT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISLT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISLT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22JIF_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22JIF_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22JMP_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22JMP_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22LOAD_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22LOAD_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22MULT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22MULT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22NOT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22NOT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22OR_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22OR_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22POP_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22POP_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22PRINT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22PRINT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22PUSH_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22PUSH_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22RET_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22RET_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22STORE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22STORE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22SUB_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22SUB_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtAddress<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAddress(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComment(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtInteger<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInteger(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLabel<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLabel(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRef<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRef(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Comment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Comment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Label<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Label(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Ref<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Ref(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Program::parse_Program;

mod __parse__Ref {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use instruction::{Instruction, Ref, OpCode};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Term_22_3a_22(&'input str),
        Term_22ADD_22(&'input str),
        Term_22AND_22(&'input str),
        Term_22CALL_22(&'input str),
        Term_22DUP_22(&'input str),
        Term_22HALT_22(&'input str),
        Term_22ISEQ_22(&'input str),
        Term_22ISGE_22(&'input str),
        Term_22ISGT_22(&'input str),
        Term_22ISLT_22(&'input str),
        Term_22JIF_22(&'input str),
        Term_22JMP_22(&'input str),
        Term_22LOAD_22(&'input str),
        Term_22MULT_22(&'input str),
        Term_22NOT_22(&'input str),
        Term_22OR_22(&'input str),
        Term_22POP_22(&'input str),
        Term_22PRINT_22(&'input str),
        Term_22PUSH_22(&'input str),
        Term_22RET_22(&'input str),
        Term_22STORE_22(&'input str),
        Term_22SUB_22(&'input str),
        Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(&'input str),
        NtAddress(usize),
        NtComment(Instruction),
        NtIdentifier(&'input str),
        NtInteger(i32),
        NtLabel(Instruction),
        NtLine(Instruction),
        NtLine_2a(::std::vec::Vec<Instruction>),
        NtLine_2b(::std::vec::Vec<Instruction>),
        NtOp(Instruction),
        NtProgram(Vec<Instruction>),
        NtRef(Instruction),
        Nt____Comment(Instruction),
        Nt____Label(Instruction),
        Nt____Line(Instruction),
        Nt____Program(Vec<Instruction>),
        Nt____Ref(Instruction),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 4, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        0,
        // State 1
        -41,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        -34,
        // State 6
        -3,
        // State 7
        -36,
        // State 8
        -35,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"":""###,
            r###""ADD""###,
            r###""AND""###,
            r###""CALL""###,
            r###""DUP""###,
            r###""HALT""###,
            r###""ISEQ""###,
            r###""ISGE""###,
            r###""ISGT""###,
            r###""ISLT""###,
            r###""JIF""###,
            r###""JMP""###,
            r###""LOAD""###,
            r###""MULT""###,
            r###""NOT""###,
            r###""OR""###,
            r###""POP""###,
            r###""PRINT""###,
            r###""PUSH""###,
            r###""RET""###,
            r###""STORE""###,
            r###""SUB""###,
            r###"r#"\'[a-z0-9]+\'"#"###,
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
    pub fn parse_Ref<
        'input,
    >(
        input: &'input str,
    ) -> Result<Instruction, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
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
                            Token(3, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            Token(4, __tok0) => __Symbol::Term_22ADD_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            Token(5, __tok0) => __Symbol::Term_22AND_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            Token(6, __tok0) => __Symbol::Term_22CALL_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            Token(7, __tok0) => __Symbol::Term_22DUP_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            Token(8, __tok0) => __Symbol::Term_22HALT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            Token(9, __tok0) => __Symbol::Term_22ISEQ_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            Token(10, __tok0) => __Symbol::Term_22ISGE_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            Token(11, __tok0) => __Symbol::Term_22ISGT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            Token(12, __tok0) => __Symbol::Term_22ISLT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token(13, __tok0) => __Symbol::Term_22JIF_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            Token(14, __tok0) => __Symbol::Term_22JMP_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token(15, __tok0) => __Symbol::Term_22LOAD_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            Token(16, __tok0) => __Symbol::Term_22MULT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            Token(17, __tok0) => __Symbol::Term_22NOT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            Token(18, __tok0) => __Symbol::Term_22OR_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            Token(19, __tok0) => __Symbol::Term_22POP_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            Token(20, __tok0) => __Symbol::Term_22PRINT_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            Token(21, __tok0) => __Symbol::Term_22PUSH_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            Token(22, __tok0) => __Symbol::Term_22RET_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            Token(23, __tok0) => __Symbol::Term_22STORE_22((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            Token(24, __tok0) => __Symbol::Term_22SUB_22((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            Token(0, __tok0) => __Symbol::Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23((__tok0)),
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
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Instruction,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // Address = r#"[0-9]+"# => ActionFn(34);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action34::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtAddress(__nt), __end));
                0
            }
            2 => {
                // Comment = r#"\'[a-z0-9]+\'"# => ActionFn(11);
                let __sym0 = __pop_Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComment(__nt), __end));
                1
            }
            3 => {
                // Identifier = r#"[a-zA-Z][a-zA-Z0-9_]*"# => ActionFn(35);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ_5d_5ba_2dzA_2dZ0_2d9___5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdentifier(__nt), __end));
                2
            }
            4 => {
                // Integer = r#"[0-9]+"# => ActionFn(33);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtInteger(__nt), __end));
                3
            }
            5 => {
                // Label = Identifier, ":" => ActionFn(10);
                let __sym1 = __pop_Term_22_3a_22(__symbols);
                let __sym0 = __pop_NtIdentifier(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action10::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLabel(__nt), __end));
                4
            }
            6 => {
                // Line = Label => ActionFn(6);
                let __sym0 = __pop_NtLabel(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            7 => {
                // Line = Op => ActionFn(7);
                let __sym0 = __pop_NtOp(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action7::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            8 => {
                // Line = Ref => ActionFn(8);
                let __sym0 = __pop_NtRef(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            9 => {
                // Line = Comment => ActionFn(9);
                let __sym0 = __pop_NtComment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine(__nt), __end));
                5
            }
            10 => {
                // Line* =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtLine_2a(__nt), __end));
                6
            }
            11 => {
                // Line* = Line+ => ActionFn(37);
                let __sym0 = __pop_NtLine_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action37::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine_2a(__nt), __end));
                6
            }
            12 => {
                // Line+ = Line => ActionFn(38);
                let __sym0 = __pop_NtLine(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtLine_2b(__nt), __end));
                7
            }
            13 => {
                // Line+ = Line+, Line => ActionFn(39);
                let __sym1 = __pop_NtLine(__symbols);
                let __sym0 = __pop_NtLine_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtLine_2b(__nt), __end));
                7
            }
            14 => {
                // Op = "ADD" => ActionFn(15);
                let __sym0 = __pop_Term_22ADD_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            15 => {
                // Op = "HALT" => ActionFn(16);
                let __sym0 = __pop_Term_22HALT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            16 => {
                // Op = "PUSH", Integer => ActionFn(17);
                let __sym1 = __pop_NtInteger(__symbols);
                let __sym0 = __pop_Term_22PUSH_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action17::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            17 => {
                // Op = "SUB" => ActionFn(18);
                let __sym0 = __pop_Term_22SUB_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            18 => {
                // Op = "MULT" => ActionFn(19);
                let __sym0 = __pop_Term_22MULT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            19 => {
                // Op = "NOT" => ActionFn(20);
                let __sym0 = __pop_Term_22NOT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            20 => {
                // Op = "AND" => ActionFn(21);
                let __sym0 = __pop_Term_22AND_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            21 => {
                // Op = "OR" => ActionFn(22);
                let __sym0 = __pop_Term_22OR_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            22 => {
                // Op = "POP" => ActionFn(23);
                let __sym0 = __pop_Term_22POP_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            23 => {
                // Op = "DUP" => ActionFn(24);
                let __sym0 = __pop_Term_22DUP_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            24 => {
                // Op = "ISEQ" => ActionFn(25);
                let __sym0 = __pop_Term_22ISEQ_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action25::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            25 => {
                // Op = "ISGT" => ActionFn(26);
                let __sym0 = __pop_Term_22ISGT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            26 => {
                // Op = "ISGE" => ActionFn(27);
                let __sym0 = __pop_Term_22ISGE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action27::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            27 => {
                // Op = "ISLT" => ActionFn(28);
                let __sym0 = __pop_Term_22ISLT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action28::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            28 => {
                // Op = "LOAD", Address => ActionFn(29);
                let __sym1 = __pop_NtAddress(__symbols);
                let __sym0 = __pop_Term_22LOAD_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action29::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            29 => {
                // Op = "STORE", Address => ActionFn(30);
                let __sym1 = __pop_NtAddress(__symbols);
                let __sym0 = __pop_Term_22STORE_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action30::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            30 => {
                // Op = "PRINT" => ActionFn(31);
                let __sym0 = __pop_Term_22PRINT_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action31::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            31 => {
                // Op = "RET" => ActionFn(32);
                let __sym0 = __pop_Term_22RET_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action32::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtOp(__nt), __end));
                8
            }
            32 => {
                // Program =  => ActionFn(40);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action40::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                9
            }
            33 => {
                // Program = Line+ => ActionFn(41);
                let __sym0 = __pop_NtLine_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action41::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtProgram(__nt), __end));
                9
            }
            34 => {
                // Ref = "CALL", Identifier => ActionFn(12);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22CALL_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRef(__nt), __end));
                10
            }
            35 => {
                // Ref = "JMP", Identifier => ActionFn(13);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22JMP_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRef(__nt), __end));
                10
            }
            36 => {
                // Ref = "JIF", Identifier => ActionFn(14);
                let __sym1 = __pop_NtIdentifier(__symbols);
                let __sym0 = __pop_Term_22JIF_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action14::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtRef(__nt), __end));
                10
            }
            37 => {
                // __Comment = Comment => ActionFn(3);
                let __sym0 = __pop_NtComment(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Comment(__nt), __end));
                11
            }
            38 => {
                // __Label = Label => ActionFn(2);
                let __sym0 = __pop_NtLabel(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Label(__nt), __end));
                12
            }
            39 => {
                // __Line = Line => ActionFn(1);
                let __sym0 = __pop_NtLine(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Line(__nt), __end));
                13
            }
            40 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_NtProgram(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Program(__nt), __end));
                14
            }
            41 => {
                // __Ref = Ref => ActionFn(4);
                let __sym0 = __pop_NtRef(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 16 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ADD_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ADD_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22AND_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22AND_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22CALL_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22CALL_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22DUP_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22DUP_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22HALT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22HALT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISEQ_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISEQ_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISGE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISGE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISGT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISGT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22ISLT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22ISLT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22JIF_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22JIF_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22JMP_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22JMP_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22LOAD_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22LOAD_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22MULT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22MULT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22NOT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22NOT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22OR_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22OR_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22POP_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22POP_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22PRINT_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22PRINT_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22PUSH_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22PUSH_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22RET_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22RET_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22STORE_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22STORE_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22SUB_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22SUB_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_27_5ba_2dz0_2d9_5d_2b_5c_27_22_23(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtAddress<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, usize, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtAddress(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComment(__v), __r) => (__l, __v, __r),
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
    fn __pop_NtInteger<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtInteger(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLabel<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLabel(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtLine_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtLine_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtOp<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtOp(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtProgram<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtProgram(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtRef<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtRef(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Comment<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Comment(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Label<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Label(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Line<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Line(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Program<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Instruction>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Program(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Ref<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Instruction, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Ref(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Ref::parse_Ref;
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use instruction::{Instruction, Ref, OpCode};
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
                "^(?u:\')(?u:[0-9a-z])+(?u:\')",
                "^(?u:[0-9])+",
                "^(?u:[A-Za-z])(?u:[0-9A-Z_-_a-z])*",
                "^(?u::)",
                "^(?u:ADD)",
                "^(?u:AND)",
                "^(?u:CALL)",
                "^(?u:DUP)",
                "^(?u:HALT)",
                "^(?u:ISEQ)",
                "^(?u:ISGE)",
                "^(?u:ISGT)",
                "^(?u:ISLT)",
                "^(?u:JIF)",
                "^(?u:JMP)",
                "^(?u:LOAD)",
                "^(?u:MULT)",
                "^(?u:NOT)",
                "^(?u:OR)",
                "^(?u:POP)",
                "^(?u:PRINT)",
                "^(?u:PUSH)",
                "^(?u:RET)",
                "^(?u:STORE)",
                "^(?u:SUB)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:\')(?u:[0-9a-z])+(?u:\')").unwrap(),
                __regex::Regex::new("^(?u:[0-9])+").unwrap(),
                __regex::Regex::new("^(?u:[A-Za-z])(?u:[0-9A-Z_-_a-z])*").unwrap(),
                __regex::Regex::new("^(?u::)").unwrap(),
                __regex::Regex::new("^(?u:ADD)").unwrap(),
                __regex::Regex::new("^(?u:AND)").unwrap(),
                __regex::Regex::new("^(?u:CALL)").unwrap(),
                __regex::Regex::new("^(?u:DUP)").unwrap(),
                __regex::Regex::new("^(?u:HALT)").unwrap(),
                __regex::Regex::new("^(?u:ISEQ)").unwrap(),
                __regex::Regex::new("^(?u:ISGE)").unwrap(),
                __regex::Regex::new("^(?u:ISGT)").unwrap(),
                __regex::Regex::new("^(?u:ISLT)").unwrap(),
                __regex::Regex::new("^(?u:JIF)").unwrap(),
                __regex::Regex::new("^(?u:JMP)").unwrap(),
                __regex::Regex::new("^(?u:LOAD)").unwrap(),
                __regex::Regex::new("^(?u:MULT)").unwrap(),
                __regex::Regex::new("^(?u:NOT)").unwrap(),
                __regex::Regex::new("^(?u:OR)").unwrap(),
                __regex::Regex::new("^(?u:POP)").unwrap(),
                __regex::Regex::new("^(?u:PRINT)").unwrap(),
                __regex::Regex::new("^(?u:PUSH)").unwrap(),
                __regex::Regex::new("^(?u:RET)").unwrap(),
                __regex::Regex::new("^(?u:STORE)").unwrap(),
                __regex::Regex::new("^(?u:SUB)").unwrap(),
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
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Instruction>, usize),
) -> Vec<Instruction>
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Instruction, usize),
) -> Instruction
{
    (__0)
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Instruction, usize),
) -> Instruction
{
    (__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Instruction, usize),
) -> Instruction
{
    (__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Instruction, usize),
) -> Instruction
{
    (__0)
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<Instruction>, usize),
) -> Vec<Instruction>
{
    (__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Instruction, usize),
) -> Instruction
{
    (__0)
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Instruction, usize),
) -> Instruction
{
    (__0)
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Instruction, usize),
) -> Instruction
{
    (__0)
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Instruction, usize),
) -> Instruction
{
    (__0)
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::Label(i.to_string())
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, c, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::Comment(c.to_string())
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, i, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::Ref(Ref::Call(i.to_string()))
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, i, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::Ref(Ref::Jmp(i.to_string()))
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, i, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::Ref(Ref::JmpIf(i.to_string()))
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::Add)
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::Halt)
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, i, _): (usize, i32, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::Push(i))
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::Subtract)
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::Multiply)
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::Not)
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::And)
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::Or)
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::Pop)
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::Dup)
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::IsEq)
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::IsGt)
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::IsGe)
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::IsLt)
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, i, _): (usize, usize, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::Load(i))
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, i, _): (usize, usize, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::Store(i))
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::Print)
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Instruction
{
    Instruction::OpCode(OpCode::Ret)
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(s).unwrap()
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    (_, s, _): (usize, &'input str, usize),
) -> usize
{
    usize::from_str(s).unwrap()
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> &'input str
{
    __0
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Instruction>
{
    vec![]
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Instruction>, usize),
) -> ::std::vec::Vec<Instruction>
{
    v
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Instruction, usize),
) -> ::std::vec::Vec<Instruction>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Instruction>, usize),
    (_, e, _): (usize, Instruction, usize),
) -> ::std::vec::Vec<Instruction>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Instruction>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action36(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Instruction>, usize),
) -> Vec<Instruction>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action37(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action5(
        input,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize) {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize),&'static str> {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        value
    }
}

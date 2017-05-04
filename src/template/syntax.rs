use std::str::FromStr;
use template::ast::Token;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Template {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use template::ast::Token;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_2c_22(&'input str),
        Termr_23_22_5b_5b_3aalpha_3a_5d_2d___5d_5b_5b_3aalpha_3a_5d_5c_5cd_2d___5d_2b_22_23(&'input str),
        Termr_23_22_5b_5c_5cd_5d_2b_22_23(&'input str),
        Nt_28_3cVar_3e_20_22_2c_22_29(Token),
        Nt_28_3cVar_3e_20_22_2c_22_29_2a(::std::vec::Vec<Token>),
        Nt_28_3cVar_3e_20_22_2c_22_29_2b(::std::vec::Vec<Token>),
        NtNum(i32),
        NtTemplate(Vec<Token>),
        NtVar(Token),
        NtVar_3f(::std::option::Option<Token>),
        Nt____Template(Vec<Token>),
        Nt____Var(Token),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 5, 0,
        // State 1
        0, 5, 0,
        // State 2
        0, 0, 0,
        // State 3
        7, 0, 0,
        // State 4
        -11, 0, 0,
        // State 5
        8, 0, 0,
        // State 6
        0, -4, 0,
        // State 7
        0, -5, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -8,
        -10,
        -14,
        -7,
        -11,
        -9,
        -4,
        -5,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 2, 0, 3, 4, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 6, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"",""###,
            r###"r#"[[:alpha:]-_][[:alpha:]\\d-_]+"#"###,
            r###"r#"[\\d]+"#"###,
        ];
        __ACTION[(__state * 3)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Template<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<Token>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
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
                (1, _) if true => 0,
                (2, _) if true => 1,
                (0, _) if true => 2,
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
                let __action = __ACTION[__state * 3 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5b_5b_3aalpha_3a_5d_2d___5d_5b_5b_3aalpha_3a_5d_5c_5cd_2d___5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b_5c_5cd_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<Token>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Var> ",") = Var, "," => ActionFn(9);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cVar_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Var> ",")* =  => ActionFn(7);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action7::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cVar_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Var> ",")* = (<Var> ",")+ => ActionFn(8);
                let __sym0 = __pop_Nt_28_3cVar_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cVar_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Var> ",")+ = Var, "," => ActionFn(12);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cVar_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Var> ",")+ = (<Var> ",")+, Var, "," => ActionFn(13);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtVar(__symbols);
                let __sym0 = __pop_Nt_28_3cVar_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cVar_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // Num = r#"[\\d]+"# => ActionFn(4);
                let __sym0 = __pop_Termr_23_22_5b_5c_5cd_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                3
            }
            7 => {
                // Template = Var => ActionFn(16);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTemplate(__nt), __end));
                4
            }
            8 => {
                // Template =  => ActionFn(17);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action17::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtTemplate(__nt), __end));
                4
            }
            9 => {
                // Template = (<Var> ",")+, Var => ActionFn(18);
                let __sym1 = __pop_NtVar(__symbols);
                let __sym0 = __pop_Nt_28_3cVar_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtTemplate(__nt), __end));
                4
            }
            10 => {
                // Template = (<Var> ",")+ => ActionFn(19);
                let __sym0 = __pop_Nt_28_3cVar_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTemplate(__nt), __end));
                4
            }
            11 => {
                // Var = r#"[[:alpha:]-_][[:alpha:]\\d-_]+"# => ActionFn(3);
                let __sym0 = __pop_Termr_23_22_5b_5b_3aalpha_3a_5d_2d___5d_5b_5b_3aalpha_3a_5d_5c_5cd_2d___5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVar(__nt), __end));
                5
            }
            12 => {
                // Var? = Var => ActionFn(5);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVar_3f(__nt), __end));
                6
            }
            13 => {
                // Var? =  => ActionFn(6);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action6::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtVar_3f(__nt), __end));
                6
            }
            14 => {
                // __Template = Template => ActionFn(0);
                let __sym0 = __pop_NtTemplate(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            15 => {
                // __Var = Var => ActionFn(1);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Var(__nt), __end));
                8
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 9 + __nonterminal] - 1;
        __states.push(__next_state);
        None
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
    fn __pop_Termr_23_22_5b_5b_3aalpha_3a_5d_2d___5d_5b_5b_3aalpha_3a_5d_5c_5cd_2d___5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_5b_3aalpha_3a_5d_2d___5d_5b_5b_3aalpha_3a_5d_5c_5cd_2d___5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_5c_5cd_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_5c_5cd_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cVar_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cVar_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cVar_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Token>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cVar_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cVar_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Token>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cVar_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTemplate<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Token>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTemplate(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVar<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVar(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVar_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Token>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVar_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Template<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Token>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Template(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Var<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Var(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Template::parse_Template;

mod __parse__Var {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use template::ast::Token;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_2c_22(&'input str),
        Termr_23_22_5b_5b_3aalpha_3a_5d_2d___5d_5b_5b_3aalpha_3a_5d_5c_5cd_2d___5d_2b_22_23(&'input str),
        Termr_23_22_5b_5c_5cd_5d_2b_22_23(&'input str),
        Nt_28_3cVar_3e_20_22_2c_22_29(Token),
        Nt_28_3cVar_3e_20_22_2c_22_29_2a(::std::vec::Vec<Token>),
        Nt_28_3cVar_3e_20_22_2c_22_29_2b(::std::vec::Vec<Token>),
        NtNum(i32),
        NtTemplate(Vec<Token>),
        NtVar(Token),
        NtVar_3f(::std::option::Option<Token>),
        Nt____Template(Vec<Token>),
        Nt____Var(Token),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 3, 0,
        // State 1
        0, 0, 0,
        // State 2
        0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        0,
        -15,
        -11,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 2, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###"",""###,
            r###"r#"[[:alpha:]-_][[:alpha:]\\d-_]+"#"###,
            r###"r#"[\\d]+"#"###,
        ];
        __ACTION[(__state * 3)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_Var<
        'input,
    >(
        input: &'input str,
    ) -> Result<Token, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
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
                (1, _) if true => 0,
                (2, _) if true => 1,
                (0, _) if true => 2,
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
                let __action = __ACTION[__state * 3 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            (1, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Termr_23_22_5b_5b_3aalpha_3a_5d_2d___5d_5b_5b_3aalpha_3a_5d_5c_5cd_2d___5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b_5c_5cd_5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    };
                    __states.push(__action - 1);
                    __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                    continue '__shift;
                } else if __action < 0 {
                    if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
                if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
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
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Token,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<Var> ",") = Var, "," => ActionFn(9);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action9::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cVar_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<Var> ",")* =  => ActionFn(7);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action7::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cVar_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<Var> ",")* = (<Var> ",")+ => ActionFn(8);
                let __sym0 = __pop_Nt_28_3cVar_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cVar_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<Var> ",")+ = Var, "," => ActionFn(12);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action12::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cVar_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<Var> ",")+ = (<Var> ",")+, Var, "," => ActionFn(13);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtVar(__symbols);
                let __sym0 = __pop_Nt_28_3cVar_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action13::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cVar_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // Num = r#"[\\d]+"# => ActionFn(4);
                let __sym0 = __pop_Termr_23_22_5b_5c_5cd_5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                3
            }
            7 => {
                // Template = Var => ActionFn(16);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTemplate(__nt), __end));
                4
            }
            8 => {
                // Template =  => ActionFn(17);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action17::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtTemplate(__nt), __end));
                4
            }
            9 => {
                // Template = (<Var> ",")+, Var => ActionFn(18);
                let __sym1 = __pop_NtVar(__symbols);
                let __sym0 = __pop_Nt_28_3cVar_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action18::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtTemplate(__nt), __end));
                4
            }
            10 => {
                // Template = (<Var> ",")+ => ActionFn(19);
                let __sym0 = __pop_Nt_28_3cVar_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTemplate(__nt), __end));
                4
            }
            11 => {
                // Var = r#"[[:alpha:]-_][[:alpha:]\\d-_]+"# => ActionFn(3);
                let __sym0 = __pop_Termr_23_22_5b_5b_3aalpha_3a_5d_2d___5d_5b_5b_3aalpha_3a_5d_5c_5cd_2d___5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVar(__nt), __end));
                5
            }
            12 => {
                // Var? = Var => ActionFn(5);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtVar_3f(__nt), __end));
                6
            }
            13 => {
                // Var? =  => ActionFn(6);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action6::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtVar_3f(__nt), __end));
                6
            }
            14 => {
                // __Template = Template => ActionFn(0);
                let __sym0 = __pop_NtTemplate(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt____Template(__nt), __end));
                7
            }
            15 => {
                // __Var = Var => ActionFn(1);
                let __sym0 = __pop_NtVar(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action1::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 9 + __nonterminal] - 1;
        __states.push(__next_state);
        None
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
    fn __pop_Termr_23_22_5b_5b_3aalpha_3a_5d_2d___5d_5b_5b_3aalpha_3a_5d_5c_5cd_2d___5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_5b_3aalpha_3a_5d_2d___5d_5b_5b_3aalpha_3a_5d_5c_5cd_2d___5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b_5c_5cd_5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_5c_5cd_5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cVar_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cVar_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cVar_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Token>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cVar_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cVar_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Token>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cVar_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, i32, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTemplate<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Token>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTemplate(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVar<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVar(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtVar_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<Token>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtVar_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Template<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Token>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Template(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Var<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Token, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Var(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Var::parse_Var;
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
                "^(?u:[0-9٠-٩۰-۹߀-߉०-९০-৯੦-੯૦-૯୦-୯௦-௯౦-౯೦-೯൦-൯෦-෯๐-๙໐-໙༠-༩၀-၉႐-႙០-៩᠐-᠙᥆-᥏᧐-᧙᪀-᪉᪐-᪙᭐-᭙᮰-᮹᱀-᱉᱐-᱙꘠-꘩꣐-꣙꤀-꤉꧐-꧙꧰-꧹꩐-꩙꯰-꯹０-９𐒠-𐒩𑁦-𑁯𑃰-𑃹𑄶-𑄿𑇐-𑇙𑋰-𑋹𑓐-𑓙𑙐-𑙙𑛀-𑛉𑜰-𑜹𑣠-𑣩𖩠-𖩩𖭐-𖭙𝟎-𝟿])+",
                "^(?u:,)",
                "^(?u:[-A-Z_-_a-z])(?u:[-0-9A-Z_-_a-z٠-٩۰-۹߀-߉०-९০-৯੦-੯૦-૯୦-୯௦-௯౦-౯೦-೯൦-൯෦-෯๐-๙໐-໙༠-༩၀-၉႐-႙០-៩᠐-᠙᥆-᥏᧐-᧙᪀-᪉᪐-᪙᭐-᭙᮰-᮹᱀-᱉᱐-᱙꘠-꘩꣐-꣙꤀-꤉꧐-꧙꧰-꧹꩐-꩙꯰-꯹０-９𐒠-𐒩𑁦-𑁯𑃰-𑃹𑄶-𑄿𑇐-𑇙𑋰-𑋹𑓐-𑓙𑙐-𑙙𑛀-𑛉𑜰-𑜹𑣠-𑣩𖩠-𖩩𖭐-𖭙𝟎-𝟿])+",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:[0-9٠-٩۰-۹߀-߉०-९০-৯੦-੯૦-૯୦-୯௦-௯౦-౯೦-೯൦-൯෦-෯๐-๙໐-໙༠-༩၀-၉႐-႙០-៩᠐-᠙᥆-᥏᧐-᧙᪀-᪉᪐-᪙᭐-᭙᮰-᮹᱀-᱉᱐-᱙꘠-꘩꣐-꣙꤀-꤉꧐-꧙꧰-꧹꩐-꩙꯰-꯹０-９𐒠-𐒩𑁦-𑁯𑃰-𑃹𑄶-𑄿𑇐-𑇙𑋰-𑋹𑓐-𑓙𑙐-𑙙𑛀-𑛉𑜰-𑜹𑣠-𑣩𖩠-𖩩𖭐-𖭙𝟎-𝟿])+").unwrap(),
                __regex::Regex::new("^(?u:,)").unwrap(),
                __regex::Regex::new("^(?u:[-A-Z_-_a-z])(?u:[-0-9A-Z_-_a-z٠-٩۰-۹߀-߉०-९০-৯੦-੯૦-૯୦-୯௦-௯౦-౯೦-೯൦-൯෦-෯๐-๙໐-໙༠-༩၀-၉႐-႙០-៩᠐-᠙᥆-᥏᧐-᧙᪀-᪉᪐-᪙᭐-᭙᮰-᮹᱀-᱉᱐-᱙꘠-꘩꣐-꣙꤀-꤉꧐-꧙꧰-꧹꩐-꩙꯰-꯹０-９𐒠-𐒩𑁦-𑁯𑃰-𑃹𑄶-𑄿𑇐-𑇙𑋰-𑋹𑓐-𑓙𑙐-𑙙𑛀-𑛉𑜰-𑜹𑣠-𑣩𖩠-𖩩𖭐-𖭙𝟎-𝟿])+").unwrap(),
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
                    for __i in 0 .. 3 {
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
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Token>, usize),
) -> Vec<Token>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Token, usize),
) -> Token
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Token>, usize),
    (_, e, _): (usize, ::std::option::Option<Token>, usize),
) -> Vec<Token>
{
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
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Token
{
    Token::Var(String::from(__0))
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> i32
{
    i32::from_str(__0).unwrap()
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Token, usize),
) -> ::std::option::Option<Token>
{
    Some(__0)
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<Token>
{
    None
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Token>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Token>, usize),
) -> ::std::vec::Vec<Token>
{
    v
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Token, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Token
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Token, usize),
) -> ::std::vec::Vec<Token>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action11<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Token>, usize),
    (_, e, _): (usize, Token, usize),
) -> ::std::vec::Vec<Token>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action12<
    'input,
>(
    input: &'input str,
    __0: (usize, Token, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<Token>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action9(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action10(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action13<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Token>, usize),
    __1: (usize, Token, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<Token>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action9(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action11(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action14<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<Token>, usize),
) -> Vec<Token>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action7(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
pub fn __action15<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Token>, usize),
    __1: (usize, ::std::option::Option<Token>, usize),
) -> Vec<Token>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action8(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action2(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
pub fn __action16<
    'input,
>(
    input: &'input str,
    __0: (usize, Token, usize),
) -> Vec<Token>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action5(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action17<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Token>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action6(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action14(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action18<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Token>, usize),
    __1: (usize, Token, usize),
) -> Vec<Token>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action5(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action19<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Token>, usize),
) -> Vec<Token>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action6(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action15(
        input,
        __0,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, (usize, &'input str), usize) {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, (usize, &'input str), usize),()> {
    type Error = ();
    fn to_triple(value: Self) -> Result<(usize,(usize, &'input str),usize),()> {
        value
    }
}

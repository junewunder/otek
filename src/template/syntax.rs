use std::str::FromStr;
use template::ast::vec_to_string;
extern crate lalrpop_util as __lalrpop_util;

mod __parse__File {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports)]

    use std::str::FromStr;
    use template::ast::vec_to_string;
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(dead_code)]
    pub enum __Symbol<'input> {
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Termr_23_22_5b_5b_3aascii_3a_5d_5d_2a_22_23(&'input str),
        NtFile(Vec<String>),
        NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e(String),
        NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e_2a(::std::vec::Vec<String>),
        NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e_2b(::std::vec::Vec<String>),
        NtText(String),
        Nt____File(Vec<String>),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 6,
        // State 1
        0, 0, 0,
        // State 2
        0, 0, -6,
        // State 3
        0, 0, 6,
        // State 4
        8, 0, 0,
        // State 5
        -8, 0, 0,
        // State 6
        0, 0, -7,
        // State 7
        0, 0, 10,
        // State 8
        0, 11, 0,
        // State 9
        0, -8, 0,
        // State 10
        0, 0, -3,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        -1,
        -9,
        -6,
        -2,
        0,
        0,
        -7,
        0,
        0,
        0,
        -3,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        2, 3, 0, 4, 5, 0,
        // State 1
        0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0,
        // State 3
        0, 7, 0, 0, 5, 0,
        // State 4
        0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 9, 0,
        // State 8
        0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""(""###,
            r###"")""###,
            r###"r#"[[:ascii:]]*"#"###,
        ];
        __ACTION[(__state * 3)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub fn parse_File<
        'input,
    >(
        input: &'input str,
    ) -> Result<Vec<String>, __lalrpop_util::ParseError<usize, (usize, &'input str), ()>>
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
                            (1, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            (2, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            (0, __tok0) => __Symbol::Termr_23_22_5b_5b_3aascii_3a_5d_5d_2a_22_23((__tok0)),
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
    ) -> Option<Result<Vec<String>,__lalrpop_util::ParseError<usize, (usize, &'input str), ()>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // File =  => ActionFn(8);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action8::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtFile(__nt), __end));
                0
            }
            2 => {
                // File = TemplatePiece<"(", ")">+ => ActionFn(9);
                let __sym0 = __pop_NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtFile(__nt), __end));
                0
            }
            3 => {
                // TemplatePiece<"(", ")"> = Text, "(", Text, ")" => ActionFn(5);
                let __sym3 = __pop_Term_22_29_22(__symbols);
                let __sym2 = __pop_NtText(__symbols);
                let __sym1 = __pop_Term_22_28_22(__symbols);
                let __sym0 = __pop_NtText(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action5::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e(__nt), __end));
                1
            }
            4 => {
                // TemplatePiece<"(", ")">* =  => ActionFn(3);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action3::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e_2a(__nt), __end));
                2
            }
            5 => {
                // TemplatePiece<"(", ")">* = TemplatePiece<"(", ")">+ => ActionFn(4);
                let __sym0 = __pop_NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action4::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e_2a(__nt), __end));
                2
            }
            6 => {
                // TemplatePiece<"(", ")">+ = TemplatePiece<"(", ")"> => ActionFn(6);
                let __sym0 = __pop_NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action6::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e_2b(__nt), __end));
                3
            }
            7 => {
                // TemplatePiece<"(", ")">+ = TemplatePiece<"(", ")">+, TemplatePiece<"(", ")"> => ActionFn(7);
                let __sym1 = __pop_NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e(__symbols);
                let __sym0 = __pop_NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e_2b(__nt), __end));
                3
            }
            8 => {
                // Text = r#"[[:ascii:]]*"# => ActionFn(2);
                let __sym0 = __pop_Termr_23_22_5b_5b_3aascii_3a_5d_5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action2::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtText(__nt), __end));
                4
            }
            9 => {
                // __File = File => ActionFn(0);
                let __sym0 = __pop_NtFile(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 6 + __nonterminal] - 1;
        __states.push(__next_state);
        None
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
    fn __pop_Termr_23_22_5b_5b_3aascii_3a_5d_5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b_5b_3aascii_3a_5d_5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtFile<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtFile(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtTemplatePiece_3c_22_28_22_2c_20_22_29_22_3e_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtText<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtText(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____File<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<String>, usize) {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____File(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__File::parse_File;
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
                "^(?u:[\u{0}-\u{7f}])*",
                "^(?u:\\()",
                "^(?u:\\))",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:[\u{0}-\u{7f}])*").unwrap(),
                __regex::Regex::new("^(?u:\\()").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
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
    (_, __0, _): (usize, Vec<String>, usize),
) -> Vec<String>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<String>, usize),
) -> Vec<String>
{
    (__0)
}

#[allow(unused_variables)]
pub fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    String::from(__0)
    //vec_to_string(&tvec)
        // tvec.into_iter()
        //     .fold(String::new(), |mut acc, l| { acc.push_str(&l); acc })
}

#[allow(unused_variables)]
pub fn __action3<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<String>
{
    vec![]
}

#[allow(unused_variables)]
pub fn __action4<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<String>, usize),
) -> ::std::vec::Vec<String>
{
    v
}

#[allow(unused_variables)]
pub fn __action5<
    'input,
>(
    input: &'input str,
    (_, text, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, replacement, _): (usize, String, usize),
    (_, _, _): (usize, &'input str, usize),
) -> String
{
    {
        let mut out = String::new();

        out.push_str(text.as_str());
        out.push_str(" ,,,,,,,, ");
        out.push_str(replacement.as_str());

        // if let Some(replacement) = Some(r) {
        //     out.push_str(replacement.as_str());
        // }

        out
    }
}

#[allow(unused_variables)]
pub fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> ::std::vec::Vec<String>
{
    vec![__0]
}

#[allow(unused_variables)]
pub fn __action7<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<String>, usize),
    (_, e, _): (usize, String, usize),
) -> ::std::vec::Vec<String>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
pub fn __action8<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<String>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action3(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
pub fn __action9<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<String>, usize),
) -> Vec<String>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action4(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
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

// use nom::anychar;
// use nom::IResult;
use nom;
use regex::Regex;

use ::state::OtekState;

// named!(replacement< Vec<char> >, delimited!(
//         char!('<'),
//         many0!(anychar),
//         char!('>')
//     )
// );

named!(date<Vec<&[u8]> >, re_bytes_matches!(r"\d{4}-\d{2}-\d{2}"));


// named!(template<Vec<u8> >)

#[test]
fn test_syntax() {
    // named!(date<Vec<&[u8]> >, re_bytes_matches!(r"\d{4}-\d{2}-\d{2}"));

    println!("{:?}", find_templates(&b"something (a)"[..]));

    // assert_eq!(
    //     find_templates(&b"something (a)"[..]),
    //     IResult::Done(&b""[..],
    //     vec![&b"a"[..]])
    // );

    // println!("{:?}", date(&b"2015-09-07"[..]));
    // assert_eq!(date(&b"2015-09-07"[..]), IResult::Done(&b""[..], vec![&b"2015-09-07"[..]]));
}

// println!("THING: {:?}", find_templates(b"<wf wef s fgergse gs gera>"));
// println!("THING: {:?}", find_templates(b"<a>"));
// println!("THING: {:?}", date(&b"2015-09-07"[..]));

// assert_eq!(replacement(b"<a>"),   IResult::Done(&b""[..], vec!['a']))

// fn make_macros(start_token: &[u8], end_token: &[u8]) {
// named!(find_templates<Vec<&[u8]> >,
//     re_bytes_matches!(
//         start_token.clone() + r".*\)" + end_token.clone()
//     )
// );
// }


pub struct OtekSyntax<'a> {
    start_token: &'a[u8],
    end_token: &'a[u8],
    byte_step: usize,
    state: OtekState,
}

impl<'a> OtekSyntax<'a> {

    pub fn new(start_token: &'a[u8], end_token: &'a[u8], state: OtekState) -> OtekSyntax<'a> {

        let byte_step = if start_token.len() > end_token.len() { start_token.len() } else { end_token.len() };



        OtekSyntax {
            start_token: start_token,
            end_token: end_token,
            byte_step: byte_step,
            state: state,
        }
    }

    pub fn parse(&self) {

    }

}

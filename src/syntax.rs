use nom;
use nom::anychar;
use nom::IResult;
use nom::be_u8;

use regex::Regex;

named!(replacement<&[u8]>, delimited!(
        char!('<'),
        many0!(anychar),
        char!('>')
    )
);

// named!(date<Vec<&[u8]> >, re_bytes_matches!(r"\d{4}-\d{2}-\d{2}"));

named!(text <&[u8]>, many0!(anychar));




named!(template<&[u8]>, do_parse!(
    fold_many0!(
        text, replacement
    )
));

named!(tag_length_value<(u8, &[u8])>,
  do_parse!(
    tag!( &[ 42u8 ][..] ) >>
    length: be_u8         >>
    bytes:  take!(length) >>
    (length, bytes)
  )
);

#[test]
fn test_syntax() {

    println!("{:?}", template(&b"something (a)"[..]));

}

use template::ast::TextType;

/// Bell char is open symbol
pub const OPEN: char = 7u8 as char;

/// Cancel char is close symbol
pub const CLOSE: char = 18u8 as char;

/// Changes open and close flags to characters that are understood by the parser
pub fn replace_flags(mut s: String, open: &String, close: &String) -> String {

    s = str::replace(s.as_str(), open.as_str(), OPEN.to_string().as_str());
    str::replace(s.as_str(), close.as_str(), CLOSE.to_string().as_str())

}

pub fn split_at_flags(input: String) -> Vec<TextType> {

    let mut output = Vec::new();
    let mut current = String::new();

    for l in input.chars() {
        match l {

            OPEN => {
                if current.len() > 0 {
                    output.push(TextType::Text(current.clone()));
                }
                current = String::new();
            },

            CLOSE => {
                if current.len() > 0 {
                    output.push(TextType::Template(current.clone()));
                }
                current = String::new();
            },

            _ => current.push(l),
        }
    };

    if current.len() > 0 {
        output.push(TextType::Text(current.clone()));
    }


    output

}

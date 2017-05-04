// pub mod ast;
mod ast;
mod syntax;
mod preprocess;

pub fn process(mut file_contents: String) {

    file_contents = preprocess::replace_flags(
        file_contents,
        &"(((".to_string(),
        &")))".to_string()
    );

    let text_pieces = preprocess::split_at_flags(file_contents);

    println!("{:?}", text_pieces);

    for text in text_pieces {
        use template::ast::TextType::*;
        match text {
            Text(s) => println!("{:?}", s),
            Template(t) => {
                // println!("{:?}, {:?}", t, t.chars());
                println!("{:?}", syntax::parse_Template(t.as_str()))
            },
        }

    }
}

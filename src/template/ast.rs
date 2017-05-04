
#[derive(Debug)]
pub enum Token {

    Var(String),

    Or,

    QuotedText(String),

    // Var_Or(Option<Box<Token>>, String),

}

#[derive(Debug)]
pub enum TextType {
    Text(String),
    Template(String),
}

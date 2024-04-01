use std::fmt;


#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy)]
pub enum TokenType {
    NULL,
    Star,
    Slash,
    Plus,
    Minus,

    Equals,

    LeftPara,
    RightPara,
    Caret,


    Number,
    Variable,
    Function,
}




#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Token {
    pub token_type: TokenType,
    pub token_value: String, 
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token_value)
    }
}

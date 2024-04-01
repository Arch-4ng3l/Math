use super::{lexer, parser, types::Expression};

pub struct Translator {
}
pub fn new() -> Translator {
    return Translator{}
}

impl Translator {
    pub fn translate(&self, input: String) -> Expression {
        let tokens = lexer::run(input);
        return parser::run(tokens);
    }
}

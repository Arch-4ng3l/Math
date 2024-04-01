use std::fmt::Display;

use crate::preprocess::token::Token;

use super::token::TokenType;


#[derive(Debug, Clone, Eq, PartialEq)] 
pub enum Expression {
        Mul {left: Box<Expression>, right: Box<Expression>}, 
        Add {left: Box<Expression>, right: Box<Expression>}, 

        AddFlat { args: Vec<Box<Expression>>},
        MulFlat { args: Vec<Box<Expression>>},

        //Fraction {left: Box<Expression>, right: Box<Expression>},
        Function {token: Token, input: Box<Expression>},
        Variable(Token),

        VarSimple{token: Token, mult: Box<Expression>},

        Pow {base: Box<Expression>, exponent: Box<Expression>},
        Number(Token),
        NONE,
}

pub struct Equation {
        pub left: Expression,
        pub right: Expression
}

impl Display for Equation {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(f, "{}={}", self.left, self.right)
        }
}



impl Display for Expression {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                        Expression::Mul {left, right} => {
                                write!(f, "({}*{})", *left, *right)
                        }
                        Expression::Add { left, right } => {
                                write!(f, "({}+{})", *left, *right)
                        }
                        Expression::Pow { base, exponent } => {
                                write!(f, "({}^{})", *base, *exponent)
                        }
                        Expression::Number(t) => {
                                write!(f, "{}", t)
                        }
                        Expression::Variable(t) => {
                                write!(f, "{}", t)
                        }
                        Expression::Function { token, input } => {
                                write!(f, "{}({})", token.token_value, *input)
                        }
                        Expression::VarSimple { token, mult } => {
                                write!(f, "{}{}", mult, token)
                        }
                        Expression::AddFlat { args } => {
                                let s = args
                                        .iter()
                                        .map(|x| format!("{}", x))
                                        .collect::<Vec<String>>()
                                        .join("+");
                                write!(f, "{}", s)
                        }
                        Expression::MulFlat { args } => {
                                let s = args
                                        .iter()
                                        .map(|x| format!("{}", x))
                                        .collect::<Vec<String>>()
                                        .join("*");
                                write!(f, "{}", s)
                        }
                        Expression::NONE => {
                                write!(f, "(NONE)")
                        }
                }
        }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
        None, 
        Add,
        Mul, 
        Pow, 
        Par,
        Call,
}

pub fn one() -> Expression { 
        Expression::Number(
                Token{
                        token_type: super::token::TokenType::Number,
                        token_value: "1".to_string(),
                }
        )
}
pub fn negative_one() -> Expression { 
        Expression::Number(
                Token{
                        token_type: super::token::TokenType::Number,
                        token_value: "-1".to_string(),
                }
        )
}
pub fn zero() -> Expression { 
        Expression::Number(
                Token{
                        token_type: super::token::TokenType::Number,
                        token_value: "0".to_string(),
                }
        )
}

pub fn create_function(name: &str, input: Expression) -> Expression {
        Expression::Function{
                token: Token{
                        token_type: TokenType::Function,
                        token_value: name.to_string(),
                },
                input: Box::from(input)
        }
}

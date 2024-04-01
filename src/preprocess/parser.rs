use crate::preprocess::{token::{Token, TokenType}, types::{Expression, Precedence}};

use super::types::negative_one;


struct Parser {
    tokens: Vec<Token>,
    cur_token: Token,
    next_token: Token,
    idx: usize,

}

fn token_to_precedence(token: Token) -> Precedence {
    return match token.token_type {
        TokenType::Star => Precedence::Mul,
        TokenType::Slash => Precedence::Mul,
        TokenType::Plus => Precedence::Add,
        TokenType::Minus => Precedence::Add,
        TokenType::LeftPara => Precedence::Par,
        TokenType::Caret => Precedence::Pow,
        _ => Precedence::None

    }
}

pub fn run(tokens: Vec<Token>) -> Expression{
    let p: Parser = Parser{
        cur_token: tokens[0].clone(),
        next_token: tokens[1].clone(),
        tokens,
        idx: 1,
    };
    return p.parse();
}

impl Parser {
    fn shift_token(&mut self) {
        self.cur_token = self.next_token.clone();
        self.idx +=1;
        self.next_token = self.tokens[self.idx].clone();
    }

    pub fn parse(mut self) -> Expression {
        let exp = self.parse_expression(Precedence::None);
        return exp;

    }


    fn parse_expression(&mut self, precedence: Precedence) -> Expression{
        let mut left = self.parse_minus();

        while precedence < token_to_precedence(self.next_token.clone()) {
            self.shift_token();
            left = self.parse_infix(left.clone());
        }

        return left;
    }

    fn parse_function(&mut self) -> Expression{
        let func_token = self.cur_token.clone();
        self.shift_token();
        self.shift_token();
        let input = self.parse_expression(Precedence::None);
        self.shift_token();

        return Expression::Function{
            token: func_token,
            input: Box::from(input),
        }
    }
    fn parse_minus(&mut self) -> Expression{
        match self.cur_token.token_type {
            TokenType::Number => {
                return Expression::Number(self.cur_token.clone());
            }
            TokenType::Variable => {
                return Expression::Variable(self.cur_token.clone());
            }
            TokenType::Function => {
                return self.parse_function()
            }
            TokenType::LeftPara => {
                self.shift_token();
                let val = self.parse_expression(Precedence::None);
                self.shift_token();
                return val
            }
            _ => {}
        }
        return match self.next_token.token_type {
            TokenType::Number => {
                Expression::Mul{
                    left: Box::from(negative_one()),
                    right: Box::from(Expression::Number(self.next_token.clone()))
                }
            }
            TokenType::Variable => {
                Expression::Mul{
                    left: Box::from(negative_one()),
                    right: Box::from(Expression::Number(self.next_token.clone()))
                }
            }
            TokenType::Function => {
                Expression::Mul{
                    left: Box::from(negative_one()),
                    right: Box::from(Expression::Function {
                        token: self.next_token.clone(),
                        input: Box::from(self.parse_expression(Precedence::Call))
                    })
                }
            }
            _ => {
                Expression::NONE
            }

        }
    }
    fn parse_infix(&mut self, left: Expression) -> Expression {
        return match self.cur_token.token_type {
            TokenType::Star => {
                self.shift_token();
                Expression::Mul{
                    left: Box::from(left),
                    right: Box::from(self.parse_expression(Precedence::Mul)),
                }

            }
            TokenType::Minus => {
                self.shift_token();
                Expression::Add{
                    left: Box::from(left),
                    right: Box::from(Expression::Mul{
                        left: Box::from(negative_one()),
                        right: Box::from(self.parse_expression(Precedence::Add))
                    })
                }
            }
            TokenType::Plus => {
                self.shift_token();
                Expression::Add{
                    left: Box::from(left),
                    right: Box::from(self.parse_expression(Precedence::Add))
                }
            }
            TokenType::Slash => {
                self.shift_token();
                Expression::Mul{
                    left: Box::from(left),
                    right: Box::from(Expression::Pow{
                        base: Box::from(self.parse_expression(Precedence::Mul)),
                        exponent: Box::from(negative_one())
                    })
                }
            }
            TokenType::Caret => {
                self.shift_token();
                Expression::Pow{
                    base:Box::from(left),
                    exponent: Box::from(self.parse_expression(Precedence::Pow))
                }
            }
            _ => {
                left
            }
        }
    }
}

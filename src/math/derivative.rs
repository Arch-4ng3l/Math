use crate::preprocess::{token::{Token, TokenType}, types::{create_function, negative_one, one, zero, Expression}};

use super::simplify::simplify;


fn get_function_der(exp: Expression) ->Expression {
    match &exp {
        Expression::Function { token, input } => {
            match token.token_value.as_str() {
                "exp" => Expression::Mul {
                    left: Box::from(derivative(*input.clone())),
                    right: Box::from(exp.clone())
                },
                "ln" => Expression::Mul {
                    left: Box::from(derivative(*input.clone())),
                    right: Box::from(Expression::Pow{
                        base: Box::from(*input.clone()),
                        exponent: Box::from(negative_one())
                    })
                },
                "cos" => Expression::Mul {
                    left: Box::from(derivative(*input.clone())),
                    right: Box::from(Expression::Mul{
                        left: Box::from(negative_one()),
                        right: Box::from(create_function("sin", *input.clone()))
                    })
                },
                "sin" => Expression::Mul {
                    left: Box::from(derivative(*input.clone())),
                    right: Box::from(create_function("cos", *input.clone()))
                },
                "tan" => Expression::Mul {
                    left: Box::from(derivative(*input.clone())),
                    right: Box::from(Expression::Pow {
                        base: Box::from(create_function("sec", *input.clone())),
                        exponent: Box::from(Expression::Number(
                            Token {
                                token_value: "2".to_string(),
                                token_type: TokenType::Number
                            }
                        ))
                    })
                },
                _ => Expression::NONE
            }
        }
        _ => {
            return Expression::NONE
        }
    }
}



fn is_der(exp: Expression) -> bool {
    match exp {
        Expression::Pow { .. } | Expression::Function { .. } | Expression::Variable(_) | Expression::VarSimple { .. } => {
            return true;
        }
        _ => {
            return false;
        }
    }
}

fn is_variable(exp: Expression) -> bool {
    match exp {
        Expression::Variable(_)| Expression::VarSimple { .. }  => {
            return true
        }
        _ => {
            return false
        }
    }

}


pub fn derivative(exp: Expression) -> Expression{
    match exp {
        Expression::Function { .. } => {
            let func = get_function_der(exp);
            return func;
        }
        Expression::Number(_) => {
            return zero()
        }
        Expression::Variable(_) => {
            return one()
        }
        Expression::VarSimple { mult, .. } => {
            return *mult;
        }

        // u * v' + u' * v
        Expression::Mul { left, right } => {
            if !(is_der(*left.clone()) && is_der(*right.clone())) {
                if is_der(*left.clone()) {
                    return simplify(Expression::Mul{
                        left: Box::from(derivative(*left.clone())),
                        right: Box::from(*right.clone()),
                    })
                } else if is_der(*right.clone()) {
                    return simplify(Expression::Mul{
                        left: Box::from(*left.clone()),
                        right: Box::from(derivative(*right.clone())),
                    })
                }
            }
            let e = Expression::Add {
                left: Box::from(Expression::Mul{
                    left: Box::from(*left.clone()),
                    right: Box::from(derivative(*right.clone()))
                }),
                right: Box::from(Expression::Mul{
                    left: Box::from(derivative(*left.clone())),
                    right: Box::from(*right.clone())
                })
            };
            return e;
        }
        Expression::Pow { base, exponent } => {
            match (&*base, &*exponent) {
                (Expression::Variable(_), e) => {
                    if !is_variable(e.clone()) {
                        return simplify(Expression::Mul{
                            left: exponent.clone(),
                            right: Box::from(Expression::Pow{
                                base: base.clone(),
                                exponent: Box::from(simplify(Expression::Add{
                                    left: exponent,
                                    right: Box::from(negative_one())
                                }))
                            })
                        }) 
                    } 
                    derivative(create_function("exp", Expression::Mul{
                        left: exponent,
                        right: Box::from(create_function("ln", *base))
                    }))
                }
                _ => {
                    Expression::NONE
                }
            }
        }
        Expression::AddFlat { args } => {
            let args: Vec<Box<Expression>> = args
                .iter()
                .map(|x| Box::from(derivative(*x.clone())))
                .collect();
            return Expression::AddFlat{args};
        }
        Expression::Add { left, right } => {
            return Expression::Add {
                left: Box::from(derivative(*left)),
                right: Box::from(derivative(*right)),
            }
        }
        Expression::MulFlat { args } => {
            let mut sum: Vec<Box<Expression>> = Vec::new();
            let (numbers, expressions): (Vec<_>, Vec<_>) = args
                .iter()
                .map(|x| *x.clone())
                .partition(|x| match x {
                    Expression::Number(_) => {
                        true
                    }
                    _ => {
                        false
                    }
                });

            let len = expressions.len();
            for i in 0..len {
                let deriv = expressions[i].clone();
                let mut factors: Vec<Box<Expression>> = expressions
                    .iter()
                    .enumerate()
                    .filter(|&(j, _)| j != i)
                    .map(|(_, e)| Box::from(e.clone()))
                    .collect()

                ;
                factors.push(Box::from(derivative(deriv)));
                sum.push(Box::from(simplify(Expression::MulFlat {args: factors})));
            }
            if numbers.len() > 0 {
                let mut i = 1.0;
                for num in numbers {
                    match num {
                        Expression::Number(t) => {
                            i *= t.token_value.parse::<f64>().unwrap();
                        }
                        _ => {}
                    }
                }
                let term = simplify(Expression::Mul { 
                    left: Box::from(Expression::Number(Token{
                        token_value: format!("{}", i),
                        token_type: TokenType::Number
                    })),
                    right: Box::from(Expression::AddFlat { args: sum } )
                });
                return term
                
            }
            return Expression::AddFlat { args: sum }
        }

        _ => {
            Expression::NONE
        }

    }
}

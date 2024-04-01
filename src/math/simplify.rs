use std::collections::HashMap;

use crate::preprocess::{token::{Token, TokenType}, types::{create_function, one, Expression}};

// Simplification Rules
//
// 3*x + ... + 4*x = 7*x + ...
// x + x
// 
//
//
//
fn is_exponent(exp: Expression) -> bool {
    match exp {
        Expression::Pow { .. } => {
            return true;
        }
        _ => {
            return false;
        }
    }
}

fn reduce_inverse(token: Token, input: Expression) -> Expression{
    match token.token_value.as_str() {
        "ln" => {
            match input {
                Expression::Function { token: token2, input: input2 }  => {
                    if token2.token_value.as_str() == "exp" {
                        return *input2.clone()
                    }

                }
                _ => {}
            }
        }
        "exp" => {
            match input {
                Expression::Function { token: token2, input: input2 }  => {
                    if token2.token_value.as_str() == "ln" {
                        return *input2.clone()
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
    return Expression::NONE
}
//
//
pub fn simplify(exp: Expression) -> Expression{
    return match exp {
        Expression::Pow { base, exponent } => {
            let base = simplify(*base);
            let exponent = simplify(*exponent);
            match (&base, &exponent) {
                (Expression::Pow { base: base2, exponent: exponent2 }, _) => {
                    return simplify(Expression::Pow {
                        base: Box::from(*base2.clone()),
                        exponent: Box::from(simplify(Expression::Mul {
                            left: Box::from(exponent),
                            right: Box::from(*exponent2.clone()),
                        }))
                    })
                }
                (Expression::Number(t), Expression::Number(t2)) => {
                    let x = t.token_value.parse::<f64>().unwrap();
                    let y = t2.token_value.parse::<f64>().unwrap();
                    return Expression::Number(Token{
                        token_type: TokenType::Number,
                        token_value: x.powf(y).to_string()
                    })
                }
                (Expression::Function { token, input }, e) => {
                    if token.token_value == "exp" {
                        return create_function(&token.token_value, simplify(Expression::Mul {
                            left: Box::from(*input.clone()),
                            right: Box::from(e.clone()),
                        }))
                    }
                    return Expression::Pow{ 
                        base: Box::from(base),
                        exponent: Box::from(exponent),
                    };

                }
                (_, Expression::Number(t)) => {
                    if t.token_value == "0" {
                        return one()
                    }

                    if t.token_value == "1" {
                        return base
                    }

                    return Expression::Pow{
                        base: Box::from(base),
                        exponent: Box::from(exponent),
                    }
                }
                _ => {
                    return Expression::Pow{
                        base: Box::from(base),
                        exponent: Box::from(exponent),
                    }
                }
            }
        }
        Expression::Mul { left, right } => {
            let left = simplify(*left);
            let right = simplify(*right);
            match (&left, &right) {
                (Expression::Number(t), Expression::Number(t2)) => {
                    return match (&t.token_value, &t2.token_value) {
                        (x, y) => {
                            let num1 = x.parse::<f64>().unwrap();
                            let num2 = y.parse::<f64>().unwrap();

                            Expression::Number(Token{
                                token_type: TokenType::Number, 
                                token_value: (num1*num2).to_string(),
                            })
                        }
                    }
                }
               (Expression::Number(num), Expression::Variable(var))|
                    (Expression::Variable(var), Expression::Number(num)) => {
                        Expression::VarSimple{
                            token: var.clone(),
                            mult: Box::from(Expression::Number(num.clone())),
                        }
                }
                (Expression::Number(num), Expression::VarSimple { token, mult }) |
                    (Expression::VarSimple { token, mult }, Expression::Number(num)) => {
                        Expression::VarSimple{
                            token: token.clone(),
                            mult: Box::from(simplify(Expression::Mul{
                                left: Box::from(Expression::Number(num.clone())),
                                right: mult.clone(),
                            })),
                        }


                }

                (Expression::Mul{ left, right }, e)|(e, Expression::Mul{ left, right }) => {
                    simplify(Expression::MulFlat{
                        args: Vec::from([Box::from(left.clone()), Box::from(right.clone()), Box::from(e.clone())])
                    })
                }
                (Expression::MulFlat { args }, e)| (e, Expression::MulFlat { args }) => {
                    let mut args = args.clone();
                    args.push(Box::from(e.clone()));
                    simplify(Expression::MulFlat{
                        args,
                    })

                }

                (Expression::Pow { base, exponent }, e) | (e, Expression::Pow { base, exponent }) => {
                    let base = base.clone();
                    let e = e.clone();
                    if *base == e {
                        return simplify(Expression::Pow {
                            base,
                            exponent: Box::from(simplify(Expression::Add{
                                left: exponent.clone(),
                                right: Box::from(one())
                            }))
                        });
                    }
                    match (e.clone(), *base.clone()) {
                        (Expression::VarSimple { token, mult }, Expression::Variable(token2)) => {
                            if token.token_value != token2.token_value {
                                return Expression::Mul{
                                    left: Box::from(left),
                                    right: Box::from(right),
                                }
                            }
                            return Expression::Pow{
                                base: Box::from(Expression::Mul{
                                    left: base.clone(),
                                    right: mult,
                                }),
                                exponent: Box::from(one())
                            }
                            

                        }

                        _ => {}
                        
                    }
                    return Expression::Mul{
                        left: Box::from(e),
                        right: Box::from(Expression::Pow{
                            base,
                            exponent: exponent.clone(),
                        })
                    }
                        
                }

                _ => {
                    Expression::Mul{
                        left: Box::from(left),
                        right: Box::from(right),
                    }

                }
            }
        }
        Expression::Add { left, right } => {
            let left = simplify(*left);
            let right = simplify(*right);
            match (&left, &right) {
                (Expression::Pow{base, exponent}, Expression::Pow { base: base2, exponent: exponent2 }) => {
                    if !(base == base2 && exponent == exponent2) {
                        return Expression::Add{
                            left: Box::from(left),
                            right: Box::from(right),
                        }
                    }

                    return Expression::Mul{
                        left: Box::from(Expression::Number(
                            Token{
                                token_type: TokenType::Number,
                                token_value: "2".to_string(),
                            }
                        )),
                        right: Box::from(right),
                    }
                }
                (Expression::Mul { left, right }, Expression::Mul { left: left2, right: right2 }) => {
                    if right == right2 && is_exponent(*right.clone()) {
                        return simplify(Expression::Mul{
                            left: Box::from(Expression::Add{
                                left: left.clone(),
                                right: left2.clone(),
                            }),
                            right: Box::from(*right.clone()),
                        })
                    }
                    return Expression::Add{
                        left: Box::from(*left.clone()),
                        right: Box::from(*right.clone()),
                    }
                }
                (Expression::Mul { left: left2, right: right2 }, Expression::Pow { base, exponent })|
                (Expression::Pow { base, exponent }, Expression::Mul { left: left2, right: right2 }) => {
                        if is_exponent(*left2.clone()) {
                            match *left2.clone() {
                                Expression::Pow { base: base2, exponent: exponent2 } => {
                                    if !(*base == base2 && *exponent == exponent2) {
                                        return Expression::Add{
                                            left: Box::from(left),
                                            right: Box::from(right),
                                        };
                                    }
                                    return Expression::Mul{
                                        left: Box::from(simplify(Expression::Add{
                                            left: Box::from(one()),
                                            right: Box::from(*right2.clone()),
                                        })),
                                        right: Box::from(*left2.clone()),
                                    }


                                }
                                _ => {}
                            }
                        } else if is_exponent(*right2.clone()) {
                            match *right2.clone() {
                                Expression::Pow { base: base2, exponent: exponent2 } => {
                                    if !(*base == base2 && *exponent == exponent2) {
                                        return Expression::Add{
                                            left: Box::from(left),
                                            right: Box::from(right),
                                        };
                                    }
                                    return Expression::Mul{
                                        left: Box::from(simplify(Expression::Add{
                                            left: Box::from(one()),
                                            right: Box::from(*left2.clone()),
                                        })),
                                        right: Box::from(*right2.clone()),
                                    }
                                }
                                _ => {}
                            }
                        }
                        Expression::Add{
                            left: Box::from(left),
                            right: Box::from(right),
                        }

                    
                }
                (Expression::Number(t), Expression::Number(t2)) => {
                    return match (&t.token_value, &t2.token_value) {
                        (x, y) => {
                            let num1 = x.parse::<f64>().unwrap();
                            let num2 = y.parse::<f64>().unwrap();

                            Expression::Number(Token{
                                token_type: TokenType::Number, 
                                token_value: (num1+num2).to_string(),
                            })
                        }
                    }
                }
                (Expression::Add { left, right }, e)|(e, Expression::Add { left, right }) => {
                    simplify(Expression::AddFlat{
                        args: Vec::from([Box::from(left.clone()), Box::from(right.clone()), Box::from(e.clone())])
                    })
                }

                (Expression::AddFlat { args }, e)| (e, Expression::AddFlat { args }) => {
                    let mut args = args.clone();
                    args.push(Box::from(e.clone()));
                    simplify(Expression::AddFlat{
                        args,
                    })

                }
                _ => {
                    Expression::Add{
                        left: Box::from(left),
                        right: Box::from(right),
                    }

                }
            }

        }
        Expression::AddFlat { args } => {
            if args.len() == 1 {
                return *args[0].clone()
            }
            let mut nums = 0.0;
            let mut other: Vec<Box<Expression>> = Vec::new();
            let mut vars: HashMap<String, Expression> = HashMap::new();
            for arg in args {
                match *arg {
                    Expression::Number(t) => {
                        nums += t.token_value.parse::<f64>().unwrap();
                    }
                    Expression::VarSimple { token, mult } => {
                        if !vars.contains_key(&token.token_value) {
                            vars.insert(token.token_value, *mult);
                            continue
                        }
                        let e = vars.get_mut(&token.token_value).unwrap().clone();
                        let new = simplify(Expression::Add{left: Box::from(e), right: mult});
                        *vars.get_mut(&token.token_value).unwrap() = new;
                    }
                    e => {
                        other.push(Box::from(simplify(e)))
                    }

                }
            }

            if nums != 0.0 {
                other.push(Box::from(Expression::Number(Token{token_type: TokenType::Number, token_value: nums.to_string()})));
            }

            for (key, val) in vars {
                other.push(Box::from(
                    Expression::VarSimple{
                        token: Token{
                            token_type: TokenType::Variable,
                            token_value: key,
                        },
                        mult: Box::from(val),
                    }
                ));
            }
            Expression::AddFlat{
                args: other
            }
        }

        Expression::MulFlat{ args } => {
            if args.len() == 1 {
                return *args[0].clone();
            }
            let mut nums = 1.0;
            let mut other: Vec<Box<Expression>> = Vec::new();
            let mut vars: HashMap<String, Expression> = HashMap::new();
            for arg in args {
                let arg_temp = arg.clone();
                match *arg {
                    Expression::Mul { left, right } => {
                        other.push(left);
                        other.push(right);
                    }
                    Expression::Number(t) => {
                        nums *= t.token_value.parse::<f64>().unwrap();
                    }
                    Expression::Pow { base, .. } => {
                        match *base {
                            Expression::Variable( token ) => {
                                if !vars.contains_key(&token.token_value) {
                                    vars.insert(token.token_value, *arg_temp);
                                    continue
                                }
                                let e = vars.get_mut(&token.token_value).unwrap().clone();
                                let new =  simplify(Expression::Mul{
                                    left: Box::from(e),
                                    right: arg_temp.clone(),
                                });
                                *vars.get_mut(&token.token_value).unwrap() = new;
                            }
                            Expression::VarSimple { token, mult } => {
                                if !vars.contains_key(&token.token_value) {
                                    vars.insert(token.token_value, *arg_temp);
                                    continue
                                }

                                let e = vars.get_mut(&token.token_value).unwrap().clone();
                                let new =  simplify(Expression::Mul{
                                    left: Box::from(simplify(Expression::Mul{
                                        left: Box::from(e),
                                        right: arg_temp.clone(),
                                    })),
                                    right: mult,
                                });
                                *vars.get_mut(&token.token_value).unwrap() = new;
                            }
                            _ => {
                                other.push(arg_temp.clone())
                            }
                        }
                    }
                    Expression::Variable(token) => {
                        if !vars.contains_key(&token.token_value) {
                            vars.insert(token.token_value, *arg_temp);
                            continue
                        }
                        let e = vars.get_mut(&token.token_value).unwrap().clone();
                        let new = simplify(Expression::Mul{left: Box::from(e), right: arg_temp});
                        *vars.get_mut(&token.token_value).unwrap() = new;

                    }
                    Expression::VarSimple { token, ..} => {
                        if !vars.contains_key(&token.token_value) {
                            vars.insert(token.token_value, *arg_temp);
                            continue
                        }
                        let e = vars.get_mut(&token.token_value).unwrap().clone();
                        let new = simplify(Expression::Mul{left: Box::from(e), right: arg_temp});
                        *vars.get_mut(&token.token_value).unwrap() = new;
                    }
                    e => {
                        other.push(Box::from(e))
                    }

                }
            }

            if nums != 1.0 {
                other.push(Box::from(Expression::Number(Token{token_type: TokenType::Number, token_value: nums.to_string()})))
            }

            for (_, val) in vars {
                other.push(Box::from(val));
            }

            Expression::MulFlat{
                args: other
            }
        }

        Expression::Function { token, input } => {
            let temp = reduce_inverse(token.clone(), *input.clone());
            match temp {
                Expression::NONE => {
                    return Expression::Function {
                        token,
                        input,
                    }
                }
                _ => {
                    return temp
                }
            }

        }
        _ => {
            exp
        }
    }
}


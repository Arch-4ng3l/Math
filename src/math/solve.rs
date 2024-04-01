use crate::preprocess::types::*;
use crate::math::simplify::simplify;
//  x + n = m => x = m - n
//  
//
//
//

enum Method {
    Add,
//    Mul, 
    Apply(String),
    Pow(Expression),
    None,
}

fn is_variable(exp: &Expression) -> bool {
    match exp {
        Expression::Variable(_)| Expression::VarSimple { .. } => {
            return true;
        }
        Expression::Pow { base, exponent } => {
            return is_variable(base) || is_variable(exponent);
        }
        _ => {
            return false;
        }
    }
}

fn get_inverse_function(func: &str) ->String { 
    return match func {
        "ln" => String::from("exp"),
        "exp" => String::from("ln"),
        "cos" => String::from("arccos"),
        "sin" => String::from("arcsin"),
        "tan" => String::from("arctan"),
        _ => {
            String::new()
        }

    }
}

fn get_inverse(exp: Expression) -> (Expression, Method) {
    match exp {
        Expression::Pow { exponent, .. } => {
            return (Expression::NONE, Method::Pow(
                Expression::Pow{
                    base: Box::from(exponent),
                    exponent: Box::from(negative_one()),
                }
            ))
        }
        Expression::Add { left, right } => {
            if is_variable(&*left) && !is_variable(&*right) {
                println!("test");
                return (simplify(Expression::Mul{
                    left: Box::from(negative_one()),
                    right,
                }), Method::Add)
            }
            if is_variable(&*right) && !is_variable(&*left) {
                println!("test1");
                return (simplify(Expression::Mul{
                    left: Box::from(negative_one()),
                    right: left,
                }), Method::Add)
            }

            return (Expression::NONE, Method::None)


        }
        Expression::AddFlat { args } => {
            let no_vars: Vec<Box<Expression>> = args
                .iter()
                .filter(|e| !is_variable(*e))
                .map(|e| e.clone())
                .collect();

            return (Expression::Mul {
                left: Box::from(negative_one()),
                right: Box::from(simplify(Expression::AddFlat{
                    args: no_vars
                }))
            }, Method::Add)
        }

        Expression::Function { token, .. } => {
            return (Expression::NONE, Method::Apply(get_inverse_function(&token.token_value)));
        }
        _ => {
            return (Expression::NONE, Method::None)
        }
    }
}


pub fn solve(eq: Equation) -> Equation {
    let (exp, meth) = get_inverse(eq.left.clone());
    match meth {
        Method::Add => {
            return solve(Equation {
                left: simplify(Expression::Add {
                    left: Box::from(eq.left),
                    right: Box::from(exp.clone()),
                }),
                right: simplify(Expression::Add {
                    left: Box::from(eq.right),
                    right: Box::from(exp),
                })
            })
        }
        Method::Pow(e) => {
            return Equation {
                left: simplify(Expression::Pow {
                    base: Box::from(eq.left),
                    exponent: Box::from(e.clone()),
                }),
                right: simplify(Expression::Pow {
                    base: Box::from(eq.right),
                    exponent: Box::from(e),
                }),
            }
        }
        Method::Apply(func) => {
            return solve(Equation {
                left: simplify(create_function(&func, eq.left)),
                right: simplify(create_function(&func, eq.right)),
            })
        }
        _ => {}
    }
    eq

}

use crate::preprocess::token::{Token, TokenType};
// Format 
// x^3+4x

fn is_number(char: char) -> bool {
    return '0' <= char && char <= '9'
}

fn is_char (char: char) -> bool {
    return 'a' <= char && char <='z' || 'A' <= char && char <= 'Z';
}

fn prepare_string(exp: String) -> String {
    let mut str = String::new();
    for i in 0..exp.len() {
        let cur_char = exp.as_bytes()[i] as char;
        match cur_char {
            ' ' | '\t' => { continue }
            _ => {
                str.push(cur_char);
            }
        }

        if i < exp.len()-1 {
            if is_number(cur_char) && is_char(exp.as_bytes()[i+1] as char) {
                str.push('*');
            }
        }

    }
    str.push('\0');
    return str;

}


pub fn run(input: String) -> Vec<Token> {
    let exp = prepare_string(input);
    let mut tokens = Vec::new();
    let mut i: usize = 0;
    while i < exp.len() {
        let mut char = exp.as_bytes()[i] as char;
        let token: Token = match char {
            '(' => Token{
                token_type: TokenType::LeftPara,
                token_value: String::from(char)

            },
            ')' => Token{
                token_type: TokenType::RightPara,
                token_value: String::from(char)
            },

            '^' => Token{
                token_type: TokenType::Caret,
                token_value: String::from(char)
            },
            '/' => Token{
                token_type: TokenType::Slash,
                token_value: String::from(char)
            },
            '*' => Token{
                token_type: TokenType::Star,
                token_value: String::from(char)
            },
            '-' => Token{
                token_type: TokenType::Minus,
                token_value: String::from(char)
            },
            '+' => Token{
                token_type: TokenType::Plus,
                token_value: String::from(char)
            },
            '=' => Token{
                token_type: TokenType::Plus,
                token_value: String::from(char)
            },
            _ => {
                let mut t: Token = Token{ 
                    token_type: TokenType::NULL,
                    token_value: "".to_string()
                };

                if is_number(char) {
                    let mut temp = String::new();
                    while is_number(char) {
                        temp.push(char);
                        i+=1;
                        char = exp.as_bytes()[i] as char;
                    }
                    if char == '.' {
                        temp.push(char);
                        i+=1;
                        char = exp.as_bytes()[i] as char;
                        while is_number(char) {
                            temp.push(char);
                            i+=1;
                            char = exp.as_bytes()[i] as char;
                        }
                    }
                    i-=1;
                    t.token_type = TokenType::Number;
                    t.token_value = temp;
                } else if is_char(char) {
                    let mut temp = String::new();
                    while is_char(char) {
                        temp.push(char);
                        i+=1;
                        char = exp.as_bytes()[i] as char;
                    }
                    if temp.len() > 1 {
                        t.token_type = TokenType::Function;
                    } else {
                        t.token_type = TokenType::Variable;
                    }
                    t.token_value = temp;
                    i-=1;
                }
                t
            }
        };
        i+=1;
        tokens.push(token);
    }
    return tokens;

}

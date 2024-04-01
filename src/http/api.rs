use std::{io::{Read, Write}, net::TcpStream};

use crate::{math::{derivative::derivative, simplify::simplify, solve::solve}, preprocess::{translator, types::Equation}};


pub fn handle(mut stream: TcpStream) {
    
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    let formula = request
        .split("\r\n\r\n")
        .collect::<Vec<&str>>()
        .pop()
        .unwrap();

    let path = request
        .lines()
        .next()
        .unwrap()
        .split_whitespace()
        .nth(1)
        .unwrap();

    match path {
        "/solve" => {
            handle_solve(formula, stream);
        }
        "/deriv" => {
            handle_deriv(formula, stream);
        }
        "/simplify" => {
            handle_simplify(formula, stream);
        }
        _ => {}
    }
}

fn handle_deriv(formula: &str, mut stream: TcpStream) {
    let t = translator::new();
    let eq = simplify(t.translate(String::from(formula)));
    let fin = simplify(derivative(eq));

    let response = format!(
        "HTTP/1.1 200 OK\r\n\r\n\r\n{}\r\n",
        fin
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn handle_solve(formula: &str, mut stream: TcpStream) {

    let t = translator::new();
    let parts: Vec<&str> = formula.split("=").collect();
    let eq = Equation{
        left: simplify(t.translate(parts[0].to_string())),
        right: simplify(t.translate(parts[1].to_string())),
    };
    let fin = solve(eq);

    let response = format!(
        "HTTP/1.1 200 OK\r\n\r\n\r\n{}\r\n",
        fin
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn handle_simplify(formula: &str, mut stream: TcpStream) {
    let t = translator::new();
    let form = simplify(t.translate(formula.to_string()));

    let response = format!(
        "HTTP/1.1 200 OK\r\n\r\n\r\n{}\r\n",
        form 
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
} 

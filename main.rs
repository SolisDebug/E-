mod lexer;
mod parser;
mod interpreter;

use lexer::tokenize;
use parser::parse;
use interpreter::interpret;
use std::io::{self, Write};

fn main() {
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let tokens = tokenize(input);
        println!("Tokens: {:?}", tokens);

        let ast = parse(&tokens);
        interpret(&ast);
    }
}

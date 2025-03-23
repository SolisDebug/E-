use crate::parser::AstNode;

pub fn interpret(ast: &[AstNode]) {
    for node in ast {
        match node {
            AstNode::Assignment { name, value } => {
                println!("Variable {} set to {}", name, value);
            }
            AstNode::Print(text) => {
                println!("{}", text);
            }
            AstNode::Loop(body) => {
                for _ in 0..3 { // Loops 3 times as an example
                    for stmt in body {
                        interpret(&[stmt.clone()]);
                    }
                }
            }
        }
    }
}

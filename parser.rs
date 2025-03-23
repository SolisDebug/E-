use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum AstNode {
    Assignment { name: String, value: f64 },
    Print(String),
    Loop(Vec<AstNode>),
}

pub fn parse(tokens: &[Token]) -> Vec<AstNode> {
    let mut iter = tokens.iter().peekable();
    let mut nodes = Vec::new();

    while let Some(token) = iter.next() {
        match token {
            Token::Identifier(name) => {
                if let Some(Token::Equals) = iter.peek() {
                    iter.next();
                    if let Some(Token::Number(value)) = iter.next() {
                        nodes.push(AstNode::Assignment { name: name.clone(), value: *value });
                    }
                }
            }
            Token::Print => {
                if let Some(next_token) = iter.next() {
                    match next_token {
                        Token::StringLiteral(text) => nodes.push(AstNode::Print(text.clone())),
                        Token::Identifier(var_name) => nodes.push(AstNode::Print(var_name.clone())),
                        _ => {}
                    }
                }
            }
            Token::LoopStart => {
                let mut body = Vec::new();
                while let Some(next_token) = iter.next() {
                    if matches!(next_token, Token::LoopEnd) {
                        break;
                    }
                    match next_token {
                        Token::Print => {
                            if let Some(Token::StringLiteral(text)) = iter.next() {
                                body.push(AstNode::Print(text.clone()));
                            }
                        }
                        _ => {}
                    }
                }
                nodes.push(AstNode::Loop(body));
            }
            _ => {}
        }
    }

    nodes
}

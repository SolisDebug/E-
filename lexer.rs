#[derive(Debug, Clone)]
pub enum Token {
    Identifier(String),
    Number(f64),
    StringLiteral(String),
    Equals,
    Print,
    LoopStart,
    LoopEnd,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            ' ' | '\t' | '\n' => { chars.next(); } // Skip whitespace
            '=' => {
                tokens.push(Token::Equals);
                chars.next();
            }
            '"' | '@' => {  // Support both `@Hello world@` and `"Hello world"`
                let quote_char = chars.next().unwrap(); // Store the opening quote (@ or ")
                let mut string = String::new();
                while let Some(&c) = chars.peek() {
                    if c == quote_char {  // Stop when the same quote appears
                        chars.next();
                        break;
                    }
                    string.push(c);
                    chars.next();
                }
                tokens.push(Token::StringLiteral(string));
            }
            '{' => {
                tokens.push(Token::LoopStart);
                chars.next();
            }
            '}' => {
                tokens.push(Token::LoopEnd);
                chars.next();
            }
            _ if ch.is_alphabetic() => {
                let mut ident = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_alphanumeric() {
                        ident.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "make" => tokens.push(Token::Print),
                    _ => tokens.push(Token::Identifier(ident)), // This handles variables
                }
            }
            _ if ch.is_digit(10) => {
                let mut num = String::new();
                while let Some(&c) = chars.peek() {
                    if c.is_digit(10) || c == '.' {
                        num.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(num.parse().unwrap()));
            }
            _ => { chars.next(); }
        }
    }

    tokens
}

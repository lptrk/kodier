mod token;
use token::Token;

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.peek().copied() {
        match ch {
            '.' => {
                chars.next();
                tokens.push(Token::Period);
            }
            ':' => {
                chars.next();
                tokens.push(Token::TypeOf);
            }
            '=' => {
                chars.next();
                tokens.push(Token::Assign);
            }
            '{' => {
                chars.next();
                tokens.push(Token::OpenBrace);
            }
            '}' => {
                chars.next();
                tokens.push(Token::CloseBrace);
            }
            '(' => {
                chars.next();
                tokens.push(Token::OpenParantheses);
            }
            ')' => {
                chars.next();
                tokens.push(Token::CloseParantheses);
            }
            '+' => {
                chars.next();
                tokens.push(Token::Plus);
            }
            '-' => {
                chars.next();
                tokens.push(Token::Hyphen);
            }
            '*' => {
                chars.next();
                tokens.push(Token::Star);
            }
            ';' => {
                chars.next();
                tokens.push(Token::Semicolon);
            }
            ',' => {
                chars.next();
                tokens.push(Token::Comma);
            }
            '/' => {
                chars.next();
                if let Some('/') = chars.peek() {
                    while let Some(&c) = chars.peek() {
                        if c == '\n' {
                            break;
                        }
                        chars.next();
                    }
                    tokens.push(Token::SingleLineComment);
                } else if let Some('*') = chars.peek() {
                    chars.next();
                    while let Some(c) = chars.next() {
                        if c == '*' && chars.peek() == Some(&'/') {
                            chars.next();
                            break;
                        }
                    }
                    tokens.push(Token::MultiLineCommentStart);
                } else {
                    tokens.push(Token::ForwardSlash);
                }
            }
            '|' => {
                chars.next();
                if let Some('|') = chars.peek() {
                    chars.next();
                    tokens.push(Token::Or);
                } else {
                    tokens.push(Token::BitwiseOr);
                }
            }
            '&' => {
                chars.next();
                if let Some('&') = chars.peek() {
                    chars.next();
                    tokens.push(Token::And);
                }
            }
            _ if ch.is_alphabetic() || ch == '_' => {
                let mut identifier = String::new();
                while let Some(&next_char) = chars.peek() {
                    if next_char.is_alphanumeric() || next_char == '_' {
                        identifier.push(next_char);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match identifier.as_str() {
                    "export" => tokens.push(Token::Export),
                    "if" => tokens.push(Token::If),
                    "return" => tokens.push(Token::Return),
                    "function" => tokens.push(Token::Function),
                    "class" => tokens.push(Token::Class),
                    "interface" => tokens.push(Token::Interface),
                    "implements" => tokens.push(Token::Implements),
                    _ => tokens.push(Token::Identifier(identifier)),
                }
            }
            _ => {
                chars.next();
            }
        }
    }
    print_tokens(&tokens);

    tokens
}
pub fn print_tokens(tokens: &Vec<Token>) {
    for token in tokens {
        match token {
            Token::Identifier(identifier) => println!("Identifier: {}", identifier),
            Token::Export => println!("Export"),
            Token::Implements => println!("Implements"),
            Token::TypeOf => println!("TypeOf"),
            Token::Assign => println!("Assign"),
            Token::Period => println!("Period"),
            Token::OpenBrace => println!("OpenBrace"),
            Token::CloseBrace => {
                println!("CloseBrace");
                println!();
                println!();
            }
            Token::If => println!("If"),
            Token::Return => println!("Return"),
            Token::Function => println!("Function"),
            Token::Class => println!("Class"),
            Token::Interface => println!("Interface"),
            Token::Plus => println!("Plus"),
            Token::Hyphen => println!("Hyphen"),
            Token::Star => println!("Star"),
            Token::Comma => println!("Comma"),
            Token::ForwardSlash => println!("ForwardSlash"),
            Token::And => println!("And"),
            Token::Or => println!("Or"),
            Token::Equal => println!("Equal"),
            Token::Inequal => println!("Inequal"),
            Token::BitwiseOr => println!("BitwiseOr"),
            Token::SingleLineComment => println!("SingleLineComment"),
            Token::MultiLineCommentStart => println!("MultiLineCommentStart"),
            Token::MultilineCommentEnd => println!("MultilineCommentEnd"),
            Token::OpenParantheses => println!("OpenParantheses"),
            Token::CloseParantheses => println!("CloseParantheses"),
            Token::Semicolon => {
                println!("Semicolon");
                println!();
            }
        }
    }
}

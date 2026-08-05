#[derive(Debug)]
pub enum Operator {
    Equals,
    Not,
    Greater,
    Less,
    Mult,
    Add,
    Sub,
    Div,
}

#[derive(Debug)]
pub enum Delimiter {
    Paren,
    Square,
    Curly,
}

impl Delimiter {
    fn matches_closing(&self, bit: &str) -> bool {
        match self {
            Delimiter::Paren => bit == ")",
            Delimiter::Square => bit == "]",
            Delimiter::Curly => bit == "}",
        }
    }

    fn is_closing(bit: &str) -> bool {
        bit.starts_with(')') || bit.starts_with(']') || bit.starts_with('}')
    }
}

#[derive(Debug)]
pub enum TokenType {
    Identifier,
    Number,
    Op(Operator, bool),
    Colon,
    Tree(Delimiter, Vec<TokenType>),
}

pub struct Span<'a> {}

pub struct Token<'a> {
    pub ty: TokenType,
    pub sp: Span<'a>,
}

pub fn parse_tree(delim: Option<Delimiter>, input: &mut &str) -> Vec<TokenType> {
    let mut tokens: Vec<TokenType> = Vec::new();
    loop {
        *input = input.trim_start();
        if input.is_empty() {
            break;
        }

        if let Some(delim) = &delim {
            if delim.matches_closing(&input[..1]) {
                *input = &input[1..];
                return tokens;
            } else if Delimiter::is_closing(&input[..1]) {
                panic!("mismatched closing delimiter {}", &input[..1]);
            }
        }

        if input.starts_with("{") {
            *input = &input[1..];
            tokens.push(TokenType::Tree(
                Delimiter::Curly,
                parse_tree(Some(Delimiter::Curly), input),
            ));
            continue;
        } else if input.starts_with("(") {
            *input = &input[1..];
            tokens.push(TokenType::Tree(
                Delimiter::Paren,
                parse_tree(Some(Delimiter::Paren), input),
            ));
            continue;
        } else if input.starts_with("[") {
            *input = &input[1..];
            tokens.push(TokenType::Tree(
                Delimiter::Square,
                parse_tree(Some(Delimiter::Square), input),
            ));
            continue;
        }

        let mut chars = input.chars().peekable();
        let mut consumed = 1usize;

        match chars.next().expect("should be non-empty") {
            // -- Operators --
            '*' => {
                consumed += 1;

                let is_eq = chars.peek().is_some_and(|c| c == &'=');

                if is_eq {
                    chars.next();
                    consumed += 1;
                }

                tokens.push(TokenType::Op(Operator::Mult, is_eq))
            }
            '+' => {
                consumed += 1;

                let is_eq = chars.peek().is_some_and(|c| c == &'=');

                if is_eq {
                    chars.next();
                    consumed += 1;
                }

                tokens.push(TokenType::Op(Operator::Add, is_eq))
            }
            '-' => {
                consumed += 1;

                let is_eq = chars.peek().is_some_and(|c| c == &'=');

                if is_eq {
                    chars.next();
                    consumed += 1;
                }

                tokens.push(TokenType::Op(Operator::Sub, is_eq))
            }
            '/' => {
                consumed += 1;

                let is_eq = chars.peek().is_some_and(|c| c == &'=');

                if is_eq {
                    chars.next();
                    consumed += 1;
                }

                tokens.push(TokenType::Op(Operator::Div, is_eq))
            }
            '<' => {
                consumed += 1;

                let is_eq = chars.peek().is_some_and(|c| c == &'=');

                if is_eq {
                    chars.next();
                    consumed += 1;
                }

                tokens.push(TokenType::Op(Operator::Less, is_eq))
            }
            '>' => {
                consumed += 1;

                let is_eq = chars.peek().is_some_and(|c| c == &'=');

                if is_eq {
                    chars.next();
                    consumed += 1;
                }

                tokens.push(TokenType::Op(Operator::Greater, is_eq))
            }
            '=' => {
                consumed += 1;

                let is_eq = chars.peek().is_some_and(|c| c == &'=');

                if is_eq {
                    chars.next();
                    consumed += 1;
                }

                tokens.push(TokenType::Op(Operator::Equals, is_eq))
            }
            '!' => {
                consumed += 1;

                let is_eq = chars.peek().is_some_and(|c| c == &'=');

                if is_eq {
                    chars.next();
                    consumed += 1;
                }

                tokens.push(TokenType::Op(Operator::Not, is_eq))
            }
            // -- Other --
            c if c.is_alphabetic() || c == '_' => {
                let mut s = String::new();
                s.push(c);

                'inner: loop {
                    match chars.peek() {
                        Some(c) if c.is_alphanumeric() || c == &'_' => {
                            s.push(*c);
                            chars.next();
                            consumed += 1;
                        }
                        _ => break 'inner,
                    }
                }

                tokens.push(TokenType::Identifier(s))
            }
            c if c.is_numeric() => {
                let mut s = String::new();
                s.push(c);

                'inner: loop {
                    match chars.peek() {
                        Some(c) if c.is_numeric() => {
                            s.push(*c);
                            chars.next();
                            consumed += 1;
                        }
                        _ => break 'inner,
                    }
                }

                tokens.push(TokenType::Number(s))
            }
            c => panic!("unknown next char: `{c}`"),
        }

        *input = &input[consumed..];
    }

    if let Some(delim) = delim {
        panic!("unclosed delimiter {delim:?}");
    } else {
        tokens
    }
}

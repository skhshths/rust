use std::io;
use std::io::Read;
use std::io::Write;

use std::collections::HashMap;

use cliutils::clean_split;
use cliutils::string_to_vec;
use cliutils::strip_all;
use cliutils::vec_to_string;
use std::fmt::Display;
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
enum Operator {
    Equals,
    Not,
    Greater,
    Less,
}

#[derive(Debug)]
enum Delimiter {
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
enum Token {
    Identifier(String),
    Integer(String),
    Op(Operator, bool),
    Colon,
    Tree(Delimiter, Vec<Token>),
}

fn parse_tree(delim: Option<Delimiter>, input: &mut &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
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
            tokens.push(Token::Tree(
                Delimiter::Curly,
                parse_tree(Some(Delimiter::Curly), input),
            ));
            continue;
        } else if input.starts_with("(") {
            *input = &input[1..];
            tokens.push(Token::Tree(
                Delimiter::Paren,
                parse_tree(Some(Delimiter::Paren), input),
            ));
            continue;
        } else if input.starts_with("[") {
            *input = &input[1..];
            tokens.push(Token::Tree(
                Delimiter::Square,
                parse_tree(Some(Delimiter::Square), input),
            ));
            continue;
        }

        let mut chars = input.chars().peekable();
        let mut consumed = 1usize;

        match chars.next().expect("should be non-empty") {
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

                tokens.push(Token::Identifier(s))
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

pub fn main() {
    let mut file: File = File::open("src/test.aaa").expect("failed to open file");

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("failed to read file");
    println!("{:?}", parse_tree(None, &mut content.as_str()));
}

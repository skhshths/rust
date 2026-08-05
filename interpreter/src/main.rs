#![allow(dead_code, unused_imports, unused_variables)]
#![deny(clippy::unwrap_used)]

mod dih;

use std::borrow::Cow;
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

fn input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().expect("failed to flush stdout");
    let mut x: String = String::new();
    io::stdin()
        .read_line(&mut x)
        .expect("failed to read line from input");
    x
}

fn count(original: &str, target: &str) -> usize {
    original.matches(target).count()
}

fn zip<T: Copy, V: Copy>(a: &[T], b: &[V]) -> Vec<(T, V)> {
    let mut out: Vec<(T, V)> = Vec::new();
    for (index, item) in a.iter().enumerate() {
        let new: (T, V) = (*item, b[index]);
        out.push(new);
    }
    out
}

// (arguments_expected, func_raw)
type OpDefinition<'a> = (Vec<&'a str>, &'a str);

enum Word<'a> {
    Simple(&'a str),
    Paren(String),
}

fn read_word<'a>(expr: &mut &'a str) -> Option<Word<'a>> {
    *expr = expr.trim();

    if expr.starts_with('(') {
        *expr = &expr[1..];
        let mut paren_count = 1;
        let mut content = String::new();

        while let Some(char) = expr.chars().next() {
            if char == '(' {
                paren_count += 1;
            }

            if char == ')' {
                paren_count -= 1;

                if paren_count == 0 {
                    *expr = &expr[1..];
                    break;
                }
            }

            content.push(char);
            *expr = &expr[1..];
        }
        Some(Word::Paren(content))
    } else {
        let word = expr.split_whitespace().next()?;
        Some(Word::Simple(word))
    }
}

#[derive(Debug, PartialEq)]
enum IntExpression {
    Value(i32),
    SubExpr(String),
    Variable(String),
}

impl IntExpression {
    fn as_string(&self) -> String {
        match self {
            IntExpression::Value(int) => int.to_string(),
            IntExpression::SubExpr(expr) => expr.to_string(),
            IntExpression::Variable(var) => var.to_string(),
        }
    }
}

fn read_int_expr(expr: Word<'_>) -> IntExpression {
    match expr {
        Word::Simple(expr) => {
            if let Ok(int) = expr.trim().parse::<i32>() {
                IntExpression::Value(int)
            } else {
                IntExpression::Variable(expr.trim().to_string())
            }
        }
        Word::Paren(expr) => IntExpression::SubExpr(expr),
    }
}

fn parsei(
    mut val: &str,
    int_variables: &HashMap<&str, i32>,
    biops: &HashMap<&str, OpDefinition>,
    unops: &HashMap<&str, OpDefinition>,
) -> i32 {
    let part_1 = read_word(&mut val).expect("must have at least one word to parse");
    let part_2 = read_word(&mut val);
    let part_3 = read_word(&mut val);

    match (part_2, part_3) {
        (None, None) => match part_1 {
            Word::Paren(expr) => {
                panic!("expected single value or variable, found parenthesised expression {expr}")
            }
            Word::Simple(expr) => {
                if let Ok(int) = expr.trim().parse::<i32>() {
                    int
                } else if let Some(x) = int_variables.get(expr) {
                    *x
                } else {
                    panic!("unknown variable {expr}");
                }
            }
        },
        (Some(part_2), None) => {
            let op = match part_1 {
                Word::Paren(expr) => panic!("expected unary operator, found expression {expr}"),
                Word::Simple(expr) => expr,
            };

            let x = match part_2 {
                Word::Paren(expr) => parsei(&expr, int_variables, biops, unops),
                Word::Simple(expr) => {
                    if let Ok(int) = expr.trim().parse::<i32>() {
                        int
                    } else if let Some(x) = int_variables.get(expr) {
                        *x
                    } else {
                        panic!("unknown variable {expr}");
                    }
                }
            };

            todo!()
        }
        (Some(part_2), Some(part_3)) => {
            let a = match part_1 {
                Word::Paren(expr) => parsei(&expr, int_variables, biops, unops),
                Word::Simple(expr) => {
                    if let Ok(int) = expr.trim().parse::<i32>() {
                        int
                    } else if let Some(x) = int_variables.get(expr) {
                        *x
                    } else {
                        panic!("unknown variable {expr}");
                    }
                }
            };

            let op = match part_2 {
                Word::Paren(expr) => panic!("expected unary operator, found expression {expr}"),
                Word::Simple(expr) => expr,
            };

            let b = match part_3 {
                Word::Paren(expr) => parsei(&expr, int_variables, biops, unops),
                Word::Simple(expr) => {
                    if let Ok(int) = expr.trim().parse::<i32>() {
                        int
                    } else if let Some(x) = int_variables.get(expr) {
                        *x
                    } else {
                        panic!("unknown variable {expr}");
                    }
                }
            };

            match op {
                "+" => a + b,
                "-" => a - b,
                "/" => a / b,
                "*" => a * b,
                op => {
                    if let Some(biop) = biops.get(op) {
                        let args: &[&str] = &biop.0;
                        let mut function: Vec<Cow<str>> =
                            biop.1.split_whitespace().map(Cow::Borrowed).collect();
                        let inputted_ints: [i32; 2] = [a, b];
                        for (index, item) in function.iter_mut().enumerate() {
                            if let Some(args_index) = args.iter().position(|x| *x == item) {
                                let val: i32 = inputted_ints[args_index];
                                *item = Cow::Owned(inputted_ints[args_index].to_string());
                            }
                        }
                        parsei(&vec_to_string(&function, " "), int_variables, biops, unops)
                    } else {
                        panic!("unknown operator {op}");
                    }
                }
            }
        }
        _ => unreachable!(),
    }

    /*
    let left_expr = read_int_expr(&mut val);

    val = val.trim_start();
    let Some(op) = val.split_whitespace().next() else {
        panic!("unexpected EOS");
    };
    val = &val[op.len()..];
    println!("val: {val}");

    let right_expr = read_int_expr(&mut val);

    let a = match left_expr {
        IntExpression::SubExpr(sub) => parsei(&sub, int_variables, biops, unops),
        IntExpression::Value(int) => int,
        IntExpression::Variable(var) => {
            if let Some(x) = int_variables.get(var.as_str()) {
                *x
            } else {
                if unops.contains_key(var.as_str()) {
                    println!("x: {var}");
                    3
                } else {
                    3
                }
            }
        }
    };

    let b = match right_expr {
        IntExpression::SubExpr(sub) => parsei(&sub, int_variables, biops, unops),
        IntExpression::Value(int) => int,
        IntExpression::Variable(var) => {
            if let Some(x) = int_variables.get(var.as_str()) {
                *x
            } else {
                if unops.contains_key(var.as_str()) {
                    println!("x: {var}");
                    3
                } else {
                    3
                }
            }
        }
    };

    match op {
        "+" => a + b,
        "-" => a - b,
        "/" => a / b,
        "*" => a * b,
        op => {
            if let Some(biop) = biops.get(op) {
                let args: &[&str] = &biop.0;
                let mut function: Vec<Cow<str>> =
                    biop.1.split_whitespace().map(Cow::Borrowed).collect();
                let inputted_ints: [i32; 2] = [a, b];
                for (index, item) in function.iter_mut().enumerate() {
                    if let Some(args_index) = args.iter().position(|x| *x == item) {
                        let val: i32 = inputted_ints[args_index];
                        *item = Cow::Owned(inputted_ints[args_index].to_string());
                    }
                }
                parsei(&vec_to_string(&function, " "), int_variables, biops, unops)
            } else {
                panic!("unknown operator {op}");
            }
        }
    }
     */
}

fn main() {
    let mut file: File = File::open("src/test.aaa").expect("failed to open file");

    let mut string_variables: HashMap<&str, String> = HashMap::new();
    let mut int_variables: HashMap<&str, i32> = HashMap::new();
    let mut biops: HashMap<&str, OpDefinition> = HashMap::new();
    let mut unops: HashMap<&str, OpDefinition> = HashMap::new();

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("failed to read file");
    let data: Vec<&str> = content
        .lines()
        .filter_map(|x| {
            let trimmed: &str = x.trim();
            if !trimmed.is_empty() {
                Some(trimmed)
            } else {
                None
            }
        })
        .collect();

    for (index, line) in data.iter().enumerate() {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let out: String = vec_to_string(&tokens, " ");
        let first: &str = tokens[0];

        if first == "print" {
            let val: Vec<&str> = clean_split(line, "print ");
            let characters: Vec<&str> = string_to_vec(val[1]);
            if characters.contains(&"\"") {
                let message: String = strip_all(&vec_to_string(&val, ""), "\"").to_string();
                println!("{message}");
            } else {
                let message: &str = &vec_to_string(&val, "");
                if string_variables.contains_key(message) {
                    let out: &str = &string_variables[message];
                    println!("{out}");
                } else if int_variables.contains_key(message) {
                    let out: i32 = int_variables[message];
                    println!("{out}");
                }
            }
        } else if first == "strshow" {
            let varname: &str = clean_split(line, " ")[1];
            let varval: &str = &string_variables[varname];
            println!("{varname}: {varval}");
        } else if first == "intshow" {
            let varname: &str = clean_split(line, " ")[1];
            let varval: &i32 = &int_variables[varname];
            println!("{varname}: {varval}");
        } else if first == ";" {
            continue;
        } else {
            let val: Vec<&str> = clean_split(line, ": ");
            let name: &str = val[0];
            let vartype: &str = clean_split(val[1], " = ")[0];
            let val: &str = clean_split(val[1], " = ")[1];

            if vartype == "string" {
                let cleaned_val: &str = strip_all(val, "\"");
                if let Some(prompt_with_end) = cleaned_val.strip_prefix("input(\"") {
                    let prompt: &str = prompt_with_end
                        .strip_suffix("\")")
                        .expect("should have `)` after prompt");

                    let out: String = input(prompt);
                    string_variables.insert(name, out);
                } else {
                    string_variables.insert(name, cleaned_val.to_string());
                }
            } else if vartype == "stringcc" {
                let cleaned_val: &str = strip_all(clean_split(line, " = ")[1], "\"");

                let left_bracket_count: usize = count(cleaned_val, "{");
                let right_bracket_count: usize = count(cleaned_val, "}");

                let mut left_bracket_indexes: Vec<usize> = Vec::new();
                let mut right_bracket_indexes: Vec<usize> = Vec::new();

                // left then right then left then right then ...
                // loop through cleaned_val as a vec, keep indexes in seperate lists
                // insert indexes into hashmaps
                // split between correspodning indexes, swap out for variables
                let characters: Vec<&str> = string_to_vec(cleaned_val);
                for (index, item) in characters.iter().enumerate() {
                    if *item == "{" {
                        left_bracket_indexes.push(index);
                    } else if *item == "}" {
                        right_bracket_indexes.push(index);
                    }
                }

                for (left, right) in zip(&left_bracket_indexes, &right_bracket_indexes) {
                    let inside: Vec<&str> = characters[left + 1..right].to_vec();
                    let inside_with_brackets: Vec<&str> = characters[left..=right].to_vec();
                    let val: &str = &vec_to_string(&inside, "");

                    let final_value = if string_variables.contains_key(val) {
                        string_variables[val].to_string()
                    } else {
                        int_variables[val].to_string()
                    };

                    let x = &vec_to_string(&inside_with_brackets, "");
                    let split_at_dec: Vec<&str> = clean_split(cleaned_val, x);

                    let new: &str =
                        &vec_to_string(&[split_at_dec[0], &final_value, split_at_dec[1]], "");
                    string_variables.insert(name, new.to_string());
                }
            } else if vartype == "int" {
                if let Ok(cleaned_val) = val.parse::<i32>() {
                    panic!("variable of type int must be iparse, not raw integer");
                } else {
                    let out: i32 = parsei(val, &int_variables, &biops, &unops);

                    int_variables.insert(name, out);
                }
            } else {
                if vartype.contains("biop") {
                    let op_type: &str = clean_split(clean_split(vartype, " -> ")[0], " ")[0];
                    let args: Vec<&str> = clean_split(
                        clean_split(clean_split(vartype, " -> ")[0], "(")[1]
                            .strip_suffix(")")
                            .expect("arguments must end in ')'"),
                        ", ",
                    );

                    if op_type == "biop" {
                        biops.insert(name, (args.clone(), val));
                    } else if op_type == "unop" {
                        unops.insert(name, (args.clone(), val));
                    }
                }
            }
        }
    }
}

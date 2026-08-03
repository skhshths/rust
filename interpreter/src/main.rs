#![allow(dead_code, unused_imports, unused_variables)]
#![deny(clippy::unwrap_used)]

mod dih;

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

fn parsei(val: &str, int_variables: &HashMap<&str, i32>) -> i32 {
    let cleaned_val: Vec<&str> = clean_split(val, " ");

    let a_init: &str = cleaned_val[0];
    let op: &str = cleaned_val[1];
    let b_init: &str = cleaned_val[2];

    let mut a = None;
    let mut b = None;
    if int_variables.contains_key(a_init) {
        a = Some(int_variables[a_init]);
    }

    if int_variables.contains_key(b_init) {
        b = Some(int_variables[b_init]);
    }

    if let Ok(val) = a_init.parse::<i32>() {
        a = Some(val);
    }

    if let Ok(val) = b_init.parse::<i32>() {
        b = Some(val);
    }

    let Some(a) = a else {
        panic!("{a_init} is neither an integer nor a variable");
    };

    let Some(b) = b else {
        panic!("{b_init} is neither an integer nor a variable")
    };

    match op {
        "+" => a + b,
        "-" => a - b,
        "/" => a / b,
        "*" => a * b,
        op => panic!("{op} not valid operation"),
    }
}

fn main() {
    let mut file: File = File::open("src/test.aaa").expect("failed to open file");

    let mut string_variables: HashMap<&str, String> = HashMap::new();
    let mut int_variables: HashMap<&str, i32> = HashMap::new();
    let mut uops: HashMap<&str, (Vec<&str>, &str)> = HashMap::new();

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
                if val.contains("(") {
                    let opname: &str = clean_split(val, "(")[0];
                    if uops.contains_key(opname) {
                        let args: Vec<&str> = uops[opname].0.clone();
                        let ret_type: &str = uops[opname].1;
                        let mut inputted: Vec<&str> = clean_split(val, "(");
                        // here
                        println!("{inputted:?}")
                    }
                }
                let cleaned_val: i32 = val
                    .parse::<i32>()
                    .expect("should be a valid signed integer");
                int_variables.insert(name, cleaned_val);
            } else if vartype == "iparse" {
                let out: i32 = parsei(val, &int_variables);

                int_variables.insert(name, out);
            } else {
                if vartype.contains("->") {
                    let op_type: &str = clean_split(clean_split(vartype, " -> ")[0], " ")[0];
                    let args: Vec<&str> = clean_split(
                        clean_split(clean_split(vartype, " -> ")[0], "(")[1]
                            .strip_suffix(")")
                            .expect("arguments must end in ')'"),
                        ", ",
                    );
                    let ret_type: &str = clean_split(vartype, " -> ")[1];
                    if op_type == "uop" {
                        uops.insert(name, (args, ret_type));
                    }
                }
            }
        }
    }
}

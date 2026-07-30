#![allow(dead_code, unused_imports, unused_variables)]

use std::io::Read;

use std::collections::HashMap;

use std::fmt::Display;
use std::fs::File;
use std::path::Path;

use cliutils::clean_split;
use cliutils::vec_to_string;
use cliutils::strip_all;
use cliutils::string_to_vec;
use cliutils::input;

fn count(original: &str, target: &str) -> i32 {
  original.matches(target).count().try_into().unwrap()
}

fn zip<T: Copy, V: Copy>(a: &[T], b: &[V]) -> Vec<(T, V)> {
  let mut out: Vec<(T, V)> = Vec::new();
  for (index, item) in a.iter().enumerate() {
    let new: (T, V) = (*item, b[index]);
    out.push(new);
  }
  out
}

fn main() {
    let path = Path::new("src/test.aaa");
    let mut file: File = File::open(&path).unwrap();

    let mut string_variables: HashMap<&str, String> = HashMap::new();

    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();
    let data: Vec<&str> = content.lines().collect();

    for (index, line) in data.iter().enumerate() {
      let tokens: Vec<&str> = line.split(" ").collect();
      let out: String = vec_to_string(&tokens, " ");
      let first: &str = tokens[0];

      if first == "print" {
        let val: Vec<&str> = clean_split(line, "print ");
        let characters: Vec<&str> = string_to_vec(val[1]);
        if characters.contains(&"\"") {
          let message: String = strip_all(&vec_to_string(&val, &""), "\"").to_string();
          println!("{message}");
        } else {
          let message: &str = &vec_to_string(&val, &"");
          if string_variables.contains_key(message) {
            let out: &str = &string_variables[message];
            println!("{out}");
          }
        }
        
      } else if first == "" {
      } else {
        let val: Vec<&str> = clean_split(line, ": ");
        let name: &str = val[0];
        let vartype: &str = clean_split(val[1], " = ")[0];
        let val: &str = clean_split(val[1], " = ")[1];

        if vartype == "string" {
          let cleaned_val: &str = strip_all(val, "\"");
          if cleaned_val.starts_with("input(\"") {
            let prompt: &str = clean_split(cleaned_val, "input(\"")[1].strip_suffix("\")").unwrap();
            let out: String = input(prompt);
            string_variables.insert(name, out);
          } else {  
            string_variables.insert(name, cleaned_val.to_string());
          }
        } else if vartype == "stringcc" {
          let cleaned_val: &str = strip_all(clean_split(line, " = ")[1], "\"");
          
          let left_bracket_count: i32 = count(cleaned_val, "{");
          let right_bracket_count: i32 = count(cleaned_val, "}");
          
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
            let final_value: String = string_variables.get(val).unwrap().to_string();
            
            let x = &vec_to_string(&inside_with_brackets, "");
            let split_at_dec: Vec<&str> = clean_split(cleaned_val, x);

            let new: String = vec_to_string(&vec![split_at_dec[0], &final_value, split_at_dec[1]], "");
            
            string_variables.insert(name, new);
          }
        }
      }
    }
}

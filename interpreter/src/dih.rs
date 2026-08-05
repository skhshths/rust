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

pub mod lexer;
pub mod parser;

pub fn main() {
    let mut file: File = File::open("src/test.aaa").expect("failed to open file");

    let mut content = String::new();

    file.read_to_string(&mut content)
        .expect("failed to read file");
    println!("{:?}", lexer::parse_tree(None, &mut content.as_str()));
}

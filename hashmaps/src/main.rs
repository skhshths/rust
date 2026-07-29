use std::collections::HashMap;
use std::fmt::Display;

use cliutils::show_vec;
use cliutils::show_hash;

fn main() {
    let mut students: HashMap<&str, i32> = HashMap::new();

    students.insert("oliver", 100);
    students.insert("asdf", 1);

    let x: Vec<String> = show_hash(&students, " - grade: ");

    for item in x {
      println!("{item}");
    }
}

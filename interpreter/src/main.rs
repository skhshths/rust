use std::io::Read;

use std::fmt::Display;
use std::fs::File;
use std::path::Path;

fn show_vec<T: Display>(v: &[T], inter: String) -> String {
  let mut out: String = String::new();
  for item in v {
    out += &item.to_string();
    out += &inter;
  }
  out
}

fn main() {
    let path = Path::new("src/test.aaa");
    let mut file: File = File::open(&path).unwrap();

    let mut content = String::new();

    file.read_to_string(&mut content).unwrap();
    let data: Vec<&str> = content.lines().collect();

    for (index, item) in data.iter().enumerate() {
        let tokens: Vec<&str> = item.split(" ").collect();
        let out: String = show_vec(&tokens, String::from(" "));
        println!("{out}");
    }
}

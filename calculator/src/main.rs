use std::io;
use std::io::Write;

fn add(x: i32, y: i32) -> i32 {
  x + y
}

fn sub(x: i32, y: i32) -> i32 {
  x - y
}

fn div(x: i32, y: i32) -> i32 {
  x / y
}

fn mult(x: i32, y: i32) -> i32 {
  x * y
}

fn input(prompt: String) -> String {
  print!("{prompt}");
  io::stdout().flush().unwrap();
  let mut x = String::new();
  io::stdin().read_line(&mut x).unwrap();
  x.trim().to_string()
}

fn main() {
  let x: i32 = input("give a number: ".to_string()).parse::<i32>().unwrap();
  let y: i32 = input("give another number: ".to_string()).parse::<i32>().unwrap();
  let op: String = input("give me an operation (+-*/): ".to_string());

  let mut result: String = String::new();
  if op == "+" {
    result = add(x, y).to_string();
  } else if op == "-" {
    result = sub(x, y).to_string();
  } else if op == "*" {
    result = mult(x, y).to_string();
  } else if op == "/" {
    result = div(x, y).to_string();
  }

  println!("{x} {op} {y} = {result}");
}
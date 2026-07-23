use std::io;
use std::io::Write;
use rand::seq::IndexedRandom;

fn input(prompt: String) -> String {
  print!("{prompt}");
  io::stdout().flush().unwrap();
  let mut x = String::new();
  io::stdin().read_line(&mut x).unwrap();
  x.trim().to_string()
}

fn get_random_string<'a>(items: &'a Vec<&'a str>) -> &'a str {
  let chosen;
  chosen = items.choose(&mut rand::rng()).unwrap();
  chosen
}
#[allow(dead_code)]
fn get_random_i32(items: &Vec<i32>) -> &i32 {
  let chosen;
  chosen = items.choose(&mut rand::rng()).unwrap();
  chosen
}

fn main() {
  // possible characters to use
  let letters: Vec<&str> = vec!["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z"];
  let letters_uppercase: Vec<&str> = vec!["A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"];
  let symbols: Vec<&str> = vec!["!", "@", "#", "$", "%", "^", "&", "*", "~"];
  
  // user preferences 
  let length: i32 = input("length of password? ".to_string()).parse::<i32>().unwrap();
  
  let with_symbols_user: String = input("want symbols (y/n)? ".to_string());
  let with_symbols: bool;1
  if with_symbols_user == "y" {
    with_symbols = true;
  } else {
    with_symbols = false;
  }

  let with_uppercase_user: String = input("want uppercase letters (y/n)? ".to_string());
  let with_uppercase: bool;
  if with_uppercase_user == "y" {
    with_uppercase = true;
  } else {
    with_uppercase = false;
  }

  // loop to add randomly chosen values of letters/symbols
  // based off of user preferences
  let mut password: String = String::new();
  
  let sym_type: Vec<&str> = vec!["s", "lu", "l"];
  for _n in 1..=length {
    let choice = get_random_string(&sym_type);

    if choice == "l" {
      let chosen_letter = get_random_string(&letters);
      password += chosen_letter;
    } else if choice == "s" && with_symbols {
      let chosen_symbol = get_random_string(&symbols);
      password += chosen_symbol;
    } else if choice == "lu" && with_uppercase {
      let chosen_letter = get_random_string(&letters_uppercase);
      password += chosen_letter;
    }
  }

  println!("generated password of length {length} is: {password}");
}

use std::io;
use std::io::Write;
use rand::Rng;

fn input(prompt: String) -> String {
  print!("{prompt}");
  io::stdout().flush().unwrap();
  let mut x = String::new();
  io::stdin().read_line(&mut x).unwrap();
  x.trim().to_string()
}

fn main() {
  let mut rng = rand::thread_rng();
  let n: i8 = rng.gen_range(1..=100);
  let mut guesses: i8 = 0;
  let max: i8 = input("how many guesses do you want? ".to_string()).parse::<i8>().unwrap();

  while guesses < max {
    let user_guess: i8 = input("guess a number from 1-100 (inclusive): ".to_string()).parse::<i8>().unwrap();
    if user_guess == n {
      println!("wow, you got it!");
      break;
    } else if user_guess > n {
      println!("lower!");
    } else if user_guess < n {
      println!("higher!");
    }
    
    guesses += 1;
  }
  println!("the number was {n}!");
}
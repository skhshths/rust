fn input(prompt: String) -> String {
  print!("{prompt}");
  io::stdout().flush().unwrap();
  let mut x = String::new();
  io::stdin().read_line(&mut x).unwrap();
  x.trim().to_string()
}

fn clear() -> () {
  print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
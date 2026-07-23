use std::io;
use std::io::Read;
use std::io::BufWriter;
use std::io::Write;
use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;

fn input(prompt: String) -> String {
  print!("{prompt}");
  io::stdout().flush().unwrap();
  let mut x = String::new();
  io::stdin().read_line(&mut x).unwrap();
  x.trim().to_string()
}

fn main() -> Result<(), std::io::Error> {
  loop {
    let choice: String = input("read or write (r/w)? ".to_string());    
    let path = Path::new("src/text.txt");

    if choice == "r" {

      let mut file: File = File::open(&path).unwrap();

      let mut content: String = String::new();

      file.read_to_string(&mut content).unwrap();
      
      println!("{content}");
    } else if choice == "w" {
      let file = OpenOptions::new().append(true).create(true).open(path)?;

      let mut writer = BufWriter::new(file);

      let line: String = input("what do you want to write to this file? ".to_string());
      
      writeln!(writer, "{line}")?;
      writer.flush()?;
    } else {
      println!("not an option!");
      break;
    }
  }
  Ok(())
}

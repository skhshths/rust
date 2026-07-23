fn main() {
    println!("Hello, world!");
}
use std::io;
use std::io::Write;

struct Task {
  name: String,
  desc: String
}

fn input(prompt: String) -> String {
  print!("{prompt}");
  io::stdout().flush().unwrap();
  let mut x = String::new();
  io::stdin().read_line(&mut x).unwrap();
  x.trim().to_string()
}

fn add_item(item_name: String, item_desc: String, todo_list: &mut Vec<Task>) -> () {
  let new_task: Task = Task {
    desc: item_desc,
    name: item_name
  };

  todo_list.push(new_task);
}

fn main() {
  let mut todo_list: Vec<Task> = Vec::new();
  
  loop {
    let exit: String = input("do u wanna exit (y/n)? ".to_string());
    if exit == "y" { break; }
    
    let task_name: String = input("task name: ".to_string());
    let task_desc: String = input("task description: ".to_string());

    add_item(task_name, task_desc, &mut todo_list);
  }

  
  for item in todo_list {
    let mut name: String = item.name;
    let mut desc: String = item.desc;
    println!("{name} | {desc}");
  }
}
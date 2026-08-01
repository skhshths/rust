#![allow(dead_code, unused_imports, unused_variables)]


use std::io;
use std::io::Write;

struct Student {
    name: String,
    age: i32,
    grade: i32,
    overall_grade: f32
}

impl Student {
    fn create(data: (&str, i32, i32, f32)) -> Student {
        Student {
            name: data.0.to_string(),
            age: data.1,
            grade: data.2,
            overall_grade: data.3
        }
    }

    fn say_hi(&self) {
        println!("yo what");
    }
}

fn input_i32(prompt: &str) -> i32 {
    loop {
        let val: String = input(prompt);
        match val.parse::<i32>() {
            Ok(n) => return n,
            Err(_) => println!("NOTVALIDINT: {:?}", val)
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut x: String = String::new();
    io::stdin().read_line(&mut x).unwrap();
    x.trim().to_string()
}

fn main() {
    let mut students: Vec<Student> = Vec::new();

    loop {
        let name: String = input("what is your name? ");
        let age: i32 = input_i32("what is your age? ");
        let grade: i32 = input_i32("what grade are you in? ");
        let overall_grade: f32 = input("what is your overall grade? ").parse::<f32>().unwrap();

        students.push(Student::create((&name, age, grade, overall_grade)));
    }
}

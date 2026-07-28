use std::io;
use std::io::Write;

use std::iter::zip;

use cliutils::input;
use cliutils::clear;

fn show_stocks(names: &Vec<String>, prices: &Vec<f32>) -> () {
  let mut x = zip(names, prices);
  for item in x {
    let x = item.0;
    let y = item.1;
    println!("{x} - ${y}");
  }
}

fn wait() -> () {
  input("---PRESS ENTER TO CONTINUE---".to_string());
}

fn get_price(stock_name: &String, names: &[String], prices: &[f32]) -> f32 {
  let i: usize = names.iter().position(|x| x == stock_name).unwrap();
  prices[i]
}

fn main() {
  let mut bank: f32 = 100.0;

  let stocks: Vec<String> = vec!["apple".to_string(), "nvidia".to_string(), "amd".to_string()];
  let mut prices: Vec<f32> = vec![100.0, 150.0, 200.0];
  
  clear();
  loop {
    println!("0. exit");
    println!("1. buy stock");
    println!("2. portfolio");
    println!("3. see stocks");
    let q: String = input(String::new());

    if q == "0" {
      break;
    } else if q == "1" {
      let stock_name = input("what stock do you want to buy? ".to_string());
      if !stocks.contains(&stock_name) {
        clear();
        println!("stock not found!");
        break;
      } else {
        let price: f32 = get_price(&stock_name, &stocks, &prices);
        if bank >= price {
          println!("you have ${bank}, this stock costs ${price}");
          println!("1. buy");
          println!("2. nevermind");
          let should_buy: String = input(String::new());
          if should_buy == "1" {
            let amount: f32 = input(String::from("how many do you want to buy? ")).parse::<f32>().unwrap();
            let total_price: f32 = price * amount;
            let has_enough: bool = bank >= total_price;

            if has_enough {

            } else {
              let difference: f32 = total_price - bank;
              println!("you don't have enough! you need ${difference} more");
            }
          }
        } else {
          let difference: f32 = price - bank;
          println!("you don't have enough for this stock! you need ${difference} more!");
        }
        wait();
      }
    } else if q == "2" {

    } else if q == "3" {
      clear();
      show_stocks(&stocks, &prices);
      wait();
    } else {
    }
    clear();
  }  
}
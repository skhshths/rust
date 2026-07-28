// parameter restrictions
use std::fmt::Display;
use std::cmp::PartialEq;

// std
use std::iter::zip;

// rand
use rand::Rng;
use rand::seq::IndexedRandom;

// cliutils
use cliutils::input;
use cliutils::clear_win;

fn zip_show<T: Display, B: Display>(a: &[T], b: &[B], middle: String, prefix: String) -> () {
  let x = zip(a, b);
  for item in x {
    let x = item.0;
    let y = item.1;
    println!("{x}{middle}{prefix}{y}");
  }
}

fn wait() -> () {
  input("---PRESS ENTER TO CONTINUE---".to_string());
}

fn get_index<T: PartialEq>(stock_name: &T, names: &[T]) -> usize {
  names.iter().position(|x| x == stock_name).unwrap()
}

fn get_random<T>(items: &[T]) -> &T {
  let chosen;
  chosen = items.choose(&mut rand::rng()).unwrap();
  chosen
}

fn update_prices(original_prices: &mut [i32]) -> Vec<i32> {
  let new_prices: Vec<i32> = Vec::new();
  let options: Vec<&str> = vec!["up", "up", "down"];
  for item in original_prices.iter_mut() {
    let choice: &str = get_random(&options);
    let change = rand::thread_rng().gen_range(0..=50);
    if choice == "up" {
      *item += change;
    } else {
      *item -= change;
    }
  }
  wait();
  new_prices
}

fn main() {
  let mut bank: i32 = 1000;
  let mut day: i32 = 1;

  let stocks: Vec<String> = vec!["apple".to_string(), "nvidia".to_string(), "amd".to_string()];
  let mut prices: Vec<i32> = vec![100, 150, 200];
  let mut owned: Vec<i32> = vec![0, 0, 0];
  
  clear_win();
  loop {
    println!("day {day}");
    println!("0. exit");
    println!("1. buy stock");
    println!("2. portfolio");
    println!("3. see stocks");
    println!("4. sell stocks");
    println!("5. sleep");
    let q: String = input(String::new()); 

    if q == "0" {
      break;
    } else if q == "1" {
      let stock_name = input("what stock do you want to buy? ".to_string());
      if !stocks.contains(&stock_name) {
        clear_win();
        println!("stock not found!");
        wait();
      } else {
        let price: i32 = prices[get_index(&stock_name, &stocks)];
        if bank >= price {
          println!("you have ${bank}, this stock costs ${price}");
          println!("1. buy");
          println!("2. nevermind");
          let should_buy: String = input(String::new());
          if should_buy == "1" {
            let amount: i32 = input(String::from("how many do you want to buy? ")).parse::<i32>().unwrap();
            let total_price: i32 = price * amount;
            let has_enough: bool = bank >= total_price;

            if has_enough {
              bank -= total_price;
              let names_index: usize = get_index(&stock_name, &stocks);
              owned[names_index] += amount;
            } else {
              let difference: i32 = total_price - bank;
              println!("you don't have enough! you need ${difference} more");
            }
          }
        } else {
          let difference: i32 = price - bank;
          println!("you don't have enough for this stock! you need ${difference} more!");
        }
        wait();
      }
    } else if q == "2" {
      clear_win();
      println!("you have ${bank}");
      zip_show(&stocks, &owned, " - ".to_string(), String::new());
      wait()
    } else if q == "3" {
      clear_win();
      zip_show(&stocks, &prices, " - ".to_string(), "$".to_string());
      wait();
    } else if q == "4" {
      let chosen_stock: String = input(String::from("what stock do you want to sell? "));
      if !stocks.contains(&chosen_stock) {
        println!("{chosen_stock} doesn't exist");
        wait();
      } else {
        let names_index: usize = get_index(&chosen_stock, &stocks);
        let amount_owned: i32 = owned[names_index];
        if amount_owned == 0 {
          println!("you don't have any {chosen_stock} shares.")
        } else {
          println!("you own {amount_owned}.");
          let amount_to_sell: i32 = input(String::from("how much do you want to sell?")).parse::<i32>().unwrap();
          let has_enough: bool = amount_to_sell >= amount_owned;
          if has_enough {
            owned[names_index] -= amount_to_sell;
            let profit: i32 = prices[names_index] * amount_to_sell;
            bank += profit;
          } else {
            println!("you don't have enough {chosen_stock} shares.");
          }
          wait();
        }
      }
    } else if q == "5" {
      day += 1;
      let new_prices: Vec<i32> = update_prices(&mut prices);
    } else {}
    clear_win();
  }  
}
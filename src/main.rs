// use std::io;

#[macro_use]
extern crate text_io;

fn main() {
  let star_line: String = String::from("******************************************");

  loop {
    println!("{}", star_line);
    println!("Please input your choice (through corresponding alphabet)");
    println!("a. Convert Celsius to Fahrenheit");
    println!("b. Convert Fahrenheit to Celsius");
    println!("c. Find nth value of the Fibonacci sequence");
    println!("Enter 'quit' without quotes to quit the program");
    println!("{}", star_line);
    println!("\n");

    let choice: String = read!();
    let choice = choice.as_str();

    match choice {
      "a" | "a." => to_fah(),
      "b" | "b." => to_cel(),
      "c" | "c." => nth_of_fib(),
      "quit" => {
        println!("Goodbye!");
        break;
      }
      _ => continue,
    }
  }
}

fn to_fah() {
  println!("Enter temperature in Celsius");
  let temp_cel: f64 = read!();

  let temp_fah = temp_cel * 1.8;
  println!("The temperature in Fahrenheit is {:.2}\n", temp_fah);
  return;
}

fn to_cel() {
  println!("Enter temperature in Fahrenheit");
  let temp_fah: f64 = read!();
  let temp_cel = temp_fah / 1.8;
  println!("The temperature in Celsius is {:.2}\n", temp_cel);
}

fn nth_of_fib() {
  println!("Enter the position of the term whose value is to be found");
  let position: u128 = read!();

  if position > 0 {
    let val_at_n: u128 = fib_val_at_n(position);
    println!(
      "The {} element of the fibonacci series is {}\n",
      position, val_at_n
    );
  } else {
    println!("Must be a valid term\n");
  }
}

fn fib_val_at_n(n: u128) -> u128 {
  let mut a: u128 = 0;
  let mut b: u128 = 1;

  for _ in 0..n {
    let tmp: u128 = a;
    a = b;
    b = tmp + a;
  }

  return b;
}

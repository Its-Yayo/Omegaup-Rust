// Luis Fer De Leon A01754574

use std::io;

fn main() {
  // Input variable
  let mut input = String::new();

  // Wrapped to i32
  io::stdin().read_line(&mut input).unwrap();
  let number_1: i32 = input.trim().parse().unwrap();
  
  // Input cleared
  input.clear();
  
  // Wrapped again to i32
  io::stdin().read_line(&mut input).unwrap();
  let number_2: i32 = input.trim().parse().unwrap();

  if number_1 > number_2 {
    println!("{}", number_1);
  } else if number_2 > number_1 {
    println!("{}", number_2);
  } else {
    println!("{}", number_1);
  }

use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();
  let number: i32 = input.trim().parse().unwrap();
  
  // Iterate from n to 1
  // Prints every number that gives mod() => 0
  // rev() is the same stuff as reverse() in Python 3.8+ ig
  for i in (1..=number).rev() {
    if number % i == 0 {
      println!("{}", i);
    }
  }
}

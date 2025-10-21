use std::io::{self, Read};

fn main() {
  // Stdin n and stdin vec! mapped through 0 to n
  let mut input = String::new();
  io::stdin().read_to_string(&mut input).unwrap();
  
  let mut iter = input.split_whitespace();
  
  let n: usize = iter.next().unwrap().parse().unwrap();
  
  let mut numbers: Vec<u32> = (0..n).map(|_| iter.next().unwrap().parse().unwrap()).collect();

    
  numbers.sort_by_key(|&x| (x.count_ones(), x));
  
  for (i, num) in numbers.iter().enumerate() {
    if i > 0 {
        print!(" ");
      }
      print!("{}", num);
    }
  println!();
  
}

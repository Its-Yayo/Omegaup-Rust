use std::io;

fn main() {
  let mut input = String::new();
  
  // Init number (sequence's size)
  io::stdin().read_line(&mut input).unwrap();
  let n: usize = input.trim().parse().unwrap();
  
  // Init vector for sequence
  let mut numbers = Vec::new();

  // Loop from 0 to n to append elements, no i this time cuz of inputs
  for _ in 0..n {
      input.clear();
      
      io::stdin().read_line(&mut input).unwrap();
      let num: i32 = input.trim().parse().unwrap();

      numbers.push(num);
    }

    // Max value from the vector
    let max_value = numbers.iter().max().unwrap();
    println!("{}", max_value);
}

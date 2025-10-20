use std::io;

fn main() {
  // Logic works like this: np(n) divides by itself until n = >=2
  // when n == 1, it computes the results through sum(2 to (n / 2))
  // It follows => np(n) = n + np(n / 2)

  let mut input = String::new();

  io::stdin().read_line(&mut input).unwrap();
  let n: i32 = input.trim().parse().unwrap();
  
  // Mut variables and accumulator
  let mut acc: i32 = 0;
  let mut nn: i32 = n; // We need the mutable input

  while nn > 0 {
    acc += nn;
    nn = nn / 2;
  }

  println!("{}", acc);
  
}

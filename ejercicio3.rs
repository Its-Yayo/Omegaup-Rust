use std::io;

fn main() {
    let mut input = String::new();

    // Vector's size
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap(); // usize for dynamic arrays

    input.clear();

    // Vector
    io::stdin().read_line(&mut input).unwrap();
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    input.clear();

    // Condition 0 or 1
    io::stdin().read_line(&mut input).unwrap();
    let mode: i32 = input.trim().parse().unwrap();

    // Vector to filter if cond == 0, if yes, it will output even numbers. If not, it will output odd numbers
    // collect() is like .join() in Python
    let result: Vec<i32> = numbers
        .into_iter()
        .filter(|&x| if mode == 0 { x % 2 == 0 } else { x % 2 != 0 })
        .collect();

    // Loop to output the previous filter on each element, adding whitespaces
    for num in result {
        println!("{} ", num);
    }
}

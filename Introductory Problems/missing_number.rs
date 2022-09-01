use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input.");
    let n: u64 = n.trim().parse().expect("Enter a number!");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");
    let input: Vec<u64> = input
        .trim()
        .split(" ")
        .map(|x| x.parse().expect("Failed to parse input."))
        .collect();

    let natural_sum = n * (n + 1) / 2;
    let input_sum: u64 = input.into_iter().sum();
    println!("{}", natural_sum - input_sum);
}

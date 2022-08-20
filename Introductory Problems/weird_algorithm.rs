use std::io;

fn main() {
    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read input.");

    let mut n: u64 = n.trim().parse().expect("Enter a number!");

    loop {
        print!("{} ", n);
        if n == 1 {
            break;
        } else if n % 2 == 0 {
            n = n / 2;
        } else {
            n = n * 3 + 1;
        }
    }
    println!("");
}

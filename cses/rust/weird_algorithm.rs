use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut n: i64 = input.trim().parse().unwrap();

    while n != 1 {
        print!("{} ", n);

        if n % 2 == 0 {
            n /= 2;
        } else {
            n = n * 3 + 1;
        }
    }

    println!("{}", n);
}

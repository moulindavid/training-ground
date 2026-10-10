use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let sum: usize = iter.map(|x| x.parse::<usize>().unwrap()).sum();
    let expected = n * (n + 1) / 2;

    println!("{}", expected - sum);
}

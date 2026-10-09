use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let numbers: Vec<usize> = iter.map(|x| x.parse().unwrap()).collect();
    let mut sum = 0;
    let mut expected = 0;
    for i in 1..n {
        expected += i;
        sum += numbers[i - 1];
    }
    expected += n;
    println!("{}", expected - sum);
}

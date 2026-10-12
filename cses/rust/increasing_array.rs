use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut nums: Vec<i64> = iter.take(n).map(|s| s.parse().unwrap()).collect();
    let mut turn: i64 = 0;

    for i in 0..(n - 1) {
        let j = i + 1;
        let current: i64 = nums[i];
        let mut above: i64 = nums[j];

        if above < current {
            let diff = current - above;
            above += diff;
            turn += diff;
        }
        nums[j] = above;
    }
    println!("{}", turn);
}

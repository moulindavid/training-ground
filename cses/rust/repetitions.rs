use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut current_char = ' ';
    let mut current_rep = 1;
    let mut longest_rep = 1;

    let input_string = input.trim();
    for char in input_string.chars() {
        if current_char == char {
            current_rep += 1;
        } else if current_rep > longest_rep {
            longest_rep = current_rep;
            current_rep = 1;
        } else {
            current_rep = 1;
        }
        current_char = char;
    }

    if current_rep > longest_rep {
        longest_rep = current_rep;
    }

    println!("{}", longest_rep);
}

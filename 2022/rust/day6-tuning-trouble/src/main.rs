use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    part2(input);
}

fn part1(input: String) {
    println!("{}", solve(input, 4));
}

fn part2(input: String) {
    println!("{}", solve(input, 14));
}

fn solve(input: String, num: usize) -> usize {
    let chars: Vec<char> = input.chars().collect();
    for (i, _c) in chars.iter().enumerate() {
        let set = chars[i..i + num].iter().collect::<HashSet<_>>();
        if set.len() == num {
            return i + num;
        }
    }

    0
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

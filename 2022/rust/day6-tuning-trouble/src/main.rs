use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    part1(input);
}

fn part1(input: String) {
    let mut set: HashSet<char> = HashSet::new();
    let chars: Vec<char> = input.chars().collect();
    for (i, c) in chars.iter().enumerate() {
        let set = chars[i..i + 4].iter().collect::<HashSet<_>>();
        if set.len() == 4 {
            println!("{}", i + 4);
            break;
        }
    }
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

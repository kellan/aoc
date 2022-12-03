use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    part1();
    //part2();
}

fn part1() {
    let input = read_stdin();

    let answer: u32 = input
        .lines()
        .map(|line| match find_dupe(line) {
            Some(j) => score(j),
            None => 0,
        })
        .sum();

    println!("{}", answer);
}

fn part2() {
    let input = read_stdin();

    let answer: u32 = input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| match find_chunk_dupe(chunk) {
            Some(c) => score(c),
            None => 0,
        })
        .sum();

    println!("{}", answer);
}

// borrowed from b0rk
// https://github.com/jvns/aoc2022/blob/main/day03/solve.rs
fn score(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 64 + 26
    }
}

fn find_chunk_dupe(chunk: &[&str]) -> Option<char> {
    chunk[0]
        .chars()
        .find(|&c| chunk[1].contains(c) && chunk[2].contains(c))
}

fn find_dupe(line: &str) -> Option<char> {
    let contents = line.split_at(line.len() / 2);
    let left: HashSet<char> = contents.0.chars().collect();
    let right: HashSet<char> = contents.1.chars().collect();

    left.intersection(&right).next().copied()
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

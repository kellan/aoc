use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn main() {
    part1();
}

fn part1() {
    let buffer = read_stdin();
    let mut answer: u32 = 0;
    let priorities = priorities();

    for line in buffer.lines() {
        let dupe = find_dupe(line);
        if let Some(j) = dupe {
            let score = priorities.get(&j).unwrap_or(&0);
            answer += score;
        };
    }
    println!("{}", answer);
}

fn priorities() -> HashMap<char, u32> {
    let mut priorities: HashMap<char, u32> = HashMap::new();
    let mut p: u32 = 1;

    ('a'..='z').for_each(|c| {
        priorities.insert(c, p);
        p += 1
    });
    ('A'..='Z').for_each(|c| {
        priorities.insert(c, p);
        p += 1
    });

    priorities
}

fn find_dupe(line: &str) -> Option<char> {
    let mut seen: HashSet<char> = HashSet::new();
    let mid = line.len() / 2;
    let mut dupe = None;
    for (i, j) in line.chars().enumerate() {
        if i < mid {
            seen.insert(j);
        } else {
            if seen.contains(&j) {
                dupe = Some(j);
            }
        }
    }

    dupe
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

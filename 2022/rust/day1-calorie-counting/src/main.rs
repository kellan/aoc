use std::io::{self, Read};

fn main() {
    //part1();
    part2();
}

fn lines() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

fn sums(buffer: String) -> Vec<i32> {
    let mut elves = Vec::new();
    let mut accum = 0;

    for line in buffer.lines() {
        if !line.trim().is_empty() {
            let num = line.parse::<i32>().unwrap();
            accum += num;
        } else {
            elves.push(accum);
            accum = 0;
        }
    }

    elves
}

fn part2() {
    let buffer = lines();
    let mut sums = sums(buffer);
    sums.sort();
    sums.reverse();
    println!("{}", sums[0] + sums[1] + sums[2]);
}
fn part1() {
    let buffer = lines();
    let mut sums = sums(buffer);
    sums.sort();
    sums.reverse();
    println!("{}", sums[0]);
}

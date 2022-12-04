use itertools::Itertools;
use std::io::{self, Read};

fn main() {
    //part1();
    part2();
}

fn part1() {
    let input = read_stdin();
    let overlaps = input
        .lines()
        .map(|line| line.split(','))
        .filter_map(|r| r.collect_tuple())
        .map(|(left, right)| contained(left, right))
        .filter(|x| *x)
        .collect::<Vec<bool>>();

    //dbg!(&overlaps);
    let answer = overlaps.len();
    dbg!(answer);
}

fn part2() {
    let input = read_stdin();
    let overlaps = input
        .lines()
        .map(|line| line.split(','))
        .filter_map(|r| r.collect_tuple())
        .map(|(left, right)| overlaps(left, right))
        .filter(|x| *x)
        .collect::<Vec<bool>>();

    //dbg!(&overlaps);
    let answer = overlaps.len();
    dbg!(answer);
}

fn contained(left: &str, right: &str) -> bool {
    let left = left
        .split('-')
        .map(|i| i.parse().unwrap())
        .take(2)
        .collect::<Vec<u32>>();

    let right = right
        .split('-')
        .map(|i| i.parse().unwrap())
        .take(2)
        .collect::<Vec<u32>>();

    (left[0] <= right[0] && left[1] >= right[1]) || (right[0] <= left[0] && right[1] >= left[1])
}

fn overlaps(left: &str, right: &str) -> bool {
    let left = left
        .split('-')
        .map(|i| i.parse().unwrap())
        .take(2)
        .collect::<Vec<u32>>();

    let right = right
        .split('-')
        .map(|i| i.parse().unwrap())
        .take(2)
        .collect::<Vec<u32>>();

    !(left[1] < right[0] || right[1] < left[0])
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

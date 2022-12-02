use std::io::{self, Read};

fn main() {
    //part1();
    part2();
}

fn part1() {
    let buffer = read_stdin();
    let mut total_score = 0;
    for line in buffer.lines() {
        total_score += score_part1(line.trim());
    }
    println!("{}", total_score);
}

fn part2() {
    let buffer = read_stdin();
    let mut total_score = 0;
    for line in buffer.lines() {
        total_score += score_part2(line.trim());
    }
    println!("{}", total_score);
}

fn score_part2(play: &str) -> i32 {
    let part1_play = match play {
        "A X" => "A Z",
        "A Y" => "A X",
        "A Z" => "A Y",
        "B X" => "B X",
        "B Y" => "B Y",
        "B Z" => "B Z",
        "C X" => "C Y",
        "C Y" => "C Z",
        "C Z" => "C X",
        &_ => "BAD INPUT",
    };

    let score = score_part1(part1_play);
    score
}

fn score_part1(play: &str) -> i32 {
    let score = match play {
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,
        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,
        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,
        &_ => 0,
    };

    score
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    let (stacks, moves) = parse_input(&input);
    //part1(stacks, moves);
    part2(stacks, moves);
}

fn part1(mut stacks: Vec<Vec<char>>, moves: Vec<Move>) {
    for m in moves {
        stacks = move_one(stacks, &m);
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!("");
}

fn part2(mut stacks: Vec<Vec<char>>, moves: Vec<Move>) {
    for m in moves {
        stacks = move_many(stacks, &m);
    }

    for stack in stacks {
        print!("{}", stack.last().unwrap());
    }
    println!("");
}

fn move_one(mut stacks: Vec<Vec<char>>, m: &Move) -> Vec<Vec<char>> {
    let mid = stacks[m.from - 1].len() - m.cnt;
    let drained: Vec<char> = stacks[m.from - 1].drain(mid..).rev().collect();
    stacks[m.to - 1].extend(drained);

    stacks
}

fn move_many(mut stacks: Vec<Vec<char>>, m: &Move) -> Vec<Vec<char>> {
    let mid = stacks[m.from - 1].len() - m.cnt;
    let drained: Vec<char> = stacks[m.from - 1].drain(mid..).collect();
    stacks[m.to - 1].extend(drained);

    stacks
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Move>) {
    let mut parts = input.split("\n\n");

    let (stacks, moves) = (parts.next().unwrap(), parts.next().unwrap());

    let stacks = parse_stacks(stacks);
    let moves = parse_moves(moves);

    (stacks, moves)
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = vec![];

    input.lines().rev().skip(1).for_each(|line| {
        let chars: Vec<char> = line.chars().collect();

        while stacks.len() < ((chars.len() + 1) / 4) {
            stacks.push(Vec::new());
        }

        let mut i = 1;

        while i < chars.len() {
            if chars[i].is_alphabetic() {
                stacks[(i - 1) / 4].push(chars[i]);
            }
            i += 4;
        }
    });

    stacks
}

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    cnt: usize,
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|l| {
            // move 1 from 2 to 1
            let parts: Vec<&str> = l.split_whitespace().collect();

            Move {
                cnt: parts[1].parse().unwrap(),
                from: parts[3].parse().unwrap(),
                to: parts[5].parse().unwrap(),
            }
        })
        .collect()
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

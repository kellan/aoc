use std::{
    f32::consts::E,
    io::{self, Read},
};

fn main() {
    let input = read_stdin();
    let ops = parse(input);
    // let score = part1(ops);
    // dbg!(score);
    part2(ops);
}

fn part2(ops: Vec<Op>) {
    let mut x = 1;
    let mut cycle = 1;

    fn update_crt(cycle: i32, x: i32) {
        let cursor = (cycle - 1) % 40;

        if cursor - 1 <= x && x <= cursor + 1 {
            print!("{}", "#");
        } else {
            print!("{}", ".");
        }
        if cycle % 40 == 0 {
            println!();
        }
    }

    for op in ops {
        update_crt(cycle, x);
        match op {
            Op::Noop => (),
            Op::AddX(add) => {
                cycle += 1;
                update_crt(cycle, x);
                x += add;
            }
        }
        cycle += 1;
    }

    //dbg!(crt);
}
fn part1(ops: Vec<Op>) -> i32 {
    let mut x = 1;
    let mut cycle = 1;
    let mut signal_strengths: Vec<i32> = Vec::new();

    fn emit(cycle: i32, x: i32, signals: &mut Vec<i32>) {
        if cycle == 20 {
            signals.push(cycle * x);
        } else if (cycle - 20) % 40 == 0 {
            signals.push(cycle * x);
        }
    }

    for op in ops {
        //println!("{:?}", op);
        match op {
            Op::Noop => (),
            Op::AddX(add) => {
                cycle += 1;
                emit(cycle, x, &mut signal_strengths);
                x += add;
            }
        }
        cycle += 1;
        emit(cycle, x, &mut signal_strengths);
    }

    // dbg!(signal_strengths);
    signal_strengths.iter().sum()
}

#[derive(Debug)]
enum Op {
    Noop,
    AddX(i32),
}

fn parse(input: String) -> Vec<Op> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let op = parts.next().unwrap();
            match op {
                "noop" => Op::Noop,
                "addx" => Op::AddX(parts.next().unwrap().parse().unwrap()),
                _ => panic!("Unknown op"),
            }
        })
        .collect()
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    let sensors = parse(input);
    dbg!(sensors);
}

fn parse(input: String) -> Vec<Sensor> {
    let mut sensors: Vec<Sensor> = vec![];

    for line in input.lines() {
        if let Some(captures) = parse_input_line(line) {
            let sx = captures[1].parse().unwrap();
            let sy = captures[2].parse().unwrap();
            let bx = captures[3].parse().unwrap();
            let by = captures[4].parse().unwrap();

            let s = Sensor {
                coords: Point { x: sx, y: sy },
                beacon: Point { x: bx, y: by },
            };

            sensors.push(s);
        }
    }

    sensors
}

#[derive(Debug)]
struct Sensor {
    coords: Point,
    beacon: Point,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Thou Shalt Not Compile Regular Expressions In A Loop
// https://github.com/rust-lang/regex/blob/master/PERFORMANCE.md#thou-shalt-not-compile-regular-expressions-in-a-loop
fn parse_input_line(text: &str) -> Option<regex::Captures> {
    lazy_static! {
        static ref LINE_REGEX: Regex = Regex::new(
            r"Sensor at x=([-\d]+), y=([-\d]+): closest beacon is at x=([-\d]+), y=([-\d]+)"
        )
        .unwrap();
    }
    LINE_REGEX.captures(text)
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

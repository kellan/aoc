use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    let sensors = parse(input);
    //solve1(sensors, 2000000);
    solve2(&sensors, 20);
}

fn solve2(sensors: &Vec<Sensor>, max: i32) {
    let (min_x, min_y) = (0, 0);
    let (max_x, max_y) = (max, max);

    let mut ranges: Vec<Vec<Range>> = vec![];

    for y in min_y..=max_y {
        let mut y_range: Vec<Range> = vec![];

        for s in sensors.iter() {
            let dy = i32::abs(s.coords.y - y);
            if dy > s.radius {
                continue;
            }
            let dx = (s.radius - dy) / 2;

            y_range.push(Range {
                left: std::cmp::max(s.coords.x - dx, 0),
                right: std::cmp::min(s.coords.x + dx, max_x),
            })
        }

        dbg!(&y_range);
        let merged = Range::merge(&y_range);
        dbg!(merged);
        break;
    }

    dbg!(ranges);
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Range {
    left: i32,
    right: i32,
}

impl Range {
    fn merge(ranges: &Vec<Range>) -> Vec<Range> {
        let mut ranges = ranges.clone();
        ranges.sort();
        ranges.reverse(); // walk it from the back, because we can't pop front

        let mut merged: Vec<Range> = Vec::new();

        while let Some(r) = ranges.pop() {
            if merged.is_empty() {
                merged.push(r);
                continue;
            }

            // if doesn't overlap, new interval
            if merged[0].right < r.left {
                merged.push(r);
            } else if merged[0].right < r.right {
                let merged_range = Range {
                    left: merged[0].left,
                    right: r.right,
                };
                merged.pop();
                merged.push(merged_range);
            }
        }
        merged
    }
}

#[derive(Debug, Copy, Clone)]
struct Sensor {
    coords: Point,
    beacon: Point,
    radius: i32,
}

impl Sensor {
    fn in_range(&self, p: &Point) -> bool {
        self.coords.manhattan_distance(p) <= self.radius
    }
}

fn cave_max_min_x(sensors: &Vec<Sensor>) -> (i32, i32) {
    let max_x = sensors.iter().map(|s| s.coords.x + s.radius).max().unwrap();
    let min_x = sensors.iter().map(|s| s.coords.x - s.radius).min().unwrap();

    (min_x, max_x)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self, p: &Point) -> i32 {
        i32::abs(self.x - p.x) + i32::abs(self.y - p.y)
    }
}

fn solve1(sensors: Vec<Sensor>, row: i32) {
    let (min_x, max_x) = cave_max_min_x(&sensors);
    dbg!(&min_x, &max_x);

    let beacons: Vec<Point> = sensors.iter().map(|s| s.beacon.to_owned()).collect();
    let mut points: Vec<i32> = vec![];

    let relevant_sensors: Vec<&Sensor> = sensors
        .iter()
        .filter(|s| i32::abs(s.coords.y - row) <= s.radius)
        .collect();

    for x in min_x..=max_x {
        let p = Point { x: x, y: row };
        if beacons.contains(&p) {
            continue;
        }

        for s in relevant_sensors.iter() {
            if s.in_range(&p) {
                points.push(p.x);
                break;
            }
        }
    }
    dbg!(points.len());
}

fn parse(input: String) -> Vec<Sensor> {
    let mut sensors: Vec<Sensor> = vec![];

    for line in input.lines() {
        if let Some(captures) = parse_input_line(line) {
            let sx = captures[1].parse().unwrap();
            let sy = captures[2].parse().unwrap();
            let bx = captures[3].parse().unwrap();
            let by = captures[4].parse().unwrap();

            let coords = Point { x: sx, y: sy };
            let beacon = Point { x: bx, y: by };
            let radius = coords.manhattan_distance(&beacon);

            let s = Sensor {
                coords,
                beacon,
                radius,
            };

            sensors.push(s);
        }
    }

    sensors
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

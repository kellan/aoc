use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    let sensors = parse(input);
    //dbg!(&sensors);
    //dbg!(&sensors[6]);
    //dbg!(sensors[6].boundary());
    solve1(sensors, 2000000);
}

fn solve1(sensors: Vec<Sensor>, row: i32) {
    let mut excluded = 0;

    let min_x = sensors
        .iter()
        .map(|s| s.boundary())
        .flatten()
        .map(|point| point.x)
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|s| s.boundary())
        .flatten()
        .map(|point| point.x)
        .max()
        .unwrap();

    dbg!(&min_x, &max_x);

    let beacons: Vec<Point> = sensors.iter().map(|s| s.beacon.to_owned()).collect();

    for x in min_x..=max_x {
        let p = Point { x: x, y: row };

        if beacons.contains(&p) {
            continue;
        }

        for s in sensors.iter() {
            if (poly_contains(&s.boundary(), &Point { x: x, y: row })) {
                excluded += 1;
                break;
            }
        }
    }

    dbg!(excluded);
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

impl Sensor {
    fn boundary(&self) -> Vec<Point> {
        let radius = self.coords.manhattan_distance(&self.beacon);
        let boundary: Vec<Point> = vec![
            Point {
                x: self.coords.x - radius,
                y: self.coords.y,
            },
            Point {
                x: self.coords.x,
                y: self.coords.y - radius,
            },
            Point {
                x: self.coords.x + radius,
                y: self.coords.y,
            },
            Point {
                x: self.coords.x,
                y: self.coords.y + radius,
            },
        ];

        boundary
    }
}

fn poly_contains(poly: &Vec<Point>, point: &Point) -> bool {
    let x = point.x;
    let y = point.y;

    let mut contains = false;
    for (i, p) in poly.iter().enumerate() {
        let j = poly.len() - 1;

        //dbg!(i, p);
        let xi = poly[i].x;
        let yi = poly[i].y;

        let xj = poly[j].x;
        let yj = poly[j].y;

        let intersect: bool = (yi > y) != (yj > y) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi);
        if (intersect) {
            contains = !contains;
        }
    }

    contains
}

//     let mut k_prev = 0;
//     let mut result = false;

//     for (k, p) in self.coordinates.iter().enumerate() {
//         if k <= 0 {
//             k_prev = self.coordinates.len() - 1
//         } else {
//             k_prev = k - 1
//         }

//         let iflng = p[1] < point.lng && self.coordinates[k_prev][1] >= point.lng
//             || self.coordinates[k_prev][1] < point.lng && p[1] >= point.lng;
//         let iflat = p[0] <= point.lat || self.coordinates[k_prev][0] <= point.lat;

//         if iflng && iflat {
//             if p[0]
//                 + (point.lng - p[1]) / (self.coordinates[k_prev][1] - p[1])
//                     * (self.coordinates[k_prev][0] - p[0])
//                 < point.lat
//             {
//                 result = !result
//             }
//         }
//     }

//     return result;
// }

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn manhattan_distance(&self, p: &Point) -> i32 {
        i32::abs(self.x - p.x) + i32::abs(self.y - p.y)
    }
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

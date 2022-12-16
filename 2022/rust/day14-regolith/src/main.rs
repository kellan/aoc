use itertools::Itertools;
use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    let cave = parse(input);
    solve(cave);
}

fn solve(mut cave: Cave) {
    let mut sand = Point(500, 0);
    let mut is_blocked = false;
    let mut endless_void = false;

    while (!endless_void) {
        while (!is_blocked) {
            (is_blocked, sand) = cave.tick(sand.to_owned());
        }
        println!("{:?} {:?}", is_blocked, sand);
        cave.sand.insert(sand.to_owned());
        println!("{:?} {:?} {:?}", cave.sand.len(), sand.1, cave.floor);

        if sand.1 >= cave.floor {
            endless_void = true;
        }
        is_blocked = false;
        sand = Point(500, 0);
    }
    dbg!(cave.sand.len());
}

fn parse(input: String) -> Cave {
    let mut cave = Cave::new();

    for line in input.lines() {
        let mut parts = line.split(" -> ").collect::<Vec<&str>>();
        let points: Vec<Point> = parts
            .iter()
            .map(|part| {
                let mut split = part.split(",");
                Point(
                    split.next().unwrap().parse().unwrap(),
                    split.next().unwrap().parse().unwrap(),
                )
            })
            .collect();

        cave.add_points(points);
    }

    cave
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Point(i32, i32);

#[derive(Debug)]
struct Cave {
    sand: HashSet<Point>,
    rocks: HashSet<Point>,
    floor: i32,
}

fn minmax(a: i32, b: i32) -> (i32, i32) {
    let x1 = std::cmp::min(a, b);
    let x2 = std::cmp::max(a, b);
    (x1, x2)
}
impl Cave {
    fn new() -> Cave {
        Self {
            sand: HashSet::new(),
            rocks: HashSet::new(),
            floor: 0,
        }
    }

    fn add_points(&mut self, points: Vec<Point>) {
        // dbg!(&points);

        for (from, to) in points.iter().tuple_windows() {
            if from.0 != to.0 {
                let (x1, x2) = minmax(from.0, to.0);
                for x in x1..=x2 {
                    let p = Point(x, from.1);
                    self.rocks.insert(p);
                }
            }

            if from.1 != to.1 {
                let (y1, y2) = minmax(from.1, to.1);

                for y in y1..=y2 {
                    let p = Point(from.0, y);
                    self.rocks.insert(p);

                    if y > self.floor {
                        self.floor = y;
                    }
                }
            }
        }
    }

    fn tick(&mut self, sand: Point) -> (bool, Point) {
        for dir in [Dir::Down, Dir::Left, Dir::Right] {
            if !self.is_blocked(&sand, dir) {
                return (false, sand.dir(dir));
            }
        }
        (true, sand)
    }

    fn is_blocked(&self, sand: &Point, dir: Dir) -> bool {
        let p = sand.dir(dir);
        self.rocks.contains(&p) || self.sand.contains(&p)
    }
}

impl Point {
    fn dir(&self, dir: Dir) -> Point {
        match dir {
            Dir::Down => {
                let x = self.0;
                let y = self.1 + 1;
                Point(x, y)
            }
            Dir::Left => {
                let x = self.0 - 1;
                let y = self.1 + 1;
                Point(x, y)
            }
            Dir::Right => {
                let x = self.0 + 1;
                let y = self.1 + 1;
                Point(x, y)
            }
        }
    }
}

#[derive(Copy, Clone)]
enum Dir {
    Down,
    Left,
    Right,
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

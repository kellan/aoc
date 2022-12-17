use itertools::Itertools;
use std::collections::HashSet;
use std::fmt;
use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    let cave = parse(input);
    //    let sand_cnt = solve1(cave);
    let sand_cnt = solve2(cave);
    dbg!(sand_cnt);
}

fn solve2(mut cave: Cave) -> usize {
    let mut sand = Point(500, 0);
    let mut is_blocked = false;
    let mut is_cave_full = false;
    cave.floor = cave.floor + 1;

    while !is_cave_full {
        while !is_blocked {
            (is_blocked, sand) = cave.tick(sand.to_owned());
            cave.last_sand = sand.to_owned();
            if sand.1 >= cave.floor {
                is_blocked = true;
            }
        }
        cave.sand.insert(sand.to_owned());
        if sand == Point(500, 0) {
            is_cave_full = true;
        }
        is_blocked = false;
        sand = Point(500, 0);
    }
    println!("{}", cave);
    cave.sand.len()
}
fn solve1(mut cave: Cave) -> usize {
    let mut sand = Point(500, 0);
    let mut is_blocked = false;
    let mut endless_void = false;

    // while we haven't passed max_y
    while !endless_void {
        // while sand can still fall
        while !is_blocked {
            // move sand until we're blocked
            (is_blocked, sand) = cave.tick(sand.to_owned());
            cave.last_sand = sand.to_owned();
            // check if we've fallen into the endless void
            if sand.1 >= cave.floor {
                endless_void = true;
                break;
            }
        }
        cave.sand.insert(sand.to_owned());

        is_blocked = false;
        sand = Point(500, 0);
        println!("{}", cave);
        //cave.draw();
    }

    cave.sand.len() - 1
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
    loop_n: i32,
    last_sand: Point,
    first_call_to_draw: bool,
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
            loop_n: 0,
            last_sand: Point(0, 0),
            first_call_to_draw: true,
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
        self.loop_n += 1;

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

impl fmt::Display for Cave {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let min_x: i32 = std::cmp::min(
            self.rocks.iter().map(|p| p.0).min().unwrap(),
            self.sand.iter().map(|p| p.0).min().unwrap(),
        );

        let max_x: i32 = std::cmp::max(
            self.rocks.iter().map(|p| p.0).max().unwrap(),
            self.sand.iter().map(|p| p.0).max().unwrap(),
        );
        let min_y: i32 = 0;
        let max_y: i32 = self.floor;

        write!(
            f,
            "Total sand {} Loop {} Last sand {},{}\n",
            self.sand.len(),
            self.loop_n,
            self.last_sand.0,
            self.last_sand.1
        );
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let mut c = '.';
                if self.rocks.contains(&Point(x, y)) {
                    c = '#';
                } else if self.sand.contains(&Point(x, y)) {
                    c = 'O';
                }
                write!(f, "{}", c)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
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

#[derive(Debug, Copy, Clone)]
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

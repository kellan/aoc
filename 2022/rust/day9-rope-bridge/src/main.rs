use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    let commands = parse(input);
    solve(commands);
}

fn solve(commands: Vec<(String, i32)>) {
    let mut rope = Rope::new();
    for cmd in commands {
        rope.cmd(cmd);
    }
    println!("{}", rope.tail_seen.len());
}

#[derive(Debug)]
struct Rope {
    head: Point,
    tail: Point,
    tail_seen: HashSet<Point>,
}

impl Rope {
    fn new() -> Rope {
        Rope {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
            tail_seen: HashSet::from([Point { x: 0, y: 0 }]),
        }
    }

    fn cmd(&mut self, cmd: (String, i32)) {
        for _ in (0..cmd.1) {
            match cmd.0.as_ref() {
                "R" => self.head.x += 1,
                "L" => self.head.x -= 1,
                "U" => self.head.y -= 1,
                "D" => self.head.y += 1,
                _ => {}
            }
            self.update_tail()
        }
    }

    fn update_tail(&mut self) {
        if self.head.distance(&self.tail) >= 2.0 {
            if self.head.x != self.tail.x {
                if self.head.x > self.tail.x {
                    self.tail.x += 1;
                } else {
                    self.tail.x -= 1;
                }
            }

            if self.head.y != self.tail.y {
                if self.head.y > self.tail.y {
                    self.tail.y += 1;
                } else {
                    self.tail.y -= 1;
                }
            }
            self.tail_seen.insert(self.tail.clone());
        }
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn distance(&self, p: &Point) -> f64 {
        let f = f64::sqrt((i32::pow(p.x - self.x, 2) + i32::pow(p.y - self.y, 2)) as f64);

        f
    }
}

fn parse(input: String) -> Vec<(String, i32)> {
    let mut commands: Vec<(String, i32)> = Vec::new();

    for l in input.lines() {
        let parts = l.split_whitespace().collect::<Vec<_>>();
        let cmd: (String, i32) = (parts[0].to_owned(), parts[1].parse().unwrap());
        commands.push(cmd);
    }

    commands
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rope() {
        let mut rope = Rope::new();

        assert_eq!(rope.tail_seen.len(), 1);
        let cmd = (String::from('R'), 4);
        rope.cmd(cmd);
        assert_eq!(rope.tail_seen.len(), 4);
        let cmd = (String::from('U'), 4);
        rope.cmd(cmd);
        assert_eq!(rope.tail_seen.len(), 7);
    }
}

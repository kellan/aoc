use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    let (grid, start, end, low_points) = parse(input);
    dbg!(&start, &end);
    // part 1
    // let end = HashSet::from([end]);
    // let (found, dist) = bfs(grid, start, end, false);
    // part 2
    dbg!(&low_points);
    let (found, dist) = bfs(grid, end, low_points, true);
    dbg!(found);
    dbg!(dist);
}

fn bfs(
    grid: HashMap<Point, u32>,
    start: Point,
    end: HashSet<Point>,
    downhill: bool,
) -> (Point, u32) {
    let mut q: VecDeque<(Point, u32)> = VecDeque::from([(start, 0)]);
    let mut visited: HashSet<Point> = HashSet::new();

    while let Some((p, dist)) = q.pop_front() {
        if end.contains(&p) {
            return (p, dist);
        }

        for neighbor in neighbors(&grid, &p, downhill) {
            if visited.contains(&neighbor) {
                continue;
            }
            q.push_back((neighbor.to_owned(), dist + 1));
            visited.insert(neighbor.to_owned());
        }
    }

    (Point(0, 0), 0)
}

fn neighbors(grid: &HashMap<Point, u32>, point: &Point, downhill: bool) -> Vec<Point> {
    let mut neighbors: Vec<Point> = Vec::new();
    let height = *grid.get(point).unwrap() as i32;

    const DIRECTIONS: [(i32, i32); 4] = [
        (-1, 0), // up
        (1, 0),  // down
        (0, 1),  // right
        (0, -1), // left
    ];

    for delta in DIRECTIONS {
        let dh = point.0 as i32 + delta.0;
        let dw = point.1 as i32 + delta.1;
        let n = Point(dh as u32, dw as u32);
        if let Some(n_height) = grid.get(&n) {
            if downhill {
                if height - *n_height as i32 <= 1 {
                    neighbors.push(n);
                }
            } else if *n_height as i32 - height <= 1 {
                neighbors.push(n);
            }
        }
    }
    neighbors
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Point(u32, u32);

fn parse(input: String) -> (HashMap<Point, u32>, Point, Point, HashSet<Point>) {
    let mut grid: HashMap<Point, u32> = HashMap::new();
    let mut start = Point(0, 0);
    let mut end = Point(0, 0);
    let mut low_points: HashSet<Point> = HashSet::new();

    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.chars().enumerate() {
            match c {
                'S' => {
                    start = Point(i as u32, j as u32);
                    grid.insert(start.to_owned(), char_to_num('a'));
                }
                'E' => {
                    end = Point(i as u32, j as u32);
                    grid.insert(end.to_owned(), char_to_num('z'));
                }
                c => {
                    let p = Point(i as u32, j as u32);
                    grid.insert(p.to_owned(), char_to_num(c));
                    if c == 'a' {
                        low_points.insert(p.to_owned());
                    }
                }
            }
        }
    }
    //dbg!(grid.get(&Point(2, 5)));
    (grid, start, end, low_points)
}

fn char_to_num(c: char) -> u32 {
    if c.is_ascii_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 64 + 26
    }
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

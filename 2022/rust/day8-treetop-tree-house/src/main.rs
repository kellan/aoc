use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    let forest = parse(input);
    //    dbg!(forest);
    // println!("{}", visible(0, 1, &forest));
    // println!("{}", visible(1, 1, &forest));
    // println!("{}", visible(2, 2, &forest));
    let visible_trees = visible_trees(&forest);
    dbg!(visible_trees);
}

fn visible_trees(forest: &Vec<Vec<u32>>) -> u32 {
    let mut found = 0;
    for i in 0..forest.len() {
        for j in 0..forest[i].len() {
            //println!("{},{}", i, j);
            if visible(i, j, forest) {
                found += 1;
            }
        }
    }

    found
}

fn visible(x: usize, y: usize, forest: &Vec<Vec<u32>>) -> bool {
    if x == 0 || y == 0 {
        return true;
    }

    let left = &forest[x][0..y];
    if is_tallest(forest[x][y], left) {
        return true;
    }

    let right = &forest[x][y + 1..];
    if is_tallest(forest[x][y], right) {
        return true;
    }

    let up = &forest[0..x].iter().map(|v| v[y]).collect::<Vec<_>>();
    if is_tallest(forest[x][y], up) {
        return true;
    }

    let down = &forest[x + 1..].iter().map(|v| v[y]).collect::<Vec<_>>();
    if is_tallest(forest[x][y], down) {
        return true;
    }

    false
}

fn is_tallest(target: u32, slice: &[u32]) -> bool {
    for i in slice {
        if *i >= target {
            //println!("{} >= {}", i, target);
            return false;
        }
    }

    true
}

fn parse(input: String) -> Vec<Vec<u32>> {
    let mut forest: Vec<Vec<u32>> = Vec::new();

    for (i, l) in input.lines().enumerate() {
        if i >= forest.len() {
            forest.push(vec![]);
        }
        for (_j, c) in l.chars().enumerate() {
            forest[i].push(c.to_digit(10).unwrap());
        }
    }

    forest
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

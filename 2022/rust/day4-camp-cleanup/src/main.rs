use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    let ranges = parse_ranges(&input);
    let answer1 = solve(&ranges, |l, r| {
        l.start <= r.start && l.end >= r.end || l.start >= r.start && l.end <= r.end
    });

    let answer2 = solve(&ranges, |l, r| {
        l.end >= r.start && l.start <= r.end || r.end >= l.start && r.start <= l.end
    });

    dbg!(answer1);
    dbg!(answer2);
}

fn solve(ranges: &Vec<(Range, Range)>, check: fn(&Range, &Range) -> bool) -> u32 {
    let bools = ranges
        .iter()
        .map(|(left, right)| check(left, right))
        .filter(|x| *x)
        .collect::<Vec<bool>>();

    bools.len() as u32
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn from(input: &str) -> Range {
        // 51-88
        let mut parts = input.split('-');
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        Range { start, end }
    }
}

fn parse_ranges(input: &str) -> Vec<(Range, Range)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let left = Range::from(parts.next().unwrap());
            let right = Range::from(parts.next().unwrap());
            (left, right)
        })
        .collect()
}

// fn part1() {
//     let input = read_stdin();
//     let ranges = parse_ranges(&input);

//     let answer = overlaps.len();
//     dbg!(answer);
// }

// fn part2() {
//     let input = read_stdin();
//     let overlaps = input
//         .lines()
//         .map(|line| line.split(','))
//         .filter_map(|r| r.collect_tuple())
//         .map(|(left, right)| overlaps(left, right))
//         .filter(|x| *x)
//         .collect::<Vec<bool>>();

//     //dbg!(&overlaps);
//     let answer = overlaps.len();
//     dbg!(answer);
// }

// fn contained(left: &str, right: &str) -> bool {
//     let left = left
//         .split('-')
//         .map(|i| i.parse().unwrap())
//         .take(2)
//         .collect::<Vec<u32>>();

//     let right = right
//         .split('-')
//         .map(|i| i.parse().unwrap())
//         .take(2)
//         .collect::<Vec<u32>>();

//     (left[0] <= right[0] && left[1] >= right[1]) || (right[0] <= left[0] && right[1] >= left[1])
// }

// fn overlaps(left: &str, right: &str) -> bool {
//     let left = left
//         .split('-')
//         .map(|i| i.parse().unwrap())
//         .take(2)
//         .collect::<Vec<u32>>();

//     let right = right
//         .split('-')
//         .map(|i| i.parse().unwrap())
//         .take(2)
//         .collect::<Vec<u32>>();

//     !(left[1] < right[0] || right[1] < left[0])
// }

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

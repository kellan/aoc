use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    let ranges = parse_ranges(&input);
    let answer1 = solve(&ranges, |l, r| {
        l.start <= r.start && l.end >= r.end || l.start >= r.start && l.end <= r.end
    });

    //    let answer2 = solve(&ranges, |l, r| !(l.end < r.start || r.end < l.start));

    // per https://nedbatchelder.com/blog/201310/range_overlap_in_two_compares.html
    // https://stackoverflow.com/questions/325933/determine-whether-two-date-ranges-overlap/325964#325964
    let answer2 = solve(&ranges, |l, r| l.end >= r.start && r.end >= l.start);

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

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

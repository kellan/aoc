use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    parse(input);
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u32,
    edges: Vec<String>,
}

fn parse(input: String) {
    let mut valves: HashMap<String, Valve> = HashMap::new();
    for line in input.lines() {
        if let Some(captures) = parse_input_line(line) {
            let edges: Vec<String> = captures[3]
                .split(',')
                .map(|s| String::from(s.trim()))
                .collect();
            let name = captures[1].to_string();
            let valve = Valve {
                name: name.to_string(),
                flow_rate: captures[2].parse().unwrap(),
                edges: edges,
            };
            valves.insert(name, valve);
        } else {
            panic!("Failed to parse: {}", line);
        }
    }
    dbg!(valves);
}

fn parse_input_line(text: &str) -> Option<regex::Captures> {
    lazy_static! {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        static ref LINE_REGEX: Regex = Regex::new(
            r"Valve ([A-Z]+) has flow rate=(\d+); tunnels? leads? to valves? (.+)"
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

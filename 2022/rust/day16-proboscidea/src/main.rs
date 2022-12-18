use core::time;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
//use std::fmt::{self, Display, Formatter};
use std::io::{self, Read};

type Path = Vec<String>;

fn main() {
    let input = read_stdin();
    let valves = parse(input);
    let mut valve_path: HashMap<String, Vec<Path>> = HashMap::new();
    for v in valves.keys() {
        let paths = paths(v.to_string(), &valves);
        valve_path.insert(v.to_string(), paths);
    }
    //dbg!(valve_path);
    best_move("AA".to_string(), 30, &valve_path, &valves);
}

fn best_move(
    start: String,
    time_remaining: u32,
    valve_path: &HashMap<String, Vec<Path>>,
    valves: &HashMap<String, Valve>,
) {
    let paths = valve_path.get(&start).unwrap();

    for p in paths {
        let dist = p.len() - 1;
        let valve = valves.get(p.last().unwrap()).unwrap();
        let score = (time_remaining - dist as u32) * valve.flow_rate;
        dbg!(p, score);
    }
}

fn paths(start: String, valves: &HashMap<String, Valve>) -> Vec<Path> {
    let aa = valves.get(&start).unwrap();
    let mut visited: HashSet<String> = HashSet::new();
    let mut path: Path = vec![];
    let paths = visit_valve(aa, path, &mut visited, &valves);
    paths
}

fn visit_valve(
    valve: &Valve,
    mut path: Path,
    visited: &mut HashSet<String>,
    valves: &HashMap<String, Valve>,
) -> Vec<Path> {
    visited.insert(valve.name.to_owned());
    path.push(valve.name.to_owned());

    let mut paths: Vec<Path> = vec![];
    for edge in valve.edges.iter() {
        if visited.contains(edge) {
            continue;
        }
        let child = valves.get(edge).unwrap();
        let child_paths = visit_valve(child, path.to_vec(), visited, valves);
        for p in child_paths.iter() {
            paths.push(p.to_vec());
        }
    }
    paths.push(path);
    paths
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u32,
    edges: Vec<String>,
}

fn parse(input: String) -> HashMap<String, Valve> {
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
    valves
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

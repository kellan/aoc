use lazy_static::lazy_static;
use regex::Regex;
use std::collections::{HashMap, HashSet};
//use std::fmt::{self, Display, Formatter};
use std::io::{self, Read};

type Path = Vec<String>;

fn main() {
    
    let input = read_stdin();
    let valves = parse(input);
    let dist = all_pairs_distances(&valves);

    let candidates: HashSet<String> = valves
        .iter()
        .filter(|(k, v)| v.flow_rate <= 0)
        .map(|(k, v)| k.to_string())
        .collect();

    //dbg!(candidates);

    let time_remaining = 30;
    find_best_path(
        "AA".to_string(),
        time_remaining,
        &candidates,
        &dist,
        &valves,
    );

    // let mut valve_path: HashMap<String, Vec<Path>> = HashMap::new();
    // for v in valves.keys() {
    //     let paths = paths(v.to_string(), &valves);
    //     valve_path.insert(v.to_string(), paths);
    // }
    //dbg!(valve_path);
    //best_move("AA".to_string(), 30, &valve_path, &valves);
}

fn find_best_path(
    current: String,
    time_remaining: usize,
    candidates: &HashSet<String>,
    distances: &HashMap<(String, String), usize>,
    valves: &HashMap<String, Valve>,
) -> (usize, HashSet<String>) {
    let mut max_flow = 0;
    let mut best_path: HashSet<String> = HashSet::new();

    dbg!(distances);
    for c in candidates.iter() {
        dbg!(&current, &c);
        let time = time_remaining
            - distances
                .get(&(current.to_string(), c.to_string()))
                .unwrap();
        if time > 0 {
            let mut flow = valves.get(c).unwrap().flow_rate * time;
            let (new_flow, mut bp) =
                find_best_path(c.to_string(), time, candidates, distances, valves);

            if flow > max_flow {
                max_flow = flow;
                bp.insert(current.to_string());
            }
        }
    }

    (max_flow, best_path)
}

fn all_pairs_distances(valves: &HashMap<String, Valve>) -> HashMap<(String, String), usize> {
    //dbg!(valves.len());
    let mut distances: HashMap<(String, String), usize> = HashMap::new();
    let mut seen: HashSet<(String, String)> = HashSet::new();

    for valve in valves.keys() {
        // skip the zero flow rates for the purposes of generating pairs
        // if valve != "AA" {
        //     if let Some(v) = valves.get(valve) {
        //         if v.flow_rate <= 0 {
        //             continue;
        //         }
        //     }
        // }

        let mut current: HashSet<String> = HashSet::new();
        let mut next: HashSet<String> = HashSet::new();
        let mut distance = 0;

        current.insert(valve.to_string());
        while !current.is_empty() {
            distance += 1;

            for v1 in current.iter() {
                for child in valves.get(v1).unwrap().edges.iter() {
                    if seen.contains(&(v1.to_string(), child.to_string())) {
                        continue;
                    }
                    next.insert(child.to_string());
                    distances.insert((v1.to_string(), child.to_string()), distance);
                    seen.insert((v1.to_string(), child.to_string()));
                }
            }
            current.clear();
            current.extend(next.drain());
        }
    }

    distances
}

// fn paths(start: String, valves: &HashMap<String, Valve>) -> Vec<Path> {
//     let aa = valves.get(&start).unwrap();
//     let mut visited: HashSet<String> = HashSet::new();
//     let mut path: Path = vec![];
//     let paths = visit_valve(aa, path, &mut visited, &valves);
//     paths
// }

// //
// // instead of the following do a BFS, every time we find a path,
// // check if it is shorter than the current path we have for those two nodes
// // but how does it shop?
// //
// //
// fn visit_valve(
//     valve: &Valve,
//     mut path: Path,
//     visited: &mut HashSet<String>,
//     valves: &HashMap<String, Valve>,
// ) -> Vec<Path> {
//     visited.insert(valve.name.to_owned());
//     path.push(valve.name.to_owned());

//     let mut paths: Vec<Path> = vec![];
//     for edge in valve.edges.iter() {
//         if visited.contains(edge) {
//             continue;
//         }
//         let child = valves.get(edge).unwrap();
//         let child_paths = visit_valve(child, path.to_vec(), visited, valves);
//         for p in child_paths.iter() {
//             paths.push(p.to_vec());
//         }
//     }
//     paths.push(path);
//     paths
// }

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: usize,
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

use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    println!("Hello, world!");
    let input = read_stdin();
    let mut troop = parse(input);

    //troop.get_mut(&0).unwrap().tick_part1();
    //part1(troop, 20);
    part2(troop, 10_000);
}

fn part1(mut troop: HashMap<u64, Monkey>, rounds: u64) {
    for _i in 0..rounds {
        for n in 0..troop.len() {
            //let distribute = troop.get_mut(&(n as u64)).unwrap().tick_part1();
            let distribute = troop.get_mut(&(n as u64)).unwrap().tick(Some(3), None);
            //dbg!(distribute);
            for dist in distribute {
                troop.get_mut(&(dist.0 as u64)).unwrap().items.push(dist.1)
            }
        }
    }

    let mut scores: Vec<u64> = troop.iter().map(|(k, v)| v.inspected).collect();
    scores.sort();
    scores.reverse();
    println!("{}", scores[0] * scores[1]);
}

fn part2(mut troop: HashMap<u64, Monkey>, rounds: u64) {
    let modulo: u64 = troop.iter().map(|(k, v)| v.divisor).product();

    for _i in 0..rounds {
        for n in 0..troop.len() {
            let distribute = troop.get_mut(&(n as u64)).unwrap().tick(None, Some(modulo));
            //dbg!(distribute);
            for dist in distribute {
                troop.get_mut(&(dist.0 as u64)).unwrap().items.push(dist.1)
            }
        }
    }

    let mut scores: Vec<u64> = troop.iter().map(|(k, v)| v.inspected).collect();
    scores.sort();
    scores.reverse();
    println!("{}", scores[0] * scores[1]);
}

#[derive(Debug)]
struct Monkey {
    id: u64,
    operation: Operation,
    items: Vec<u64>,
    divisor: u64,
    on_true: u64,
    on_false: u64,
    inspected: u64,
}

impl Monkey {
    fn new() -> Monkey {
        Monkey {
            id: 0,
            operation: Operation::Noop,
            items: vec![],
            divisor: 0,
            on_true: 0,
            on_false: 0,
            inspected: 0,
        }
    }

    fn tick(&mut self, bored: Option<u64>, modulo: Option<u64>) -> Vec<(u64, u64)> {
        let mut distribute: Vec<(u64, u64)> = Vec::new();
        while let Some(item) = self.items.pop() {
            let mut worried = self.operation.inspect(item);
            self.inspected += 1;

            if let Some(modulo) = modulo {
                worried = worried % modulo;
            }

            if let Some(bored) = bored {
                worried = worried / 3;
            }

            if worried % self.divisor == 0 {
                distribute.push((self.on_true, worried));
            } else {
                distribute.push((self.on_false, worried));
            }
        }

        distribute
    }
}

#[derive(Debug)]
enum Operation {
    Multiply(u64),
    Add(u64),
    Square,
    Noop,
}

impl Operation {
    fn new(parts: (&str, &str, &str)) -> Operation {
        match parts {
            ("old", "*", "old") => Operation::Square,
            ("old", "*", num) => Operation::Multiply(to_num(num)),
            ("old", "+", num) => Operation::Add(to_num(num)),
            _ => Operation::Noop,
        }
    }

    fn inspect(&self, old: u64) -> u64 {
        match self {
            Operation::Square => old * old,
            Operation::Add(num) => old + num,
            Operation::Multiply(num) => old * num,
            Operation::Noop => old,
        }
    }
}

fn parse(input: String) -> HashMap<u64, Monkey> {
    let parts = input.split("\n\n");
    let mut troop: HashMap<u64, Monkey> = HashMap::new();

    for part in parts {
        let mut monkey = Monkey::new();
        for line in part.lines() {
            let p = line.split_whitespace().collect::<Vec<&str>>();

            match (p[0], p[1]) {
                ("Monkey", id) => monkey.id = to_num(id),
                ("Starting", "items:") => {
                    let items: Vec<u64> = p[2..].iter().map(|s| to_num(s)).collect();
                    monkey.items = items;
                }
                ("Operation:", _) => monkey.operation = Operation::new((p[3], p[4], p[5])),
                ("Test:", "divisible") => monkey.divisor = p[3].parse().unwrap(),
                ("If", "true:") => monkey.on_true = p[5].parse().unwrap(),
                ("If", "false:") => monkey.on_false = p[5].parse().unwrap(),
                _ => (),
            }
        }
        //dbg!(monkey);
        troop.insert(monkey.id, monkey);
    }

    troop
}

fn to_num(input: &str) -> u64 {
    String::from(input)
        .chars()
        .filter(|c| c.is_digit(10))
        .collect::<String>()
        .parse()
        .unwrap()
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

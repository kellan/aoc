use std::io::{self, Read};

fn main() {
    println!("Hello, world!");
    let input = read_stdin();

    parse(input);
}

#[derive(Debug)]
struct Monkey {
    id: u32,
    operation: Operation,
    items: Vec<u32>,
    divisor: u32,
    on_true: u32,
    on_false: u32,
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
        }
    }
}

#[derive(Debug)]
enum Operation {
    Multiply(u32),
    Add(u32),
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
}

fn parse(input: String) {
    let parts = input.split("\n\n");

    for part in parts {
        let mut monkey = Monkey::new();
        for line in part.lines() {
            let p = line.split_whitespace().collect::<Vec<&str>>();

            match (p[0], p[1]) {
                ("Monkey", id) => monkey.id = to_num(id),
                ("Starting", "items:") => {
                    let items: Vec<u32> = p[2..].iter().map(|s| to_num(s)).collect();
                    monkey.items = items;
                }
                ("Operation:", _) => monkey.operation = Operation::new((p[3], p[4], p[5])),
                ("Test:", "divisible") => monkey.divisor = p[3].parse().unwrap(),
                ("If", "true:") => monkey.on_true = p[5].parse().unwrap(),
                ("If", "false:") => monkey.on_false = p[5].parse().unwrap(),
                _ => (),
            }
        }
        dbg!(monkey);
    }
}

fn to_num(input: &str) -> u32 {
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

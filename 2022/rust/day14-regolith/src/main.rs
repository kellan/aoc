use std::io::{self, Read};

fn main() {
    println!("Hello, world!");
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

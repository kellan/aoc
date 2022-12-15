use serde::Deserialize;
use serde_json::{Result, Value};
use std::io::{self, Read};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum Node {
    Int(u32),
    List(Vec<Node>),
}

fn main() {
    println!("Hello, world!");
    //let input = read_stdin();
    //let data = "[1,1,3,1,1]";
    let data = "[[1],[2,3,4]]";
    let v: Node = serde_json::from_str(data).unwrap();
    dbg!(v);
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

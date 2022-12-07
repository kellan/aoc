use regex::Regex;
use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    let root = parse_input(input);
}

fn parse_input(input: String) -> Folder {
    let mut root = Folder::new(String::from("/"));
    let mut cwd = &root;

    input.lines().for_each(|l| {
        if l == "$ cd .." {
            if let Some(parent) = &cwd.parent {
                cwd = &parent;
            }
        } else if l == "$ cd /" {
            cwd = &root;
        } else if l.starts_with("ls") {
            //
        } else if l.starts_with("dir") {
            let name = l.replace("dir ", "");
            if !cwd.folders.contains_key(&name) {
                let f = Folder::new(name);
                //cwd.folders.insert(f.name.to_owned(), f);
                cwd.name = f.name;
            }
        } else {
            // file
        }
    });

    root
}

#[derive(Debug)]
struct Folder {
    parent: Option<Box<Folder>>,
    name: String,
    size: u32,
    files: Vec<File>,
    folders: HashMap<String, Folder>,
}

impl Folder {
    fn new(name: String) -> Folder {
        Folder {
            name: name,
            parent: None,
            size: 0,
            files: vec![],
            folders: HashMap::new(),
        }
    }

    fn folder(&mut self, name: String) {
        if !self.folders.contains_key(&name) {
            let f = Folder::new(name);
            self.folders.insert(f.name.to_owned(), f);
        }
    }
}
#[derive(Debug)]
struct File {
    name: String,
    size: u32,
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

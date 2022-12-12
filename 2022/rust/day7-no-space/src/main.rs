use std::io::{self, Read};

fn main() {
    let input = read_stdin();
    let mut lines = input.lines();
    let mut root = Folder::new(String::from("/"));
    _ = lines.next(); // remove root cd /
    parse_folder(&mut root, &mut lines);
    let sizes = root.smaller_than();
    let sum: u32 = sizes.iter().map(|(_, s)| s).sum();
    dbg!(sum);

    let free = 70_000_000 - root.size();
    let diff = 30_000_000 - free;

    let sizes = root.larger_than(diff);
    let min = sizes.iter().map(|(_, s)| s).min();
    dbg!(min);
}

fn parse_folder(root: &mut Folder, lines: &mut std::str::Lines) {
    while let Some(l) = lines.next() {
        let parts = l.split_whitespace().collect::<Vec<&str>>();

        match (parts[0], parts[1]) {
            ("$", "cd") => {
                let name = parts[2];
                if name == ".." {
                    return;
                }
                let mut f = Folder::new(String::from(name));
                parse_folder(&mut f, lines);
                root.folders.push(f);
            }
            ("$", "ls") => {}
            ("dir", _name) => {}
            (size, name) => {
                root.files.push(File {
                    _name: String::from(name),
                    size: size.parse().unwrap(),
                });
            }
        }
    }
}

#[derive(Debug)]
struct Folder {
    name: String,
    files: Vec<File>,
    folders: Vec<Folder>,
}

impl Folder {
    fn new(name: String) -> Folder {
        Folder {
            name: name,
            files: vec![],
            folders: vec![],
        }
    }

    fn size(&self) -> u32 {
        // XXX: was going to memoize here, but that would require a mutable reference
        let file_size: u32 = self.files.iter().map(|f| f.size).sum();
        let folder_size: u32 = self.folders.iter().map(|f| f.size()).sum();
        file_size + folder_size
    }

    fn filter(&self, check: fn(u32) -> bool) -> Vec<(String, u32)> {
        let mut sizes: Vec<(String, u32)> = Vec::new();
        if check(self.size()) {
            sizes.push((self.name.to_owned(), self.size()));
        }

        for f in self.folders.iter() {
            sizes.extend(f.filter(check));
        }

        sizes
    }

    fn smaller_than(&self) -> Vec<(String, u32)> {
        fn check(size: u32) -> bool {
            return size <= 100_000;
        }

        self.filter(check)
    }

    fn larger_than(&self, target: u32) -> Vec<(String, u32)> {
        // XXX: can't make this work with filter(), can't figure out closures

        let mut sizes: Vec<(String, u32)> = Vec::new();
        if self.size() >= target {
            sizes.push((self.name.to_owned(), self.size()));
        }

        for f in self.folders.iter() {
            sizes.extend(f.larger_than(target));
        }

        sizes
    }

    // fn all_sub_folders(&self) -> Vec<Folder> {
    //     let sizes: Vec:<String,u32> = Vec::new();

    // }
}

#[derive(Debug)]
struct File {
    _name: String,
    size: u32,
}

fn read_stdin() -> String {
    let mut buffer = String::new();
    _ = io::stdin().read_to_string(&mut buffer);
    return buffer;
}

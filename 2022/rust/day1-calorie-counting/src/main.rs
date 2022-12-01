use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let mut max = 0;
    let mut elf = 0;

    for line in buffer.lines() {
        if !line.trim().is_empty() {
            let num = line.parse::<i32>().unwrap();
            elf += num;
        } else {
            if elf > max {
                max = elf;
            }
            elf = 0;
        }
    }
    println!("{}", max);

    Ok(())
}

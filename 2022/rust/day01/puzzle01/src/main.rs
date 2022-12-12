use std::env;
use std::fs;
use std::io;

fn main() {
    let file_path: &str;
    file_path = "../../input.txt";
    println!("In file {}", file_path);

    let contents =
        fs::read_to_string("../../input.txt").expect("Should have been able to read the file");

    let mut highest: u32 = 0;
    let mut current: u32 = 0;

    while contents.lines().next() != None {
        if contents.len() == 0 {
            if current > highest {
                highest = current;
            }
            current = 0;
        }
        current += contents.parse::<u32>().unwrap();
    }

    if current > highest {
        highest = current;
    }

    println!("Total {highest} calories that the elf is carrying\n");
}

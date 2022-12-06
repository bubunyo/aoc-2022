#![allow(dead_code)]

use std::collections::HashSet;
use std::io::BufRead;
use std::{fs, str};

fn main() {
    print!("Part-1::{}\n", sol("./input", 4));
    print!("Part-2::{}\n", sol("./input", 14));
}

fn sol(input: &str, l: usize) -> usize {
    let lines = read_lines(input);
    for line in lines {
        for i in 0..line.len() - l - 1 {
            let m: HashSet<char> =
                HashSet::from_iter(line.get(i..i + l).unwrap().chars().into_iter());
            if m.len() == l {
                return i + l;
            }
        }
    }
    0
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = fs::File::open(filename).expect(&format!("File {filename}.txt not found"));
    let reader = std::io::BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

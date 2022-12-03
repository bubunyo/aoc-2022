#![allow(dead_code)]

use std::collections::HashSet;
use std::io::BufRead;
use std::{fs, str};

fn main() {
    print!("Part-1::{}\n", part1("./input"));
    print!("Part-2::{}\n", part2("./input"));
}

fn part1(input: &str) -> u32 {
    let lines = read_lines(input);
    let mut acc: u32 = 0;

    let mut ls = HashSet::new();
    let mut rs = HashSet::new();

    for line in lines {
        if line.len() % 2 != 0 {
            panic!("line legth not equally distributed")
        }
        ls.clear();
        rs.clear();

        for i in 0..line.len() / 2 {
            let l = line.as_bytes()[i];
            let r = line.as_bytes()[line.len() - i - 1];

            if rs.contains(&l) {
                acc = acc + score(l) as u32;
                break;
            }
            ls.insert(l);
            if ls.contains(&r) {
                acc = acc + score(r) as u32;
                break;
            }
            rs.insert(r);
        }
    }

    acc
}

fn part2(input: &str) -> u32 {
    let lines = read_lines(input);
    let mut acc: u32 = 0;
    let mut pos = 1;

    let mut sets: Vec<HashSet<u8>> = Vec::new();

    for line in lines {
        sets.push(HashSet::from_iter(line.as_bytes().iter().map(|e| *e)));

        if pos % 3 == 0 {
            let sc = intersection(&mut sets);
            for j in sc {
                acc = acc + score(j) as u32
            }
            sets.clear()
        }
        pos = pos + 1
    }

    acc
}

fn score(c: u8) -> u8 {
    if c >= 'a' as u8 && c <= 'z' as u8 {
        c - ('a' as u8) + 1
    } else {
        c - 38
    }
}

fn intersection(sets: &mut Vec<HashSet<u8>>) -> HashSet<u8> {
    if sets.is_empty() {
        return HashSet::new();
    }

    if sets.len() == 1 {
        return sets.pop().unwrap();
    }

    let mut result = sets.pop().unwrap();
    result.retain(|item| sets.iter().all(|set| set.contains(item)));
    result
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = fs::File::open(filename).expect(&format!("File {filename}.txt not found"));
    let reader = std::io::BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

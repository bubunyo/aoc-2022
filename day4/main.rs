#![allow(dead_code)]

use std::io::BufRead;
use std::{fs, str};

fn main() {
    print!("Part-1::{}\n", part1("./input"));
    print!("Part-2::{}\n", part2("./input"));
}

fn part1(input: &str) -> u32 {
    let lines = read_lines(input);
    let mut acc: u32 = 0;

    for line in lines {
        let mut p = line.split(",").collect::<Vec<&str>>().into_iter();
        let mut l = p.next().unwrap().split("-");
        let lx = l.next().unwrap().parse::<u32>().unwrap();
        let ly = l.next().unwrap().parse::<u32>().unwrap();
        let mut l = p.next().unwrap().split("-");
        let rx = l.next().unwrap().parse::<u32>().unwrap();
        let ry = l.next().unwrap().parse::<u32>().unwrap();
        if (lx <= rx && ly >= ry) || (rx <= lx && ry >= ly) {
            acc = acc + 1
        }
    }
    acc
}

fn part2(input: &str) -> u32 {
    let lines = read_lines(input);
    let mut acc: u32 = 0;

    for line in lines {
        let mut p = line.split(",").collect::<Vec<&str>>().into_iter();
        let mut l = p.next().unwrap().split("-");
        let lx = l.next().unwrap().parse::<u32>().unwrap();
        let ly = l.next().unwrap().parse::<u32>().unwrap();
        let mut l = p.next().unwrap().split("-");
        let rx = l.next().unwrap().parse::<u32>().unwrap();
        let ry = l.next().unwrap().parse::<u32>().unwrap();
        let lb = if lx > rx { lx } else { rx };
        let rb = if ly > ry { ry } else { ly };
        if lb <= rb {
            acc = acc + 1
        }
    }
    acc
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = fs::File::open(filename).expect(&format!("File {filename}.txt not found"));
    let reader = std::io::BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

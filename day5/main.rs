#![allow(dead_code)]

use std::collections::VecDeque;
use std::io::BufRead;
use std::{fs, str};

fn main() {
    print!("Part-1::{}\n", part1("./input"));
    print!("Part-2::{}\n", part2("./input"));
}

fn part1(input: &str) -> String {
    let lines = read_lines(input);

    let mut p_done = false;

    let mut a: Vec<VecDeque<String>> = vec![];

    for line in lines {
        if line.trim().is_empty() {
            p_done = true;
            continue;
        }
        if !p_done {
            let e = line.split("").collect::<Vec<&str>>();
            if a.len() == 0 {
                let p = (e.len() - 1) / 4;
                a = vec![VecDeque::new(); p];
            }
            let mut j = 0;
            for i in (2..e.len()).step_by(4) {
                let o = e[i].to_owned();
                // print!("{i} -> {:?}", o);
                let t = o.trim();
                if !t.is_empty() {
                    if let Err(_) = t.parse::<usize>() {
                        a[j].push_back(o)
                    }
                }
                j = j + 1;
            }
            continue;
        }
        // moves
        let mvs = line.split(" ").collect::<Vec<&str>>();
        let count = mvs[1].trim().parse::<usize>().unwrap();
        let from = mvs[3].trim().parse::<usize>().unwrap() - 1;
        let to = mvs[5].trim().parse::<usize>().unwrap() - 1;
        for _ in 0..count {
            if let Some(e) = a[from].pop_front() {
                a[to].push_front(e)
            }
        }
    }

    a.iter()
        .map(|x| x.front().unwrap().to_string())
        .collect::<Vec<_>>()
        .concat::<_>()
}

fn part2(input: &str) -> String {
    let lines = read_lines(input);

    let mut p_done = false;

    let mut a: Vec<VecDeque<String>> = vec![];

    for line in lines {
        if line.trim().is_empty() {
            p_done = true;
            continue;
        }
        if !p_done {
            let e = line.split("").collect::<Vec<&str>>();
            if a.len() == 0 {
                let p = (e.len() - 1) / 4;
                a = vec![VecDeque::new(); p];
            }
            let mut j = 0;
            for i in (2..e.len()).step_by(4) {
                let o = e[i].to_owned();
                // print!("{i} -> {:?}", o);
                let t = o.trim();
                if !t.is_empty() {
                    if let Err(_) = t.parse::<usize>() {
                        a[j].push_back(o)
                    }
                }
                j = j + 1;
            }
            continue;
        }
        // moves
        let mvs = line.split(" ").collect::<Vec<&str>>();
        let count = mvs[1].trim().parse::<usize>().unwrap();
        let from = mvs[3].trim().parse::<usize>().unwrap() - 1;
        let to = mvs[5].trim().parse::<usize>().unwrap() - 1;
        let mut d: VecDeque<_> = a[from].drain(0..count).collect();
        d.append(&mut a[to]);
        a[to] = d;
    }

    a.iter()
        .map(|x| x.front().unwrap().to_string())
        .collect::<Vec<_>>()
        .concat::<_>()
}

fn read_lines(filename: &str) -> Vec<String> {
    let file = fs::File::open(filename).expect(&format!("File {filename}.txt not found"));
    let reader = std::io::BufReader::new(file);
    reader.lines().map(|l| l.unwrap()).collect()
}

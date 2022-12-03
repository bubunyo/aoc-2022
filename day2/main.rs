#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    print!("Part-1::{}\n", part1(String::from("./input")));
    print!("Part-2::{}\n", part2(String::from("./input")));
}

fn part1(input: String) -> String {
    let mut acc: i64 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(input) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(entry) = line {
                let mut play = entry.split_whitespace();
                let s = score(play_entry_p1(play.next()), play_entry_p1(play.next()));
                acc = acc + s as i64
            }
        }
    }

    return format!(" acc {}", acc);
}

fn part2(input: String) -> String {
    let mut acc: i64 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(input) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(entry) = line {
                let mut play = entry.split_whitespace();
                let s = score_p2(play_entry_p1(play.next()), play_entry_p2(play.next()));
                acc = acc + s as i64
            }
        }
    }

    return format!(" acc {}", acc);
}

#[derive(Debug, Copy, Clone)]
enum Score {
    Win = 6,
    Draw = 3,
    Loose = 0,
    Unknown,
}

#[derive(Debug, PartialEq)]
enum Play {
    Rock = 1,
    Papper = 2,
    Scissors = 3,
    Unknown = 0,
}

impl Play {
    fn win(&self) -> &Play {
        match self {
            Play::Scissors => &Play::Rock,
            Play::Rock => &Play::Papper,
            Play::Papper => &Play::Scissors,
            _ => &Play::Unknown,
        }
    }
    fn loose(&self) -> &Play {
        match self {
            Play::Scissors => &Play::Papper,
            Play::Rock => &Play::Scissors,
            Play::Papper => &Play::Rock,
            _ => &Play::Unknown,
        }
    }
    fn want_outcome(&self, s: &Score) -> &Play {
        match s {
            Score::Win => self.win(),
            Score::Loose => self.loose(),
            Score::Draw => &self,
            _ => &Play::Unknown,
        }
    }
}

fn score(l: &Play, r: &Play) -> u8 {
    let s: &Score = if *l == *r {
        &Score::Draw
    } else if r == l.win() {
        &Score::Win
    } else {
        &Score::Loose
    };

    return *r as u8 + *s as u8;
}

fn score_p2(l: &Play, o: &Score) -> u8 {
    let r: &Play = l.want_outcome(o);
    return *r as u8 + *o as u8;
}

fn play_entry_p1(e: Option<&str>) -> &Play {
    match e {
        Some("A") | Some("X") => &Play::Rock,
        Some("B") | Some("Y") => &Play::Papper,
        Some("C") | Some("Z") => &Play::Scissors,
        _ => &Play::Unknown,
    }
}

fn play_entry_p2(e: Option<&str>) -> &Score {
    match e {
        Some("X") => &Score::Loose,
        Some("Y") => &Score::Draw,
        Some("Z") => &Score::Win,
        _ => &Score::Unknown,
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

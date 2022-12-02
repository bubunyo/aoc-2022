use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    print!("Part-1::{}\n", part1(String::from("./input")));
    print!("Part-2::{}\n", part2(String::from("./input")));
}
fn part1(input: String) -> String {
    let mut acc = 0.0;
    let mut max = 0.0;
    let mut index = 1;
    let mut lead = 0;

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(input) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(entry) = line {
                if entry == "" {
                    if acc > max {
                        max = acc;
                        lead = index;
                    }

                    acc = 0.0;
                    index = index + 1;
                    continue;
                }
                let entry: f64 = entry.parse().unwrap();
                acc = acc + entry
            }
        }
    }

    return format!("lead {}, acc {}", lead, max);
}

fn part2(input: String) -> String {
    let mut acc = 0.0;

    let mut accs: Vec<f64> = Vec::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(input) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(entry) = line {
                if entry == "" {
                    accs.push(acc);
                    acc = 0.0;
                    continue;
                }
                let entry: f64 = entry.parse().unwrap();
                acc = acc + entry
            }
        }
    }
    accs.sort_by(|a, b| b.partial_cmp(a).unwrap());

    acc = 0.0;
    for i in 0..3 {
        acc = acc + accs[i]
    }

    return format!("acc {:?}", acc);
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

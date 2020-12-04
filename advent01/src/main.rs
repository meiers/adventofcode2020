// use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// From: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {

    let filename = "data/advent01.txt";
    let mut vec = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            // these commands may "panick"
            let string: String = line.unwrap();
            if string.trim().len() == 0 { continue; }
            let num: u32 = string.trim().parse::<u32>().unwrap();
            vec.push(num);
        }
    }

    // Solve part 1
    for a in 0..vec.len() {
        for b in 0..vec.len() {
            if a<=b { continue; }
            if vec[a] + vec[b] == 2020 {
                println!("{0} + {1} = {2} --> {0} * {1} = {3}", vec[a], vec[b], vec[a]+vec[b], vec[a]*vec[b]);
            }
        }
    }

    // Solve part 2
    for a in 0..vec.len() {
        for b in 0..vec.len() {
            for c in 0..vec.len() {
                if a<=b || b<=c { continue; }
                let sum: u32 = vec[a] + vec[b] + vec[c];
                if sum == 2020 {
                    let prod: u32 = vec[a] * vec[b] * vec[c];
                    println!("{0} + {1} + {2} = {3} --> {0} * {1} * {2} = {4}", vec[a], vec[b], vec[c], sum, prod);
                }
            }
        }
    }
}
// use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

// From: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {

    let filename = "data/advent02.txt";
    //let mut vec = Vec::new();

    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    if let Ok(lines) = read_lines(filename) {
        let (mut valid, mut invalid) = (0,0);
        for line in lines {
            let string: String = line.unwrap();
            if string.trim().len() == 0 { continue; }

            // complicated parsing to get the values out of each capture (aka group)
            let captures = re.captures(&string).unwrap();
            let min = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let max = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();
            let chr = captures.get(3).unwrap().as_str().chars().next().unwrap();
            let txt = captures.get(4).unwrap().as_str();

            let mut counter: u32 = 0;
            for c in txt.chars() {
                if c == chr {
                    counter+=1;
                }
            }
            if counter <=max && counter >= min {
                valid+=1;
                // println!("VALID   ({})\t{}", counter, string)
            } else {
                invalid+=1;
                // println!("INVALID ({})\t{}", counter, string)
            }
        }
        println!("PART I --- VALID: {}\tINVALID: {}\t (sum: {})", valid, invalid, valid+invalid)
    }



    if let Ok(lines) = read_lines(filename) {
        let (mut valid, mut invalid) = (0,0);
        for line in lines {
            let string: String = line.unwrap();
            if string.trim().len() == 0 { continue; }

            // complicated parsing to get the values out of each capture (aka group)
            let captures = re.captures(&string).unwrap();
            let p1 = captures.get(1).unwrap().as_str().parse::<usize>().unwrap() - 1;
            let p2 = captures.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1;
            let chr = captures.get(3).unwrap().as_str();
            let txt = captures.get(4).unwrap().as_str();

            if p2 < txt.len() {
                if chr == &txt[p1..p1+1] && chr != &txt[p2..p2+1] {
                    valid += 1;
                    // println!("VALID   {},{}\t{}", &txt[p1..p1+1], &txt[p2..p2+1], string);
                }
                else if chr != &txt[p1..p1+1] && chr == &txt[p2..p2+1] {
                    valid += 1;
                    // println!("VALID   {},{}\t{}", &txt[p1..p1+1], &txt[p2..p2+1], string);
                } else {
                    // println!("INVALID {},{}\t{}", &txt[p1..p1+1], &txt[p2..p2+1], string);
                    invalid += 1;
                }
            }
        }
        // Note that there are no cases of p2 >= txt.len()
        // so I don't need to handle this
        println!("PART II -- VALID: {}\tINVALID: {}\t (sum: {})", valid, invalid, valid+invalid)

    }
}
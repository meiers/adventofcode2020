// use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

// From: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn get_expected_fields() -> HashSet<String> {
    let mut expected_fields = HashSet::new();
    expected_fields.insert("byr".to_owned()); // (Birth Year)
    expected_fields.insert("iyr".to_owned()); // (Issue Year)
    expected_fields.insert("eyr".to_owned()); // (Expiration Year)
    expected_fields.insert("hgt".to_owned()); // (Height)
    expected_fields.insert("hcl".to_owned()); // (Hair Color)
    expected_fields.insert("ecl".to_owned()); // (Eye Color)
    expected_fields.insert("pid".to_owned()); // (Passport ID)
    // do not insert: expected_fields.insert("cid"); // (Country ID)
    expected_fields
}


fn get_fields(x: &str) -> HashSet<String> {
    let v: Vec<&str> = x.split(' ').collect();
    println!("Processing: {}", v.join(" | "));

    let mut found = HashSet::new();

    for chunk in x.split(' ') {
        assert_eq!(chunk[3..4], ":".to_string());
        let entry = chunk[0..3].to_owned();
        assert!(!found.contains(&entry));
        found.insert(entry);
    }
    found
}

fn are_fields_valid(expected: &HashSet<String>, found: &HashSet<String>) -> bool {
    for s in expected.symmetric_difference(found) {
        println!("   DIFF: {}", s);
        if s != "cid" {
            return false;
        }
    }
    true
}

fn main() {
    let filename = "data/advent04.txt";
    
    let expected = get_expected_fields();

    let (mut valid, mut invalid) = (0, 0);
    
    if let Ok(lines) = read_lines(filename) {
        let mut buffer: String = "".to_owned();
        for line in lines {
            let string: String = line.unwrap();
            if string.trim().len() == 0 {
                let found = get_fields(&buffer);
                if are_fields_valid(&expected, &found) {
                    println!("   VALID");
                    valid += 1;
                } else {
                    println!("   INVALID");
                    invalid += 1;
                }
                buffer = "".to_owned();
            } else {
                if buffer.len() > 0 { 
                    buffer.push_str(" ");
                }
                buffer.push_str(&string);
            }
        }
        if buffer.len() > 0 {
            let found = get_fields(&buffer);
            if are_fields_valid(&expected, &found) {
                println!("   VALID");
                valid += 1;
            } else {
                println!("   INVALID");
                invalid += 1;
            }
        }
    }
    println!("PART I: {} valid, {} invalid, sum = {}", valid, invalid, valid+invalid)
}

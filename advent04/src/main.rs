// use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::{HashSet, HashMap};
use regex::Regex;


// From: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

type Validator = fn(&String) -> bool;


fn get_expected_fields() -> HashMap<String, Validator> {
    let mut expected_fields = HashMap::new();
    //let re1 = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    fn validate_byr(s: &String) -> bool {
        let re_4d = Regex::new(r"^\d{4}$").unwrap();
        if !re_4d.is_match(s) {
            return false
        }
        let num = s.parse::<u32>().unwrap();
        num >= 1920 && num <= 2002
    }
    fn validate_iyr(s: &String) -> bool {
        let re_4d = Regex::new(r"^\d{4}$").unwrap();
        if !re_4d.is_match(s) {
            return false
        }
        let num = s.parse::<u32>().unwrap();
        num >= 2010 && num <= 2020
    }
    fn validate_eyr(s: &String) -> bool {
        let re_4d = Regex::new(r"^\d{4}$").unwrap();
        if !re_4d.is_match(s) {
            return false
        }
        let num = s.parse::<u32>().unwrap();
        num >= 2020 && num <= 2030
    }
    fn validate_hgt(s: &String) -> bool {
        let re_hgt = Regex::new(r"^(\d+)(in|cm)$").unwrap();
        if !re_hgt.is_match(s) {
            return false
        }
        let captures = re_hgt.captures(&s).unwrap();
        let num = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
        let unit = captures.get(2).unwrap().as_str();
        if unit == "in" {
            num >= 59 && num <= 76
        } else {
            num >= 150 && num <= 193
        }
    }
    fn validate_hcl(s: &String) -> bool {
        let re_hex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
        re_hex.is_match(s)
    }
    fn validate_ecl(s: &String) -> bool {
        let re_ecl = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
        re_ecl.is_match(s)
    }
    fn validate_pid(s: &String) -> bool {
        let re_pid = Regex::new(r"^\d{9}$").unwrap();
        re_pid.is_match(s)
    }


    expected_fields.insert("byr".to_owned(), validate_byr as Validator); // (Birth Year)
    expected_fields.insert("iyr".to_owned(), validate_iyr as Validator); // (Issue Year)
    expected_fields.insert("eyr".to_owned(), validate_eyr as Validator); // (Expiration Year)
    expected_fields.insert("hgt".to_owned(), validate_hgt as Validator); // (Height)
    expected_fields.insert("hcl".to_owned(), validate_hcl as Validator); // (Hair Color)
    expected_fields.insert("ecl".to_owned(), validate_ecl as Validator); // (Eye Color)
    expected_fields.insert("pid".to_owned(), validate_pid as Validator); // (Passport ID)
    // do not insert: expected_fields.insert("cid"); // (Country ID)
    expected_fields
}


fn get_fields(x: &str) -> HashMap<String, String> {
    let v: Vec<&str> = x.split(' ').collect();
    // println!("Processing: {}", v.join(" | "));

    let mut found = HashMap::new();

    for chunk in x.split(' ') {
        assert_eq!(chunk[3..4], ":".to_string());
        let key = chunk[0..3].to_owned();
        let value = chunk[4..].to_owned();
        found.insert(key, value);
    }
    found
}

fn are_fields_valid(expected: &HashMap<String, Validator>, found: &HashMap<String, String>) -> bool {
    let expected_keys: HashSet<String> = expected.keys().cloned().collect();
    let found_keys: HashSet<String> = found.keys().cloned().collect();
    for s in expected_keys.symmetric_difference(&found_keys) {
        // println!("   DIFF: {}", s);
        if s != "cid" {
            return false;
        }
    }

    // Next, check each field for validity
    for key in expected_keys {
        let validator = expected.get(&key).unwrap();
        let value = found.get(&key).unwrap();
        // println!("Value: {} -> {}", key, value);
        // println!("   {}", validator(value));
        if !validator(value) {
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
                    // println!("   VALID");
                    valid += 1;
                } else {
                    // println!("   INVALID");
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
                // println!("   VALID");
                valid += 1;
            } else {
                // println!("   INVALID");
                invalid += 1;
            }
        }
    }
    println!("PART I:  247 valid (code was in prev. commit)");
    println!("PART II: {} valid, {} invalid, sum = {}", valid, invalid, valid+invalid)
}

use std::fs::read_to_string;
use std::collections::HashSet;

fn count_chars(s: &str) -> usize {
    let mut set = HashSet::new();
    // It would be safer to replace everything [^a-z]
    for chr in s.replace("\n","").chars() {
        set.insert(chr);
    }
    set.len()
}

fn num_unique_chars_that_are_in_all_rows(s: &str) -> usize {
    let mut set: HashSet<char> = HashSet::new();
    let mut first_row = true;
    for row in s.split("\n") {
        if row.len() == 0 {
            continue;
        }
        if first_row {
            set = row.chars().collect();
            first_row = false
        } else {
            let other: HashSet<char> = row.chars().collect();
            set = set.intersection(&other).cloned().collect();
            if set.len() == 0 {
                break
            }
        }
    }
    set.len()
}

fn main() {
    let filename = "data/advent06.txt";
    let contents = read_to_string(filename).unwrap();
    let mut sum = 0;
    for block in contents.split("\n\n") {
        sum += count_chars(block);
    }
    println!("PART I:  sum = {}", sum);
    
    sum = 0;
    for block in contents.split("\n\n") {
        // println!("{}\ncount = {}\n---", block, num_unique_chars_that_are_in_all_rows(block));
        sum += num_unique_chars_that_are_in_all_rows(block);
    }
    println!("PART II: sum = {}", sum);
}

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

    let filename = "data/advent05.txt";

    let mut max_sid = 0;
    let mut vec = vec![0; 1024];

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            // these commands may "panick"
            let string: String = line.unwrap();
            if string.trim().len() == 0 { continue; }
            
            let row_s = string[0..7].to_owned().replace("B","1").replace("F","0");
            let col_s = string[7..10].to_owned().replace("R","1").replace("L","0");
            let row = isize::from_str_radix(&row_s, 2).unwrap();
            let col = isize::from_str_radix(&col_s, 2).unwrap();
            let sid = 8*row + col;
            vec[sid as usize] = 1;
            if sid > max_sid {
                max_sid = sid
            }
            // println!("{},{} -> row: {}, col: {} -> ID: {}", &string[0..7], &string[7..10], row, col, sid);
        }
    }
    println!("Part I: Max seat ID = {}", max_sid);
    
    println!("{}", vec.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(""));
    for i in 0..vec.len() {
        if i > 0 && i < vec.len() -1 && vec[i] == 0 && vec[i-1] == 1 && vec[i+1] == 1 {
            println!("Part II: Free seat ID = {}", i);
        }
    }

}
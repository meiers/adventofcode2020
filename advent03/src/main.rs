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

fn read_matrix() -> Vec<Vec<i8>> {

    let filename = "data/advent03.txt";
    let mut vec: Vec<Vec<i8>> = Vec::new();

    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            let string: String = line.unwrap();
            if string.trim().len() == 0 { continue; }

            let mut inner_vec = Vec::new();
            for c in string.chars() {
                if c == '#' {
                    inner_vec.push(1)
                } else {
                    inner_vec.push(0)
                }
            }
            vec.push(inner_vec);
        }
    }
    return vec;
}

fn count_trees(mat: &Vec<Vec<i8>>, right: usize, down: usize) -> usize {
    let w = mat[0].len();
    let h = mat.len();

    let (mut x, mut y): (usize, usize) = (0, 0);
    let mut trees = 0;
    while y < h {
        if mat[y][x % w]==1 {
            trees += 1;
            //println!("mat[{}][{}] is a tree!", y, x);
        }
        y+=down;
        x+=right;
    }
    trees // no "return" in Rust
}


fn main() {

    let mat = read_matrix();
    println!("PART I:   {} trees", count_trees(&mat, 3, 1));

    // part 2
    let a = count_trees(&mat, 1, 1);
    let b = count_trees(&mat, 3, 1);
    let c = count_trees(&mat, 5, 1);
    let d = count_trees(&mat, 7, 1);
    let e = count_trees(&mat, 1, 2);
    println!("{} {} {} {} {}", a, b, c, d, e);
    println!("PART II:  {}", a*b*c*d*e);
}
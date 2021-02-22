use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input").expect("open file error!");
    let rows: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|row| row.expect("read line error!"))
        .collect();

    // part 1
    // println!("{}", slope(&rows, 3, 1));

    // part 2
    println!(
        "{}",
        slope(&rows, 1, 1)
            * slope(&rows, 3, 1)
            * slope(&rows, 5, 1)
            * slope(&rows, 7, 1)
            * slope(&rows, 1, 2)
    );
}

fn slope(rows: &Vec<String>, right: usize, down: usize) -> usize {
    let mut count = 0;

    let mut idx = 0;
    for row_idx in (0..rows.len()).step_by(down) {
        let pos = rows[row_idx].get(idx..(idx + 1)).unwrap();

        if pos == "#" {
            count = count + 1;
        }

        idx = idx + right;
        if idx >= rows[row_idx].len() {
            idx = idx - rows[row_idx].len();
        }
    }

    count
}

use std::{collections::HashSet, fs::File, io::{self, BufRead}};

fn main() {
    let file = File::open("input").expect("Open file error!");

    let mut max = 0;
    let mut min = 127 * 8 + 7;
    let mut set = HashSet::new();
    io::BufReader::new(file)
        .lines()
        .map(|row| row.expect("Read line error!"))
        .for_each(|row| {
            let id = part1(&row);
            set.insert(id);
            max = max.max(id);
            min = min.min(id);
        });

    // part 1
    // println!("{}", max);

    // part 2
    for idx in min..=max {
        if !set.contains(&idx) {
            println!("{}", idx);
            break;
        }
    }
}

fn part1(row: &String) -> usize {
    let (_, row_idx) = &row[0..7].chars().fold((0, 127), |base, c| {
        if c == 'F' {
            (base.0, (base.0 + base.1) / 2)
        } else {
            ((base.0 + base.1) / 2, base.1)
        }
    });
    let (_, col_idx) = &row[7..].chars().fold((0, 7), |base, c| {
        if c == 'L' {
            (base.0, (base.0 + base.1) / 2)
        } else {
            ((base.0 + base.1) / 2, base.1)
        }
    });
    
    row_idx * 8 + col_idx
}

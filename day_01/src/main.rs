use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file: File = File::open("input").expect("Open file error");
    let num_set: HashSet<usize> = io::BufReader::new(file)
        .lines()
        .map(|row| row.expect("Read line error"))
        .map(|row| row.parse::<usize>().unwrap())
        .collect();

    // part 1
    match check(&num_set, 0) {
        Some((num_1, num_2)) => {
            println!("{} * {} = {}", num_1, num_2, num_1 * num_2)
        }
        None => {}
    }

    // part 2
    for num in num_set.iter() {
        match check(&num_set, num.clone()) {
            Some((num_1, num_2)) => {
                println!("{} * {} * {} = {}", num, num_1, num_2, num * num_1 * num_2);
                break;
            }
            None => {}
        }
    }
}

fn check(set: &HashSet<usize>, offset: usize) -> Option<(usize, usize)> {
    let target = 2020 - offset;
    for num in set.iter() {
        let tmp = target.checked_sub(num.clone());
        match tmp {
            Some(val) => {
                if set.contains(&val) {
                    return Some((num.clone(), val));
                }
            }
            None => {}
        }
    }
    None
}

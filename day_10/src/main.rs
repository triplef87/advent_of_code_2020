use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input").expect("Open file error");
    let mut data: Vec<usize> = io::BufReader::new(file)
        .lines()
        .map(|row| row.expect("Read line error").parse().unwrap())
        .collect();
    data.sort();
    // part 1
    // part1(&data);

    // part 2
    part2(&data);
}

fn part1(data: &Vec<usize>) {
    let mut count_1 = 0;
    let mut count_3 = 1;
    let mut prev = 0;

    data.iter().for_each(|num| {
        match num - prev {
            1 => count_1 = count_1 + 1,
            3 => count_3 = count_3 + 1,
            _ => {}
        }
        prev = *num;
    });

    println!("{}", count_1 * count_3);
}

fn part2(data: &Vec<usize>) {
    let set: HashSet<usize> = data.iter().map(|ele| *ele).collect();
    let max = data.iter().max().unwrap();

    let mut count_vec: Vec<usize> = vec![0; max + 1];
    count_vec[0] = 1;

    for idx in 0..count_vec.len() {
        if set.contains(&(idx + 1)) {
            count_vec[idx + 1] = count_vec[idx + 1] + count_vec[idx];
        }
        if set.contains(&(idx + 2)) {
            count_vec[idx + 2] = count_vec[idx + 2] + count_vec[idx];
        }
        if set.contains(&(idx + 3)) {
            count_vec[idx + 3] = count_vec[idx + 3] + count_vec[idx];
        }
    }

    println!("{}", count_vec[*max]);
}

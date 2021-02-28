use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input").expect("Open file error");
    let data: Vec<usize> = io::BufReader::new(file)
        .lines()
        .map(|row| row.expect("Read line error").parse::<usize>().unwrap())
        .collect();

    for idx in 25..data.len() {
        if part1(&data[(idx-25)..idx], data[idx]) {
            println!("part 1: {}", data[idx]);

            // part 2
            let (left, right) = part2(&data, data[idx]);
            let min = data[left..right].iter().min().unwrap();
            let max = data[left..right].iter().max().unwrap();
            println!("part 2: {}", min + max);
            break;
        }
    }
}

fn part1(data: &[usize], num: usize) -> bool {
    let set: HashSet<usize> = data.iter().map(|ele| *ele).collect();

    for &ele in data {
        if num > ele && set.contains(&(num - ele)) {
            return false;
        }
    }

    true
}

fn part2(data: &Vec<usize>, num: usize) -> (usize, usize) {
    let mut left = 0;
    let mut right = 1;

    let mut sum = data[left] + data[right];
    while sum != num {
        if sum < num {
            right = right + 1;
            sum = sum + data[right];
        } else {
            sum = sum - data[left];
            left = left + 1;
        }
    }

    (left, right + 1)
}

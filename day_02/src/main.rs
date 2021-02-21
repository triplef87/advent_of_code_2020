use std::{
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input").expect("Open file error!");

    let mut counter = 0;
    io::BufReader::new(file)
        .lines()
        .map(|row| row.expect("Read line error!"))
        .for_each(|row| {
            let splits: Vec<&str> = row.split(": ").collect();
            // part 1
            // if part1(splits[0], splits[1]) {
            //     counter = counter + 1;
            // }

            // part 2
            if part2(splits[0], splits[1]) {
                counter = counter + 1;
            }
        });
    println!("{}", counter);
}

// fn part1(rule_str: &str, input: &str) -> bool {
//     let rule_slice: Vec<&str> = rule_str.split(|c| c == '-' || c == ' ').collect();
//
//     let low_bound: usize = rule_slice[0].parse().unwrap();
//     let high_bound: usize = rule_slice[1].parse().unwrap();
//     let ch: char = rule_slice[2].parse().unwrap();
//
//     let count = input.chars().filter(|&c| c == ch).count();
//
//     count >= low_bound && count <= high_bound
// }

fn part2(rule_str: &str, input: &str) -> bool {
    let rule_slice: Vec<&str> = rule_str.split(|c| c == '-' || c == ' ').collect();

    let left_pos: usize = rule_slice[0].parse::<usize>().unwrap() - 1;
    let right_pos: usize = rule_slice[1].parse::<usize>().unwrap() - 1;
    let ch: char = rule_slice[2].parse().unwrap();

    let mut count = 0;
    input.chars().enumerate().for_each(|(idx, c)| {
        if c == ch && (idx == left_pos || idx == right_pos) {
            count = count + 1;
        }
    });

    count == 1
}

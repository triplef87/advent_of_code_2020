use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input").expect("Open file error");

    let mut map: HashMap<String, Vec<(usize, String)>> = HashMap::new();
    let mut rev_map: HashMap<String, Vec<String>> = HashMap::new();
    io::BufReader::new(file)
        .lines()
        .map(|row| row.expect("Read line error"))
        .for_each(|row| process_row(row, &mut map, &mut rev_map));

    println!("{}", part1(&rev_map));
    println!("{}", part2(&map));
}

fn process_row(
    row: String,
    map: &mut HashMap<String, Vec<(usize, String)>>,
    rev_map: &mut HashMap<String, Vec<String>>,
) {
    let split_row: Vec<&str> = row.split(" bags contain ").collect();

    let bag = split_row[0].to_string();
    let mut contain_bags = Vec::new();

    if split_row[1] != "no other bags." {
        let split_contains: Vec<&str> = split_row[1].split_whitespace().collect();

        let mut idx = 0;
        while idx < split_contains.len() {
            let num: usize = split_contains[idx].parse().unwrap();
            let contain_bag = format!("{} {}", split_contains[idx + 1], split_contains[idx + 2]);

            contain_bags.push((num, contain_bag.clone()));
            let entry = rev_map.entry(contain_bag).or_insert(Vec::new());
            entry.push(bag.clone());

            idx = idx + 4;
        }
    }

    map.insert(bag.clone(), contain_bags.clone());
}

fn part1(rev_map: &HashMap<String, Vec<String>>) -> usize {
    let mut stack = vec!["shiny gold".to_string()];
    let mut set: HashSet<String> = HashSet::new();

    while !stack.is_empty() {
        let tmp = stack.pop().unwrap();
        if let Some(next) = rev_map.get(&tmp) {
            next.iter().for_each(|ele| {
                if set.insert(ele.clone()) {
                    stack.push(ele.clone());
                }
            })
        }
    }

    set.len()
}

fn part2(map: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut counter = 0;
    let mut stack: Vec<(usize, String)> = vec![(1, "shiny gold".to_string())];

    while !stack.is_empty() {
        let (num, bag) = stack.pop().unwrap();

        if let Some(next) = map.get(&bag) {
            next.iter().for_each(|(next_num, next_bag)| {
                stack.push((num * next_num, next_bag.clone()));
            });
        }

        counter = counter + num;
    }

    counter - 1
}

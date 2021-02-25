use std::{collections::{HashMap, HashSet}, fs::File, io::{self, BufRead}};

fn main() {
    let file = File::open("input").expect("Open file error!");
    
    // part 1
    // let mut set = HashSet::new();
    let mut counter = 0;
    let mut row_count = 0;
    let mut map: HashMap<char, usize> = HashMap::new();
    io::BufReader::new(file)
        .lines()
        .map(|row| row.expect("Read line error!"))
        .for_each(|row| {
            // part1(row, &mut set, &mut counter);
            part2(row, &mut map, &mut counter, &mut row_count);
        });

    // part 1
    // println!("{}", counter + set.len());

    // part 2
    println!("{}", counter + map.iter().filter(|(_, &val)| val == row_count).count());
}

// fn part1(row: String, set: &mut HashSet<char>, counter: &mut usize) {
//     if row.is_empty() {
//         *counter = *counter + set.len();
//         set.clear();
//     } else {
//         row.chars().for_each(|c| {
//             set.insert(c);
//         });
//     }
// }

fn part2(row: String, map: &mut HashMap<char, usize>, counter: &mut usize, row_count: &mut usize) {
    if row.is_empty() {
        *counter = *counter + map.iter().filter(|(_, &val)| val == *row_count).count();
        *row_count = 0;
        map.clear();
    } else {
        row.chars().for_each(|c| {
            let entry = map.entry(c).or_insert(0);
            *entry = *entry + 1;
        });
        *row_count = *row_count + 1;
    }
}

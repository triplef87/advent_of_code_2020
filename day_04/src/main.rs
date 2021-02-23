use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
};

fn main() {
    let file = File::open("input").expect("Open file error!");

    let mut map = HashMap::new();
    let mut counter = 0;
    io::BufReader::new(file)
        .lines()
        .map(|row| row.expect("Read line error!"))
        .for_each(|row| {
            if row.is_empty() {
                if part2(&map) {
                    counter = counter + 1;
                }
                map.clear();
            } else {
                row.split_whitespace().for_each(|ele| {
                    let (key, val) = ele.split_at(3);
                    map.insert(key.to_string(), val.trim_start_matches(':').to_string());
                });
            }
        });

    println!("{}", counter);
}

// fn part1(map: &HashMap<String, String>) -> bool {
//     if map.len() == 8 {
//         return true;
//     }
//
//     if map.len() == 7 && !map.contains_key("cid") {
//         return true;
//     }
//     false
// }

fn part2(map: &HashMap<String, String>) -> bool {
    let mut counter = 0;
    for (key, val) in map.iter() {
        let mut check = false;
        match &key[..] {
            "byr" => {
                let num = val.parse::<usize>().unwrap();
                check = num >= 1920 && num <= 2002;
            }
            "iyr" => {
                let num = val.parse::<usize>().unwrap();
                check = num >= 2010 && num <= 2020;
            }
            "eyr" => {
                let num = val.parse::<usize>().unwrap();
                check = num >= 2020 && num <= 2030;
            }
            "hgt" => {
                if val.ends_with("cm") {
                    let num = val[0..val.len() - 2].parse::<usize>().unwrap();
                    check = num >= 150 && num <= 193;
                } else if val.ends_with("in") {
                    let num = val[0..val.len() - 2].parse::<usize>().unwrap();
                    check = num >= 59 && num <= 76;
                }
            }
            "hcl" => {
                if val.starts_with("#") {
                    let tmp = val.trim_matches('#');
                    check = tmp.len() == 6
                        && tmp.chars().fold(true, |base, c| {
                            base && (c.is_numeric() || (c >= 'a' && c <= 'f'))
                        });
                }
            }
            "ecl" => match &val[..] {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => check = true,
                _ => {}
            },
            "pid" => {
                check = val.len() == 9 && val.chars().fold(true, |base, c| base && c.is_numeric())
            }
            _ => {}
        }
        if check {
            counter = counter + 1;
        } else if key != "cid" {
            return false;
        }
    }

    counter == 7
}

use std::{collections::HashMap, fs::File, io::{self, BufRead}};

fn main() {
    let file = File::open("input").expect("Open file error");


    let mut map: HashMap<u64, u64> = HashMap::new();   
    let mut mask = String::new();
    io::BufReader::new(file).lines()
        .map(|row| row.expect("Read line error"))
        .for_each(|row| {
            if row.starts_with("mask") {
                mask = row[7..].to_string();
            } else {
                let splits: Vec<&str> = row.split(" = ").collect(); 
                let idx: u64 = splits[0][4..splits[0].len()-1].parse().unwrap();
                let val: u64 = splits[1].parse().unwrap();
                
                // part 1
                //let entry = map.entry(idx).or_insert(0);
                //*entry = part1(&mask, val);

                // part 2
                part2(&mask, idx).iter().for_each(|&addr| {
                    let entry = map.entry(addr).or_insert(0);
                    *entry = val;
                });
            }
        });

    let val = map.values().fold(0, |base, &inc| base + inc);
    println!("{}", val);
}

// part 1
//fn part1(mask: &str, val: u64) -> u64 {
//    let mut base = 0;
//    mask.chars().enumerate().for_each(|(idx, ch)| {
//        let mut bit = (val >> (35-idx)) & 1;
//
//        match ch {
//            '0' => bit = 0,
//            '1' => bit = 1,
//            _ => {}
//        }
//
//        base = base + (bit << (35-idx));
//    });
//    
//    return base;
//}

// part 2
fn part2(mask: &str, val: u64) -> Vec<u64> {
    let mut addrs: Vec<u64> = vec![0];

    mask.chars().enumerate().for_each(|(idx, ch)| {
        match ch {
            '1' => {
                addrs.iter_mut().for_each(|addr| {
                    *addr = *addr + (1 << (35-idx));
                });
            },
            'X' => {
                let mut tmp = Vec::new();

                addrs.iter().for_each(|addr| {
                    tmp.push(addr.clone());
                    tmp.push(addr + (1 << (35-idx)));
                });

                addrs = tmp;
            }
            _ => {
                addrs.iter_mut().for_each(|addr| *addr = *addr + (((val >> (35-idx)) & 1) << (35-idx)));
            }
        }
    });
    addrs 
}

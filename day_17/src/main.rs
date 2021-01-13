use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
};
// part 1
//
//#[derive(Debug, Hash, PartialEq, Eq)]
//struct Pos {
//    x: isize,
//    y: isize,
//    z: isize,
//}

// part 2
#[derive(Debug, Hash, PartialEq, Eq)]
struct Pos {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

fn main() {
    let file = File::open("input").expect("Open file error!");

    let mut light_set = HashSet::new();
    let mut row_num = 0;
    io::BufReader::new(file)
        .lines()
        .map(|row| row.expect("Read line error!"))
        .for_each(|row| {
            row.chars().enumerate().for_each(|(idx, c)| {
                match c {
                    '#' => {
                        // part 1
                        //light_set.insert(Pos {
                        //    x: row_num,
                        //    y: idx as isize,
                        //    z: 0,
                        //});
                        light_set.insert(Pos {
                            x: row_num,
                            y: idx as isize,
                            z: 0,
                            w: 0,
                        });
                    }
                    _ => {}
                };
            });
            row_num = row_num + 1;
        });

    for _ in 0..6 {
        let light_count = gen_light_count(&light_set);
        light_set = gen_new_map(light_set, light_count);
    }
    println!("{}", light_set.len());
}
// part 1
//fn gen_light_count(set: &HashSet<Pos>) -> HashMap<Pos, usize> {
//    let mut map = HashMap::new();
//
//    for ele in set.iter() {
//        let mut neighbers = Vec::new();
//        for x_diff in -1..2 {
//            for y_diff in -1..2 {
//                for z_diff in -1..2 {
//                    neighbers.push(Pos {
//                        x: ele.x + x_diff,
//                        y: ele.y + y_diff,
//                        z: ele.z + z_diff,
//                    });
//                }
//            }
//        }
//        neighbers.remove(13);
//
//        for neighber in neighbers {
//            let map_ele = map.entry(neighber).or_insert(0);
//            *map_ele = *map_ele + 1;
//        }
//    }
//    map
//}
//

// part 2
fn gen_light_count(set: &HashSet<Pos>) -> HashMap<Pos, usize> {
    let mut count = HashMap::new();

    for ele in set.iter() {
        let mut neighbers = Vec::new();

        for x_d in -1..2 {
            for y_d in -1..2 {
                for z_d in -1..2 {
                    for w_d in -1..2 {
                        neighbers.push(Pos {
                            x: ele.x + x_d,
                            y: ele.y + y_d,
                            z: ele.z + z_d,
                            w: ele.w + w_d,
                        });
                    }
                }
            }
        }
        neighbers.remove(40);

        for neighber in neighbers {
            let entry = count.entry(neighber).or_insert(0);
            *entry = *entry + 1;
        }
    }

    count
}

fn gen_new_map(set: HashSet<Pos>, count: HashMap<Pos, usize>) -> HashSet<Pos> {
    let mut new_set = HashSet::new();

    for (key, val) in count.into_iter() {
        if set.contains(&key) {
            match val {
                2 | 3 => {
                    new_set.insert(key);
                }
                _ => {}
            }
        } else {
            match val {
                3 => {
                    new_set.insert(key);
                }
                _ => {}
            }
        }
    }
    new_set
}

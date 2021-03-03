use std::{fs::File, io::{self, BufRead}, mem};

#[derive(Clone, Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

// part 1
// impl Dir {
//     fn rotate(&mut self, right: bool, offset: isize) {
//         let rotate_list = if right {
//             vec![Dir::North, Dir::East, Dir::South, Dir::West]
//         } else {
//             vec![Dir::North, Dir::West, Dir::South, Dir::East]
//         };
// 
//         let mut idx = match (self.clone(), right) {
//             (Dir::North, _) => 0,
//             (Dir::East, true) | (Dir::West, false) => 1,
//             (Dir::South, _) => 2,
//             (Dir::West, true) | (Dir::East, false) => 3,
//         };
// 
//         idx = idx + offset / 90;
//         if idx > 3 {
//             idx = idx - 4;
//         }
//         *self = rotate_list[idx as usize].clone();
//     }
// }

#[derive(Debug)]
struct Ship {
    x: isize,
    y: isize,
    dir: Dir,
}

impl Ship {
    // part 1
    // fn mov(&mut self, offset: isize, dir: Dir) {
    //     match dir {
    //         Dir::North => self.y = self.y + offset,
    //         Dir::East => self.x = self.x + offset,
    //         Dir::South => self.y = self.y - offset,
    //         Dir::West => self.x = self.x - offset,
    //     }
    // }

    // part 2
    fn forward(&mut self, times: isize, x_offset: isize, y_offset: isize) {
        self.x = self.x + x_offset * times;
        self.y = self.y + y_offset * times;
    }
}

// part 2
struct WayPoint {
    x: isize,
    y: isize
}

impl WayPoint {
    fn rotate(&mut self, right: bool, offset: isize) {
        match (offset, right) {
            (90, true) | (270, false) => {
                mem::swap(&mut self.x, &mut self.y);
                self.y = self.y * -1;
            }
            (180, _) => {
                self.x = self.x * -1;
                self.y = self.y * -1;
            },
            (270, true) | (90, false) => {
                mem::swap(&mut self.x, &mut self.y);
                self.x = self.x * -1;
            }
            _ => {}
        }
    }
}

fn main() {
    let file = File::open("input").expect("Open file error!");

    let mut ship = Ship {
        x: 0,
        y: 0,
        dir: Dir::East,
    };

    // part 2
    let mut waypoint = WayPoint {
        x: 10,
        y: 1
    };

    io::BufReader::new(file)
        .lines()
        .map(|row| row.expect("Read line error!"))
        .for_each(|row| part2(row, &mut ship, &mut waypoint));

    println!("{}", ship.x.abs() + ship.y.abs());
}

// part 1
// fn part1(row: String, ship: &mut Ship) {
//     let offset: isize = row[1..].parse().unwrap();
// 
//     match &row[0..1] {
//         "N" => ship.mov(offset, Dir::North),
//         "E" => ship.mov(offset, Dir::East),
//         "S" => ship.mov(offset, Dir::South),
//         "W" => ship.mov(offset, Dir::West),
//         "L" => ship.dir.rotate(false, offset),
//         "R" => ship.dir.rotate(true, offset),
//         "F" => ship.mov(offset, ship.dir.clone()),
//         _ => {}
//     }
// }

// part 2
fn part2(row: String, ship: &mut Ship, waypoint: &mut WayPoint) {
    let offset: isize = row[1..].parse().unwrap();

    match &row[0..1] {
        "N" => waypoint.y = waypoint.y + offset,
        "E" => waypoint.x = waypoint.x + offset,
        "S" => waypoint.y = waypoint.y - offset,
        "W" => waypoint.x = waypoint.x - offset,
        "L" => waypoint.rotate(false, offset),
        "R" => waypoint.rotate(true, offset),
        "F" => ship.forward(offset, waypoint.x, waypoint.y),
        _ => {}
    }
}

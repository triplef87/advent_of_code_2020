use std::{
    fs::File,
    io::{self, BufRead},
};

enum State {
    Floor,
    Empty,
    Occupied,
}

enum Dir {
    Neg,
    Pos,
    Empty,
}

impl Dir {
    fn mov(&self, val: usize, max_val: usize) -> Option<usize> {
        match self {
            Dir::Neg => val.checked_sub(1),
            Dir::Pos => {
                if val == max_val {
                    None
                } else {
                    Some(val + 1)
                }
            }
            Dir::Empty => Some(val),
        }
    }
}

struct Seat {
    state: State,
    count: usize,
}

impl Seat {
    fn update(&mut self) -> bool {
        let mut change = false;
        match self.state {
            State::Floor => self.count = 0,
            State::Empty => {
                if self.count == 0 {
                    self.state = State::Occupied;
                    change = true;
                }
                self.count = 0;
            }
            State::Occupied => {
                // part 1
                // if self.count >= 4 {

                // part 2
                if self.count >= 5 {
                    self.state = State::Empty;
                    change = true;
                }
                self.count = 0;
            }
        }
        change
    }
}

fn main() {
    let file = File::open("input").expect("Open file error!");
    let mut data: Vec<Vec<Seat>> = io::BufReader::new(file)
        .lines()
        .map(|row| row.expect("Read line error"))
        .map(|row| {
            let mut tmp: Vec<Seat> = Vec::new();
            row.chars().for_each(|c| match c {
                '#' => tmp.push(Seat {
                    state: State::Occupied,
                    count: 0,
                }),
                'L' => tmp.push(Seat {
                    state: State::Empty,
                    count: 0,
                }),
                '.' => tmp.push(Seat {
                    state: State::Floor,
                    count: 0,
                }),
                _ => {}
            });
            tmp
        })
        .collect();
    println!("{}", compute(&mut data));
}

fn compute(data: &mut Vec<Vec<Seat>>) -> usize {
    let mut change_count = 1;

    while change_count > 0 {
        compute_occupied(data);

        change_count = 0;
        data.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|seat| {
                if seat.update() {
                    change_count = change_count + 1;
                }
            });
        });
    }

    data.iter()
        .flatten()
        .filter(|seat| match seat.state {
            State::Occupied => true,
            _ => false,
        })
        .count()
}

fn compute_occupied(data: &mut Vec<Vec<Seat>>) {
    let max_row = data.len() - 1;
    let max_col = data[0].len() - 1;

    for i in 0..=max_row {
        for j in 0..=max_col {
            match data[i][j].state {
                State::Occupied => add_occupied(data, i, j, max_row, max_col),
                _ => {}
            }
        }
    }
}

// part 2
fn add_occupied(data: &mut Vec<Vec<Seat>>, i: usize, j: usize, max_i: usize, max_j: usize) {
    let dirs = vec![
        (Dir::Neg, Dir::Neg),
        (Dir::Empty, Dir::Neg),
        (Dir::Pos, Dir::Neg),
        (Dir::Neg, Dir::Empty),
        (Dir::Pos, Dir::Empty),
        (Dir::Neg, Dir::Pos),
        (Dir::Empty, Dir::Pos),
        (Dir::Pos, Dir::Pos),
    ];

    for dir in dirs {
        // part 1
        // if let Some(mov_i) = dir.0.mov(i, max_i) {
        //     if let Some(mov_j) = dir.1.mov(j, max_j) {
        //         data[mov_i][mov_j].count = data[mov_i][mov_j].count + 1;
        //     }
        // }

        // part 2
        let mut tmp_i = i;
        let mut tmp_j = j;

        let mut conti = true;
        while conti {
            match (dir.0.mov(tmp_i, max_i), dir.1.mov(tmp_j, max_j)) {
                (Some(mov_i), Some(mov_j)) => {
                    data[mov_i][mov_j].count = data[mov_i][mov_j].count + 1;
                    tmp_i = mov_i;
                    tmp_j = mov_j;
                    match data[mov_i][mov_j].state {
                        State::Empty | State::Occupied => break,
                        State::Floor => {}
                    }
                }
                _ => conti = false,
            }
        }
    }
}

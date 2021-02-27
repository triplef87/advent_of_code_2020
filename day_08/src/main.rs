use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
enum InsType {
    Nop,
    Acc,
    Jmp,
}

struct Ins {
    ins: InsType,
    offset: isize,
}

fn main() {
    let file = File::open("input").expect("Open file error");

    let mut instructions: Vec<Ins> = io::BufReader::new(file)
        .lines()
        .map(|row| {
            let tmp = row.expect("Read line error");
            let ins = match &tmp[..3] {
                "nop" => InsType::Nop,
                "acc" => InsType::Acc,
                _ => InsType::Jmp,
            };

            let offset: isize = if &tmp[4..5] == "+" {
                tmp[5..].parse::<isize>().unwrap()
            } else {
                tmp[5..].parse::<isize>().unwrap() * -1
            };

            Ins { ins, offset }
        })
        .collect();

    // part 1
    // let mut counter = 0;
    // part1(&instructions, &mut counter);
    // println!("{}", counter);

    // part 2
    let mut check = true;
    let mut counter = 0;
    let mut idx = 0;
    while check {
        counter = 0;
        match instructions[idx].ins {
            InsType::Nop => {
                instructions[idx].ins = InsType::Jmp;
                check = part1(&instructions, &mut counter);
                instructions[idx].ins = InsType::Nop;
            }
            InsType::Jmp => {
                instructions[idx].ins = InsType::Nop;
                check = part1(&instructions, &mut counter);
                instructions[idx].ins = InsType::Jmp;
            }
            InsType::Acc => {}
        }
        idx = idx + 1;
    }
    println!("{}", counter);
}

fn part1(instructions: &Vec<Ins>, counter: &mut isize) -> bool {
    let mut idx = 0;
    let mut set = vec![true; instructions.len()];

    while idx < instructions.len() && set[idx] {
        set[idx] = false;
        match instructions[idx].ins {
            InsType::Nop => idx = idx + 1,
            InsType::Jmp => {
                if instructions[idx].offset.is_positive() {
                    idx = idx + instructions[idx].offset.abs() as usize
                } else {
                    idx = idx - instructions[idx].offset.abs() as usize
                }
            }
            InsType::Acc => {
                *counter = *counter + instructions[idx].offset;
                idx = idx + 1;
            }
        }
    }

    idx < instructions.len()
}

use std::{
    fs::File,
    io::{self, BufRead},
};

enum Status {
    Empty,
    Add,
    Mul,
}

fn main() {
    let file = File::open("test").expect("Open file error!");

    let mut ans = 0;
    io::BufReader::new(file)
        .lines()
        .map(|row| row.expect("Read line error!"))
        .for_each(|row| {
            ans = ans + process(&row);
            println!("{}", process(&row));
        });

    println!("{}", ans);
}

fn process(row: &str) -> usize {
    let mut stack = Vec::new();
    let mut buffer = String::new();

    row.chars().for_each(|ch| match ch {
        '(' => {
            stack.push(buffer.clone());
            buffer.clear();
            stack.push("(".to_string());
        }
        ')' => {
            while stack.last().unwrap() != "(" {
                let mut tmp = stack.pop().unwrap();
                tmp.push_str(&buffer);
                buffer = tmp;
            }
            stack.pop();

            let new_num = compute(&buffer);
            buffer.clear();

            stack.push(new_num.to_string());
        }
        _ => buffer.push(ch),
    });
    stack.push(buffer);

    let total = stack.iter().fold(String::new(), |mut base, ele| {
        base.push_str(ele);
        base
    });

    compute(&total)
}

// part 1
// fn compute(row: &str) -> usize {
//     let mut num = 0;

//     let mut status = Status::Empty;
//     row.split_whitespace().for_each(|ele| match ele {
//         "+" => status = Status::Add,
//         "*" => status = Status::Mul,
//         _ => match status {
//             Status::Empty => num = ele.parse().unwrap(),
//             Status::Add => num = num + ele.parse::<usize>().unwrap(),
//             Status::Mul => num = num * ele.parse::<usize>().unwrap(),
//         },
//     });

//     num
// }

// part 2
fn compute(row: &str) -> usize {
    let mut num = 0;

    let mut stack = Vec::new();
    let mut status = Status::Empty;
    row.split_whitespace().for_each(|ele| match ele {
        "+" => status = Status::Add,
        "*" => status = Status::Mul,
        _ => match status {
            Status::Empty => num = ele.parse().unwrap(),
            Status::Add => num = num + ele.parse::<usize>().unwrap(),
            Status::Mul => {
                stack.push(num);
                num = ele.parse().unwrap();
            }
        },
    });

    if stack.is_empty() {
        num
    } else {
        num * stack.iter().fold(1, |base, acc| base * acc)
    }
}

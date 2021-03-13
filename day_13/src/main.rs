use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Read file error!");

    // part 1
    // println!("{}", part1(contents));

    // part 2
    println!("{}", part2(contents));
}

// part 1
// fn part1(input: String) -> usize {
//     let mut lines = input.lines();
// 
//     let timestamp = match lines.next() {
//         Some(val) => val.parse::<usize>().unwrap(),
//         None => panic!("Can't get timestamp"),
//     };
//     let bus_list: Vec<usize> = match lines.next() {
//         Some(row) => row
//             .split(',')
//             .filter(|ele| -> bool { *ele != "x" })
//             .map(|ele| -> usize { ele.parse::<usize>().unwrap() })
//             .collect(),
//         None => panic!("Can't get bus list"),
//     };
// 
//     let mut ans = (usize::MAX, 0);
//     for bus in bus_list {
//         let remind = timestamp % bus;
//         let wait = if remind == 0 { remind } else { bus - remind };
// 
//         if wait < ans.0 {
//             ans = (wait, bus);
//         }
//     }
// 
//     ans.0 * ans.1
// }

// part 2
fn part2(input: String) -> usize {
    let mut lines = input.lines();
    lines.next(); 

    let bus_list: Vec<(usize, usize)> = match lines.next() {
        None => panic!("Can't read bus list"),
        Some(row) => row
            .split(',')
            .enumerate()
            .map(|(idx, ele)| {
                if ele == "x" {
                    None
                } else {
                    let bus: usize = ele.parse().unwrap();
                    let remind = idx % bus;
                    if remind == 0 {
                        Some((bus, 0))
                    } else {
                        Some((bus, bus - remind))
                    }
                }
            })
            .filter(|ele| -> bool { ele.is_some() })
            .map(|ele| ele.unwrap())
            .collect(),
    };
    
    let total = bus_list.iter().fold(1, |base, &next| base * next.0);
    let ans = bus_list.iter().fold(0, |base, &next| {
        let step = total / next.0;
        let mut tmp = step;
        while tmp % next.0 != next.1 {
            tmp += step;            
        }
        base + tmp
    });

    ans % total
}

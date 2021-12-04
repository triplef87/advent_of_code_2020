fn main() {
    // Test
    // let mut cups: Vec<usize> = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];

    // Input
    let mut cups: Vec<usize> = vec![6, 5, 3, 4, 2, 7, 9, 1, 8];

    // part 1
    // let cups: Vec<usize> = solve_game(cups, 100);
    // print!("Cups:");
    // let mut tmp = 1;
    // for _ in 0..cups.len()-1 {
    //     print!(" {}", tmp);
    //     tmp = cups[tmp];
    // }
    // println!();

    // part 2
    cups.append(&mut (10..1000001).collect());
    let cups: Vec<usize> = solve_game(cups, 10000000);

    let mut tmp: usize = 1;
    let mut result: usize = 1; 
    for _ in 0..2 {
        println!("{}", cups[tmp]);
        tmp = cups[tmp];
        result = result * tmp;
    }
    println!("Result: {}", result);
}

fn solve_game(cups: Vec<usize>, move_time: usize) -> Vec<usize> {
    let len: usize = cups.len();
    let mut cups_link: Vec<usize> = vec![0; len+1];
    let last: usize = cups.last().unwrap().clone();

    let mut head: usize = 0;
    for cup in cups {
        cups_link[head] = cup;
        head = cup;
    }
    cups_link[last] = cups_link[0];

    head = cups_link[0];
    for _ in 0..move_time {
        let pick: usize = head;

        let mut reserve: Vec<usize> = Vec::new();
        for _ in 0..3 {
            reserve.push(cups_link[head]);
            head = cups_link[head];
        }

        let mut end: usize = if pick == 1 { len } else { pick - 1 };
        while reserve.contains(&end) {
            end = if end == 1 { len } else { end - 1 };
        }

        head = cups_link[head];
        cups_link[pick] = head;

        cups_link[reserve[2]] = cups_link[end];
        cups_link[end] = reserve[0];

        head = cups_link[pick];
    }

    cups_link
}
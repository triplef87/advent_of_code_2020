use std::{
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
struct Rule {
    arg: String,
    field1: (usize, usize),
    field2: (usize, usize),
}

impl Rule {
    fn check_valid(&self, num: usize) -> bool {
        (num >= self.field1.0 && num <= self.field1.1)
            || (num >= self.field2.0 && num <= self.field2.1)
    }
}

fn main() {
    let file = File::open("input").expect("Open file error");

    let mut rules = Vec::new();
    let mut my_ticket = Vec::new();
    let mut nearby_tickets = Vec::new();
    let mut state = 0;
    io::BufReader::new(file)
        .lines()
        .map(|row| row.expect("Read line error!"))
        .filter(|row| !row.is_empty())
        .for_each(|row| match state {
            0 => {
                if row.eq("your ticket:") {
                    state = 1;
                } else {
                    let splits: Vec<&str> = row.split(": ").collect();
                    let arg = splits[0].to_string();

                    let field_splits: Vec<&str> = splits[1].split(" or ").collect();
                    let field1: Vec<usize> = field_splits[0]
                        .split('-')
                        .map(|ele| ele.parse::<usize>().unwrap())
                        .collect();

                    let field2: Vec<usize> = field_splits[1]
                        .split('-')
                        .map(|ele| ele.parse::<usize>().unwrap())
                        .collect();

                    rules.push(Rule {
                        arg,
                        field1: (field1[0], field1[1]),
                        field2: (field2[0], field2[1]),
                    });
                }
            }
            1 => {
                if row.eq("nearby tickets:") {
                    state = 2;
                } else {
                    my_ticket = row
                        .split(',')
                        .map(|ele| ele.parse::<usize>().unwrap())
                        .collect();
                }
            }
            2 => {
                let tmp: Vec<usize> = row
                    .split(',')
                    .map(|ele| ele.parse::<usize>().unwrap())
                    .collect();
                nearby_tickets.push(tmp);
            }
            _ => {}
        });

    // part 1
    //let ans = nearby_tickets
    //    .iter()
    //    .flatten()
    //    .filter(|num| {
    //        let mut valid = true;
    //        for rule in &rules {
    //            valid = rule.check_valid(**num);
    //            if valid {
    //                break;
    //            }
    //        }
    //        !valid
    //    })
    //    .fold(0, |base, &acc| base + acc);
    //println!("{}", ans);

    // part 2
    let valid_tickets: Vec<Vec<usize>> = nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket.iter().fold(true, |mut valid, num| {
                if valid {
                    valid = rules
                        .iter()
                        .fold(false, |check, rule| check || rule.check_valid(*num))
                }
                valid
            })
        })
        .map(|ticket| ticket.clone())
        .collect();

    let mut ticket_rule: Vec<Vec<usize>> = Vec::new();
    for idx in 0..valid_tickets[0].len() {
        let mut stack = Vec::new();
        for (rule_idx, rule) in rules.iter().enumerate() {
            let mut check = true;
            for ticket in &valid_tickets {
                check = check && rule.check_valid(ticket[idx]);
                if !check {
                    break;
                }
            }
            if check {
                stack.push(rule_idx);
            }
        }
        ticket_rule.push(stack);
    }

    // match ticket col to rule
    let mut map: Vec<usize> = vec![0; ticket_rule.len()];
    loop {
        let mut num = ticket_rule.len();
        for idx in 0..ticket_rule.len() {
            if ticket_rule[idx].len() == 1 {
                let rule = ticket_rule.get_mut(idx).unwrap();
                num = rule.pop().unwrap();
                map[idx] = num;
                break;
            }
        }

        if num == ticket_rule.len() {
            break;
        }

        for ticket in &mut ticket_rule {
            ticket.retain(|&x| x != num);
        }
    }

    let mut depature_list: Vec<usize> = Vec::new();
    for (idx, &rule_num) in map.iter().enumerate() {
        if rules[rule_num].arg.starts_with("departure") {
            depature_list.push(idx);
        }
        println!("{} {}", idx, rules[rule_num].arg);
    }

    let ans = depature_list
        .iter()
        .fold(1, |base, &idx| base * my_ticket[idx]);
    println!("{}", ans);
}

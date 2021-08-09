use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
enum Rule {
    Ch(char),
    Seq(Vec<Vec<usize>>)
}

impl Rule {
    fn parse(row: &str) -> Rule {
        if row.starts_with("\"") {
            let ch = row.chars().nth(1).unwrap();
            return Rule::Ch(ch);
        }

        let seq: Vec<Vec<usize>> = row.split(" | ")
            .map(|ele| {
                ele.split_whitespace()
                    .map(|num| num.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        Rule::Seq(seq)
    }
}

#[derive(Debug)]
struct Node {
    input: Vec<char>,
    rules: Vec<Rule>
}

fn main() {
    let block: Vec<String> = fs::read_to_string("input")
        .expect("Read string from file error!")
        .split("\n\n")
        .map(|row| row.to_string())
        .collect();

    let mut rule_map: HashMap<usize, Rule> = block[0]
        .lines()
        .map(|row| {
            let pairs: Vec<&str> = row.split(": ").collect();
            (pairs[0].parse::<usize>().unwrap(), Rule::parse(pairs[1]))
        })
        .collect();

    // part 2
    let entry = rule_map.entry(8).or_insert(Rule::Ch('a'));
    *entry = Rule::Seq(vec![vec![42], vec![42, 8]]);

    let entry = rule_map.entry(11).or_insert(Rule::Ch('a'));
    *entry = Rule::Seq(vec![vec![42, 31], vec![42, 11, 31]]);

    let count = block[1].lines()
        .filter(|row| {
            let rule = rule_map.get(&0).unwrap();
            let input: Vec<char> = row.chars().collect();
            let stack: Vec<Node> = vec![ Node {
                input: input,
                rules: vec![rule.clone()],
            }];
            
            matches(stack, &rule_map)
        }).count();
    
    println!("{}", count);
}

fn matches(mut stack: Vec<Node>, rule_map: &HashMap<usize, Rule>) -> bool {
    while !stack.is_empty() {
        let mut node: Node = stack.pop().unwrap();
        if node.input.is_empty() || node.rules.is_empty() {
            // part 1
            // return node.input.is_empty() && node.rules.is_empty()

            // part 2
            if node.input.is_empty() && node.rules.is_empty() {
                return true;
            } else {
                continue;
            }
        }
        
        let rule: Rule = node.rules.remove(0);
        match rule {
            Rule::Ch(node_ch) => {
                let ch: char = node.input.remove(0);
                if ch == node_ch {             
                    stack.push(node);
                }
            },
            Rule::Seq(node_rules) => {
                node_rules.iter().for_each(|node_rule| {
                    let mut new_rules: Vec<Rule> = node_rule.iter().map(|num| {
                        rule_map.get(num).unwrap().clone()
                    }).collect();

                    let mut old_rules: Vec<Rule> = node.rules.clone();
                    new_rules.append(&mut old_rules);
                    stack.push(Node {
                        input: node.input.clone(),
                        rules: new_rules                        
                    });
                });
            },
        } 
    }

    return false
}


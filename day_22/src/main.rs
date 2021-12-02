use std::{collections::HashSet, fs};

fn main() {
    let mut decks: Vec<Vec<usize>> = fs::read_to_string("input").expect("Read input file error!")
        .split("\n\n")
        .map(|block| {
            let mut rows: Vec<&str> = block.split("\n").filter(|row| !row.is_empty()).collect();
            rows.remove(0);
            rows.iter().map(|row| row.parse::<usize>().unwrap()).collect()
        })
        .collect();

    //part_1(&mut decks);

    //let mut part_1_ans: usize = 0;
    //for deck in &decks {
    //    if deck.is_empty() { continue; }

    //    let len: usize = deck.len();
    //    part_1_ans = deck.iter().enumerate().fold(0, |base, (idx, ele)| base + ele * (len-idx));
    //}
    //println!("{}", part_1_ans);

    let mut deck_1: Vec<usize> = decks.remove(0);
    let mut deck_2: Vec<usize> = decks.remove(0);

    let won_deck: Vec<usize> = if part_2(&mut deck_1, &mut deck_2) {
        deck_1
    } else {
        deck_2
    };

    let len:usize = won_deck.len();
    println!("{}", won_deck.iter().enumerate().fold(0, |base, (idx, ele)| base + ele * (len-idx)));
}

fn part_1(decks: &mut Vec<Vec<usize>>) {
    while !(decks[0].is_empty() || decks[1].is_empty()) {
        let num_0: usize = decks[0].remove(0);
        let num_1: usize = decks[1].remove(0);

        if num_0 > num_1 {
            decks[0].push(num_0);
            decks[0].push(num_1);
        }

        if num_1 > num_0 {
            decks[1].push(num_1);
            decks[1].push(num_0);
        }
    }
}

fn part_2(deck_1: &mut Vec<usize>, deck_2: &mut Vec<usize>) -> bool {
    let mut deck_set: HashSet<(Vec<usize>, Vec<usize>)> = HashSet::new();
    while !(deck_1.is_empty() || deck_2.is_empty()) {
        if !deck_set.insert((deck_1.to_vec(), deck_2.to_vec())) { return true; }
        let num_0: usize = deck_1.remove(0);
        let num_1: usize = deck_2.remove(0);

        if num_0 <= deck_1.len() && num_1 <= deck_2.len() {
            let mut sub_deck_1: Vec<usize> = deck_1[..num_0].to_vec();
            let mut sub_deck_2: Vec<usize> = deck_2[..num_1].to_vec();

            if part_2(&mut sub_deck_1, &mut sub_deck_2) {
                deck_1.push(num_0);
                deck_1.push(num_1);
            } else {
                deck_2.push(num_1);
                deck_2.push(num_0);
            }

            continue;
        } 

        if num_0 > num_1 {
            deck_1.push(num_0);
            deck_1.push(num_1);
        }

        if num_1 > num_0 {
            deck_2.push(num_1);
            deck_2.push(num_0);
        }
    }

    deck_2.is_empty()
}

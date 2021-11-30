use std::{collections::{HashMap, HashSet}, fs};

fn main() {
    let mut i_map: HashMap<String, HashSet<usize>> = HashMap::new();
    let mut a_map: HashMap<String, HashSet<usize>> = HashMap::new();

    fs::read_to_string("input").expect("Read file error!")
        .split("\n")
        .filter(|row| !row.is_empty())
        .enumerate()
        .for_each(|(idx,row)| {
            let (i_list, mut a_list): (&str, &str) = row.split_once(" (contains ").unwrap();

            i_list.split(" ").for_each(|ingredient| {
                let entry = i_map.entry(ingredient.to_string()).or_insert(HashSet::new());
                entry.insert(idx);
            });
            
            a_list = &a_list[..a_list.len()-1];
            a_list.split(", ").for_each(|allergen| {
                let entry = a_map.entry(allergen.to_string()).or_insert(HashSet::new());
                entry.insert(idx);
            });
        });

    let l_to_a: HashMap<String, String> = find_match(&i_map, &a_map).expect("Can't find match");
    println!("Part1: {}", i_map.iter().fold(0, |base, (ingre, ingre_set)| {
        if l_to_a.contains_key(ingre) {
            base
        } else {
            base + ingre_set.len()
        }
    }));

    let mut part2_list: Vec<String> = l_to_a.keys().map(|ele| ele.to_string()).collect();
    part2_list.sort_by(|a,b| {
        let a_allergen = l_to_a.get(a).unwrap();
        let b_allergen = l_to_a.get(b).unwrap();
        a_allergen.cmp(b_allergen)
    });
    
    println!("Part 2: {}", part2_list.join(","));
}

fn gen_candidate(i_map: &HashMap<String, HashSet<usize>>, a_map: &HashMap<String, HashSet<usize>>)
    -> Vec<(String, Vec<String>)> {
    let mut candidate: Vec<(String, Vec<String>)> = Vec::new();
    for allergen in a_map.keys() {
        let mut ingre_list: Vec<String> = Vec::new();

        for ingredient in i_map.keys() {
            let mut check = true;
            let ingredient_set = i_map.get(ingredient).unwrap();
            for idx in a_map.get(allergen).unwrap() {
                if !ingredient_set.contains(idx) {
                    check = false;
                    break;
                }
            }

            if check { ingre_list.push(ingredient.to_string()); }
        }
        candidate.push((allergen.to_string(), ingre_list));
    }

    candidate.sort_by(|a, b| a.1.len().cmp(&b.1.len()));
    candidate
}

fn find_match(i_map: &HashMap<String, HashSet<usize>>, a_map: &HashMap<String, HashSet<usize>>)
    -> Option<HashMap<String, String>> {
    find_match_recursive(gen_candidate(i_map, a_map), HashMap::new())
}

fn find_match_recursive(mut candidates: Vec<(String, Vec<String>)>, l_to_a: HashMap<String, String>)
    -> Option<HashMap<String, String>> {
    if candidates.is_empty() {
        return Some(l_to_a);
    }

    let candidate: (String, Vec<String>) = candidates.remove(0);
    for ingre in candidate.1 {
        if !l_to_a.contains_key(&ingre) {
            let mut new_l_to_a: HashMap<String, String> = l_to_a.clone();
            new_l_to_a.insert(ingre.clone(), candidate.0.clone());
            
            let result = find_match_recursive(candidates.clone(), new_l_to_a);
            if result.is_some() { return result; }
        }
    }

    None
}

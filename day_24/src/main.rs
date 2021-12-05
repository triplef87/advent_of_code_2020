use std::{fs, collections::{HashMap, HashSet}};

fn main() {
    let orders: Vec<String> = read_file("input");
    let black_tiles: HashSet<(isize, isize)> = part_1(&orders);
    part_2(black_tiles);
}

fn read_file(file_name: &str) -> Vec<String> {
    fs::read_to_string(file_name).expect("Read file error!")
        .split("\n")
        .filter(|row| !row.is_empty())
        .map(|row| row.to_string())
        .collect()
}

fn part_1(orders: &Vec<String>) -> HashSet<(isize, isize)> {
    let mut tiles_map: HashMap<(isize,isize), bool> = HashMap::new();

    orders.iter().for_each(|order| {
        let mut direct: String = String::new();
        let mut idx: (isize, isize) = (0, 0);
        order.chars().for_each(|ch| {
            direct.push(ch);

            match direct.as_str() {
                "e" => {                   
                    idx.1 = idx.1 + 2;
                    direct.clear();
                },
                "se" => {
                    idx.0 = idx.0 + 1;
                    idx.1 = idx.1 + 1;
                    direct.clear();
                },
                "ne" => {
                    idx.0 = idx.0 - 1;
                    idx.1 = idx.1 + 1;
                    direct.clear();
                },
                "w" => {
                    idx.1 = idx.1 - 2;
                    direct.clear();
                },
                "sw" => {
                    idx.0 = idx.0 + 1;
                    idx.1 = idx.1 - 1;
                    direct.clear();
                },
                "nw" => {
                    idx.0 = idx.0 - 1;
                    idx.1 = idx.1 - 1;
                    direct.clear();
                },
                _ => {}
            }
        });

        let entry = tiles_map.entry(idx).or_insert(true);
        *entry = !*entry;
    });
    
    let mut black_tiles: HashSet<(isize, isize)> = HashSet::new();
    for (tile_idx, white) in tiles_map {
        if !white { black_tiles.insert(tile_idx); }
    }

    black_tiles
}

fn part_2(mut black_tiles: HashSet<(isize, isize)>) {
    for _ in 0..100 {
        let mut black_neighbors: HashMap<(isize, isize), usize> = HashMap::new();
        for tile in &black_tiles {
            let neighbors: Vec<(isize, isize)> = vec![
                (tile.0, tile.1 + 2), //e
                (tile.0 + 1, tile.1 + 1), //se
                (tile.0 - 1, tile.1 + 1), //ne
                (tile.0, tile.1 - 2), //w
                (tile.0 + 1, tile.1 - 1), //sw
                (tile.0 - 1, tile.1 - 1), //nw
            ];

            if !black_neighbors.contains_key(tile) { black_neighbors.insert(*tile, 0); }
            for neighbor in neighbors {
                let entry = black_neighbors.entry(neighbor).or_insert(0);
                *entry = *entry + 1;
            }
        }

        let black_tiles_tmp: HashSet<(isize, isize)> = black_tiles.clone();
        for (tile, black_count) in black_neighbors {
            if black_tiles_tmp.contains(&tile) {
                if black_count == 0 || black_count > 2 { black_tiles.remove(&tile); }
            } else {
                if black_count == 2 { black_tiles.insert(tile); }
            }
        }
    }

    println!("{}", black_tiles.len());
}
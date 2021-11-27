use std::{collections::HashMap, fs};

#[derive(Debug, Clone)]
struct Tile {
    id: usize,
    img: Vec<String>,
    edges: Vec<(String, String)>
}

impl Tile {
    fn parse(block: &str) -> Tile {
        let mut rows: Vec<String> = block.split("\n").filter(|row| !row.is_empty()).map(|row| row.to_string()).collect();
        let id: usize = rows.remove(0)[5..9].parse::<usize>().unwrap();
        Tile::new(id, rows)
    }

    fn new(id: usize, content: Vec<String>) -> Tile {
        let ori_edgs: Vec<String> = vec![
            content.first().unwrap().clone(), // Top
            content.last().unwrap().clone(), // Bot
            content.iter().map(|row| row.chars().nth(0).unwrap()).collect(), // Left
            content.iter().map(|row| row.chars().rev().nth(0).unwrap()).collect() // Right
        ];

        let edges: Vec<(String, String)> = ori_edgs.iter().map(|edge| (edge.clone(), edge.chars().rev().collect())).collect();

        Tile { id, img: content, edges }
    }

    fn count_neighbers(id: &usize, tile_map: &HashMap<usize, Tile>) -> usize {
        tile_map.get(id).unwrap().edges.iter()
            .filter(|edge| Tile::match_edges(id, &edge.0, tile_map).is_some())
            .count()
    }

    fn match_edges(id: &usize, edge: &String, tile_map: &HashMap<usize, Tile>) -> Option<(usize, usize, bool)> {
        for (other_id, other_tile) in tile_map {
            if *id == *other_id { continue; }
            for (edge_id, other_edges) in other_tile.edges.iter().enumerate() {
                if *edge == other_edges.0 { return Some((*other_id, edge_id, true)); }
                if *edge == other_edges.1 { return Some((*other_id, edge_id, false)); }
            }
        }

        None
    }

    fn rotate(self) -> Tile {
        let rows: Vec<String> = Tile::rotate_img(self.img);
        
        Tile::new(self.id, rows)
    }

    fn rotate_img(mut img: Vec<String>) -> Vec<String> {
        let mut rows: Vec<String>  = Vec::new();
        for ori_row in img.iter_mut().rev() {
            for idx in 0..ori_row.len() {
                let ch = ori_row.remove(0);
                match rows.get_mut(idx) {
                    Some(row) => row.push(ch),
                    None => rows.push(ch.to_string())
                }
            }
        }

        rows
    }

    fn v_flip(self) -> Tile {
        Tile::new(self.id, self.img.iter().rev().map(|row| row.to_string()).collect())
    }

    fn h_flip(self) -> Tile {
        Tile::new(self.id, self.img.iter().map(|row| row.chars().rev().collect()).collect())
    }
}

fn main() {
    let mut tile_map: HashMap<usize, Tile> = fs::read_to_string("input").expect("Read file error")
        .split("\n\n")
        .map(|block| {
            let tile = Tile::parse(block);
            (tile.id, tile)
        })
        .collect();

    let corners: Vec<usize> = tile_map.keys()
        .filter(|id| Tile::count_neighbers(id, &tile_map) == 2)
        .map(|id| *id)
        .collect();

    println!("{}", corners.iter().fold(1, |mul, id| mul * id));

    // Part 2
    // Select first corner as top left corner
    let mut top_left: Tile = tile_map.remove(corners.get(0).unwrap()).unwrap();
    let mut tmp: Tile = top_left.clone();

    // Rotate selected corner to correspond top-left
    while Tile::match_edges(&top_left.id, &top_left.edges[0].0, &tile_map).is_some() ||
        Tile::match_edges(&top_left.id, &top_left.edges[2].0, &tile_map).is_some() {
        top_left = tmp.rotate();
        tmp = top_left.clone();
    }
    tile_map.insert(top_left.id, top_left.clone());

    // Use top-left corner to fill the full grid
    let grid: Vec<Vec<usize>> = solve_grid(top_left.clone(), vec![top_left.id], &mut tile_map);
    let grid: Vec<String> = remove_gaps(grid, &tile_map);

    // Define monster
    let mut monster_count: usize = 0;
    let mut monster: Vec<String> = vec![
        "                  # ".to_string(),
        "#    ##    ##    ###".to_string(),
        " #  #  #  #  #  #   ".to_string()
    ];

    // Rotate monster image to find monster
    let mut rotate_count: usize = 0;
    while monster_count == 0 && rotate_count < 4 {
        monster_count = find_monster(&grid, &monster);
        monster = Tile::rotate_img(monster);
        rotate_count = rotate_count + 1
    }

    // If not found monster, flip the monster image
    if monster_count == 0 {
        monster = monster.iter().rev().map(|row| row.to_string()).collect();
    }

    // Rotate filped monster image to find monster
    rotate_count = 0;
    while monster_count == 0 && rotate_count < 4 {
        monster_count = find_monster(&grid, &monster);
        monster = Tile::rotate_img(monster);
        rotate_count = rotate_count + 1
    }

    // Get total # in grid
    let grid_num: usize = grid.iter().fold(0, |base, row| { 
        base + row.chars().fold(0, |base, ch| if ch == '#' { base+1 } else { base })
    });

    // Get total # in monster
    let monster_num: usize = monster.iter().fold(0, |base, row| { 
        base + row.chars().fold(0, |base, ch| if ch == '#' { base+1 } else { base })
    });

    // Final ans
    println!("{}", grid_num - monster_num * monster_count);
}

fn solve_grid(mut current: Tile, mut id_row: Vec<usize>, tile_map: &mut HashMap<usize, Tile> ) -> Vec<Vec<usize>> {
    let mut grid: Vec<Vec<usize>> = Vec::new();
    loop {
        match Tile::match_edges(&current.id, &current.edges[3].0, &tile_map) {
            Some((next_tile_id, edge_id, is_normal)) => {
                let mut next_tile: Tile = tile_map.remove(&next_tile_id).unwrap();
                match (edge_id, is_normal) {
                    (0, true) => {
                        next_tile = next_tile.v_flip();
                        next_tile = next_tile.rotate();
                    },
                    (1, true) => { next_tile = next_tile.rotate(); },
                    (2, true) => {},
                    (3, true) => { next_tile = next_tile.h_flip(); },
                    (0, false) => {
                        next_tile = next_tile.h_flip();
                        next_tile = next_tile.v_flip();
                        next_tile = next_tile.rotate();
                    },
                    (1, false) => {
                        next_tile = next_tile.rotate();
                        next_tile = next_tile.v_flip();
                    },
                    (2, false) => { next_tile = next_tile.v_flip(); },
                    (3, false) => {
                        next_tile = next_tile.v_flip();
                        next_tile = next_tile.h_flip();
                    },
                    (_, _) => unreachable!()
                }
                tile_map.insert(next_tile_id, next_tile.clone());
                id_row.push(next_tile_id);
                current = next_tile;
            },
            None => {
                let head: &Tile = tile_map.get(&id_row[0]).unwrap();
                let mut old_row = Vec::new();
                old_row.append(&mut id_row);
                grid.push(old_row);
                
                match Tile::match_edges(&head.id, &head.edges[1].0, &tile_map) {
                    Some((next_tile_id, edge_id, is_normal)) => {
                        let mut next_tile: Tile = tile_map.remove(&next_tile_id).unwrap();
                        match (edge_id, is_normal) {
                            (0, true) => {},
                            (1, true) => { next_tile = next_tile.v_flip();},
                            (2, true) => {
                                next_tile = next_tile.rotate();
                                next_tile = next_tile.h_flip();
                            },
                            (3, true) => {
                                next_tile = next_tile.h_flip();
                                next_tile = next_tile.v_flip();
                                next_tile = next_tile.rotate();
                            },
                            (0, false) => { next_tile = next_tile.h_flip(); },
                            (1, false) => {
                                next_tile = next_tile.h_flip();
                                next_tile = next_tile.v_flip();
                            },
                            (2, false) => { next_tile = next_tile.rotate(); },
                            (3, false) => {
                                next_tile = next_tile.h_flip();
                                next_tile = next_tile.rotate();
                            },
                            (_, _) => unreachable!()
                        }
                        tile_map.insert(next_tile_id, next_tile.clone());
                        id_row.push(next_tile_id);
                        current = next_tile;
                    },
                    None => { break; }
                }
            },
        }
    }

    grid
}

fn remove_gaps(grid: Vec<Vec<usize>>, tile_map: &HashMap<usize, Tile>) -> Vec<String> {
    let mut new_grid: Vec<String> = Vec::new();
    let tile_len: usize = tile_map.get(&grid[0][0]).unwrap().img.len() - 1;
    for row in grid {
        for idx in 1..tile_len {
            let mut new_row: String = String::new();
            for tile_id in &row {
                let mut tile_row: String = tile_map.get(tile_id).unwrap().img[idx].clone();
                tile_row.pop();
                tile_row.remove(0);
                new_row.push_str(&tile_row);
            } 
            new_grid.push(new_row);
        }
    }

    new_grid
}

fn find_monster(grid: &Vec<String>, monster: &Vec<String>) -> usize {
    let mut count = 0;

    let grid_width: usize = grid[0].len();
    let grid_height: usize = grid.len();

    let monster_width: usize = monster[0].len();
    let monster_height: usize = monster.len();

    for x in 0..(grid_height-monster_height) {
        for y in 0..(grid_width-monster_width) {
            let mut check = true;
            for monster_x in 0..monster_height {
                for monster_y in 0..monster_width {
                    if monster[monster_x].chars().nth(monster_y).unwrap() == '#' &&
                        grid[x+monster_x].chars().nth(y+monster_y).unwrap() != '#' {
                        check = false;
                        break;
                    }
                }
            }

            if check { count = count + 1; }
        }
    }
    count
}

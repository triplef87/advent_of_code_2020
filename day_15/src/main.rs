use std::collections::HashMap;

fn main() {
    let input = vec![12,1,16,3,11,0];
    let len = input.len() + 1;
    
    let mut map: HashMap<usize, usize> = HashMap::new();
    input.into_iter().enumerate().for_each(|(idx, val)| {
        map.insert(val, idx+1);
    });

    let mut val = 0;
    for idx in len..30000000 {
        if map.contains_key(&val) {
            if let Some(prev_idx) = map.get_mut(&val) {
                val = idx - *prev_idx;
                *prev_idx = idx;
            }
        } else {
            map.insert(val, idx);
            val = 0;
        }
    }
    println!("{}", val);
}

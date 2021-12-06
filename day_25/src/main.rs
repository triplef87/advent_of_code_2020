fn main() {
    // test
    // let card_pub: usize = 5764801;
    // let door_pub: usize = 17807724;

    // input
    let card_pub: usize = 13233401;
    let door_pub: usize = 6552760;
    
    let card_loop: usize = find_loop(card_pub);
    println!("{}", transform(door_pub, card_loop));
}

fn find_loop(pubkey: usize) -> usize {
    let mut val: usize = 1;
    let mut loop_size: usize = 1;
    loop {
        val = val * 7;
        val = val % 20201227;

        if val == pubkey { break; }
        loop_size = loop_size + 1;
    }

    loop_size
} 

fn transform(sub: usize, loop_size: usize) -> usize {
    let mut val: usize = 1;
    for _ in 0..loop_size {
        val = val * sub;
        val = val % 20201227;
    }

    val
}
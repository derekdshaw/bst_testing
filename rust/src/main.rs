use std::collections::HashSet;
use rand::Rng;
use bst::bst::BST;

fn make_large_data() -> (HashSet<i32>, i32) {
    let mut rng = rand::thread_rng();
    let mut data = HashSet::new();
    
    while data.len() < 1_000_000 {
        data.insert(rng.gen_range(1..2_000_000));
    }
    
    let target = rng.gen_range(1..2_000_000);
    (data, target)
}

// Have main do some work for profiling with external tools.
fn main() {
    let (data, _) = make_large_data();
    let mut bst = BST::new();

    for &item in &data {
        bst.insert(item);
    }

    println!("done");
}

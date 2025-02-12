use bst::bst::BST;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::collections::HashSet;
use rand::Rng;

fn make_large_data() -> HashSet<i32> {
    let mut rng = rand::thread_rng();
    let mut data = HashSet::new();
    
    while data.len() < 1_000_000 {
        data.insert(rng.gen_range(1..2_000_000));
    }
    
    data
}

fn build_large_tree_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("BST Operations");
    
    // Configure the measurement time (default is 5 seconds)
    group.measurement_time(std::time::Duration::from_secs(60));
    // Optionally reduce sample size since we're doing longer measurements
    group.sample_size(20);
    
    // Generate the data once before benchmarking
    let data = make_large_data();
    
    group.bench_function("build large tree", |b| {
        b.iter(|| {
            let mut tree = BST::new();
            for &item in &data {
                black_box(tree.insert(item));
            }
            black_box(tree)
        });
    });
    
    group.finish();
}

criterion_group!(benches, build_large_tree_benchmark);
criterion_main!(benches);

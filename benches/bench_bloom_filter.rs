use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId, Criterion,
    PlotConfiguration,
};
use std::hint::black_box;
use std::time::Duration;

use bloom_filter_benches::*;

/// num items and num bytes is ~8 hashes per item,
/// which is the hardcoded num hashes for sbbf.
const NUM_BYTES: usize = 1 << 16;
const NUM_BITS: usize = NUM_BYTES << 3;
const NUM_ITEMS: usize = 45_000;

fn run<T: Container<u64>>(group: &mut BenchmarkGroup<'_, WallTime>, num_items: usize, seed: u64) {
    let mut bloom: T = Container::new(NUM_BITS, num_items);
    bloom.extend(random_numbers(seed).take(num_items));
    let sample_vals: Vec<u64> = random_numbers(1234).take(1000).collect();
    let id = BenchmarkId::new(T::name(), num_items);
    group.bench_with_input(id, &num_items, |b, _| {
        b.iter(|| {
            for val in sample_vals.iter() {
                black_box(bloom.check(val));
            }
        })
    });
}

#[rustfmt::skip]
fn bench(c: &mut Criterion) {
    let sample_seed = 1234;
    for seed in [1234, 9876] {
        let item_type = if seed == sample_seed {
            "Member"
        } else {
            "Non-Member"
        };

        let mut group = c.benchmark_group(&format!(
            "{} Check Speed ({}Kb Allocated)",
            item_type,
            NUM_BYTES / 1000
        ));
        group.measurement_time(Duration::from_secs(10));
        group.plot_config(PlotConfiguration::default());

        run::<fastbloom::BloomFilter<rapidhash::fast::RandomState>>(&mut group, NUM_ITEMS, seed);
        //run::<fastbloom::BloomFilter<ahash::RandomState>>(&mut group, NUM_ITEMS, seed);
        // run_bench_for::<fastbloom::AtomicBloomFilter<rapidhash::fast::RandomState>>(&mut group, NUM_ITEMS, seed);
        run::<sbbf_rs_safe::Filter>(&mut group, NUM_ITEMS, seed);
        run::<bloom::BloomFilter>(&mut group, NUM_ITEMS, seed);
        run::<bloomfilter::Bloom<u64>>(&mut group, NUM_ITEMS, seed);
        run::<probabilistic_collections::bloom::BloomFilter<u64>>(&mut group, NUM_ITEMS, seed);
        run::<solana_bloom::bloom::Bloom<solana_program::hash::Hash>>(&mut group, NUM_ITEMS, seed);
        run::<datasketches::bloom::BloomFilter>(&mut group, NUM_ITEMS, seed);
        // run::<fastbloom_rs::BloomFilter>(&mut group, NUM_ITEMS, seed);

        group.finish();
    }
}
criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(5000);
    targets = bench
);
criterion_main!(benches);

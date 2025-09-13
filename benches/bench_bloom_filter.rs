use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId,
    Criterion, PlotConfiguration,
};

use bloomfilter::Bloom;
use fastbloom_rs;
use probabilistic_collections::bloom::BloomFilter as ProbBloomFilter;
use sbbf_rs_safe;
use std::hash::Hash;

use bloom_filter_benches::*;

const NUM_BYTES: usize = 1 << 16;

fn run_bench_for<T: Container<u64>>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    num_items: usize,
    seed: u64,
) {
    assert!(num_items >= 10);
    run_bench_for_input::<u64, T>(
        group,
        num_items,
        random_numbers(seed).take(num_items),
        random_numbers(1234).take(10),
    )
}

fn run_bench_for_input<X: Hash, T: Container<X>>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    num_items: usize,
    data: impl IntoIterator<Item = X>,
    bench_data: impl IntoIterator<Item = X>,
) {
    let num_bits = NUM_BYTES * 8;
    let mut bloom: T = Container::new(num_bits, num_items);
    bloom.extend(data.into_iter());
    println!("{:?}", bloom.num_hashes());
    let sample_vals = bench_data.into_iter().collect::<Vec<X>>();
    group.bench_with_input(
        BenchmarkId::new(T::name(), num_items),
        &num_items,
        |b, _| {
            b.iter(|| {
                for val in sample_vals.iter() {
                    black_box(bloom.check(val));
                }
            })
        },
    );
}

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
        group.plot_config(PlotConfiguration::default());
        for num_items in [45_000] {
            /*
            run_bench_for::<fastbloom::BloomFilter<ahash::RandomState>>(
                &mut group, num_items, seed,
            );
            run_bench_for::<fastbloom::AtomicBloomFilter<ahash::RandomState>>(
                &mut group, num_items, seed,
            );

            run_bench_for::<bloom::BloomFilter>(&mut group, num_items, seed);
            run_bench_for::<Bloom<u64>>(&mut group, num_items, seed);
            run_bench_for::<ProbBloomFilter<u64>>(&mut group, num_items, seed);
            run_bench_for::<sbbf_rs_safe::Filter>(&mut group, num_items, seed);
            */
            run_bench_for::<solana_bloom::bloom::Bloom<solana_program::hash::Hash>>(
                &mut group, num_items, seed,
            );
            // run_bench_for::<fastbloom_rs::BloomFilter>(&mut group, num_items, seed);
        }

        group.finish();
    }
}
criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench
);
criterion_main!(benches);

use criterion::{
    black_box, criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, BenchmarkId,
    Criterion, PlotConfiguration,
};

use ahash;
use bloom::ASMS;
use bloomfilter::Bloom;
use fastbloom::BloomFilter;
use fastbloom_rs;
use fastbloom_rs::Hashes;
use fastbloom_rs::Membership;
use probabilistic_collections::bloom::BloomFilter as ProbBloomFilter;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use sbbf_rs_safe;
use std::collections::HashSet;
use std::hash::BuildHasher;
use std::hash::Hash;
use std::iter::repeat;

use bloom_filter_benches::*;

fn run_bench_for<T: Container<String>>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    num_items: usize,
    seed: u64,
) {
    run_bench_for_input::<String, T>(
        group,
        num_items,
        random_strings(num_items, 6, 12, seed),
        random_strings(1000, 6, 12, 1234),
    )
}

fn run_bench_for_input<X: Hash, T: Container<X>>(
    group: &mut BenchmarkGroup<'_, WallTime>,
    num_items: usize,
    data: impl IntoIterator<IntoIter = impl ExactSizeIterator<Item = X>>,
    bench_data: impl IntoIterator<IntoIter = impl ExactSizeIterator<Item = X>>,
) {
    let num_bytes = 262144;
    let num_bits = num_bytes * 8;
    let bloom: T = Container::new(num_bits, data);
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
    let num_bytes = 262144;
    for seed in [1234, 9876] {
        let item_type = if seed == sample_seed {
            "Member"
        } else {
            "Non-Member"
        };
        /*
        let mut group = c.benchmark_group(&format!(
            "{} Check Speed vs Items ({}Kb Allocated, SipHash)",
            item_type,
            num_bytes / 1000
        ));
        group.plot_config(PlotConfiguration::default());
        for num_items in [
            2000, 3000, 5000, 7500, 10_000, 15_000, 20_000, 25_000, 50_000, 75_000, 100_000,
        ] {
            run_bench_for::<fastbloom::BloomFilter<512>>(&mut group, num_items, seed);
            run_bench_for::<bloom::BloomFilter>(&mut group, num_items, seed);
            run_bench_for::<Bloom<String>>(&mut group, num_items, seed);
            run_bench_for::<ProbBloomFilter<String>>(&mut group, num_items, seed);
        }
        group.finish();
        */
        let mut g2 = c.benchmark_group(&format!(
            "{} Check Speed vs Items ({}Kb Allocated)",
            item_type,
            num_bytes / 1000
        ));
        g2.plot_config(PlotConfiguration::default());
        for num_items in [
            2000, 3000, 5000, 6000, 7000, 8000, 9000, 10_000, 15_000, 20_000, 25_000, 50_000,
        ] {
            // run_bench_for::<fastbloom::BloomFilter<512, ahash::RandomState>>(&mut g2, num_items, seed, );
            // run_bench_for::<fastbloom::BloomFilter<64, ahash::RandomState>>(&mut g2, num_items, seed, );
            run_bench_for::<fastbloom::BloomFilter<512, XXHashWrapper>>(&mut g2, num_items, seed);
            //run_bench_for::<fastbloom::BloomFilter<64, XXHashWrapper>>(&mut g2, num_items, seed, );
            //run_bench_for::<sbbf_rs_safe::Filter>(&mut g2, num_items, seed);
            // run_bench_for::<fastbloom_rs::BloomFilter>(&mut g2, num_items, seed);
        }
        g2.finish();
        /*

        let mut g3 = c.benchmark_group(&format!(
            "{} Check Speed vs Items ({}Kb Allocated)",
            item_type,
            num_bytes / 1000
        ));
        g3.plot_config(PlotConfiguration::default());
        for num_items in [
            1000, 2000, 3000, 4000, 5000, 7500, 10_000, 12_500, 15_000, 20_000, 25_000, 50_000,
            75_000, 100_000,
        ] {
            run_bench_for::<fastbloom::BloomFilter<512, ahash::RandomState>>(
                &mut g3, num_items, seed,
            );
            run_bench_for::<fastbloom::BloomFilter<256, ahash::RandomState>>(
                &mut g3, num_items, seed,
            );
            run_bench_for::<fastbloom::BloomFilter<128, ahash::RandomState>>(
                &mut g3, num_items, seed,
            );
            run_bench_for::<fastbloom::BloomFilter<64, ahash::RandomState>>(
                &mut g3, num_items, seed,
            );
        }
        g3.finish();
        */
    }
}
criterion_group!(
    name = benches;
    config = Criterion::default();
    targets = bench
);
criterion_main!(benches);

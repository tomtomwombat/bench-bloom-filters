use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use std::hash::Hash;

mod container;
pub use container::{Container, XXHashWrapper};

const TRIALS: usize = 200_000_000;

pub fn list_fp<T: Container<u64>>() -> Vec<(usize, f64)> {
    let size_bytes = 1 << 16;
    let log_num_items = (3..=16).collect::<Vec<_>>();
    log_num_items
        .par_iter()
        .map(|x| {
            let num_items = 1 << *x;
            let prev_num_items = num_items / 2;
            let step = std::cmp::max(1, prev_num_items / 128);
            ((prev_num_items + step)..=num_items)
                .step_by(step)
                .map(|sub_items| {
                    let fp = false_pos_rate_for::<T>(sub_items, size_bytes);
                    (sub_items, fp)
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn false_pos_rate_for<T: Container<u64>>(num_items: usize, size_bytes: usize) -> f64 {
    let num_bits = size_bytes * 8;
    let filter = T::new(num_bits, random_numbers(num_items, 53824), num_items);

    let anti_vals = random_numbers(TRIALS / 100, 1234).map(|x| x + u32::MAX as u64);
    false_pos_rate(&filter, anti_vals)
}

fn false_pos_rate<X: Hash>(
    filter: &impl Container<X>,
    anti_vals: impl IntoIterator<Item = X>,
) -> f64 {
    let mut total = 0;
    let mut false_positives = 0;
    for x in anti_vals.into_iter() {
        total += 1;
        false_positives += filter.check(&x) as usize;
    }
    (false_positives as f64) / (total as f64)
}

pub fn random_numbers(num: usize, seed: u64) -> impl Iterator<Item = u64> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..=num).map(move |_| rng.gen::<u32>() as u64)
}

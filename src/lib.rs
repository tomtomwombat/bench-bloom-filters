use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use std::hash::Hash;

mod container;
pub use container::{Container, XXHashWrapper};

const TRIALS: usize = 2_000_000_000;
const STEPS: usize = 128;

pub fn list_fp<T: Container<u64>>() -> Vec<(usize, f64)> {
    let mag = 12;
    let size_bytes = 1 << mag;
    let num_bits = size_bytes * 8;
    let log_num_items = (1..=mag).collect::<Vec<_>>();
    log_num_items
        .par_iter()
        .map(|x| {
            let num_items = 1 << *x;
            let prev_num_items = num_items / 2;
            let step = std::cmp::max(1, prev_num_items / STEPS);
            let init_num_items = prev_num_items + step;

            let mut items = random_numbers(53824).into_iter();
            let mut filter = T::new(num_bits, init_num_items);
            filter.extend(take(&mut items, prev_num_items));

            (init_num_items..=num_items)
                .step_by(step)
                .map(|sub_items| {
                    let empty_filter = T::new(num_bits, sub_items);
                    if empty_filter.num_hashes() != filter.num_hashes() {
                        items = random_numbers(53824);
                        filter = empty_filter;
                        filter.extend(take(&mut items, sub_items));
                    } else {
                        filter.extend(take(&mut items, step));
                    }
                    /*
                    let mut items = random_numbers(53824).into_iter();
                    let mut filter = T::new(num_bits, sub_items);
                    filter.extend(take(&mut items, sub_items));
                    */
                    let fp = false_pos_rate_for::<T>(&filter);
                    (sub_items, fp)
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()
}

fn take<T: Iterator<Item = u64>>(
    iter: &mut T,
    num: usize,
) -> impl Iterator<Item = u64> + use<'_, T> {
    (0..=num).map(|_| iter.next().unwrap())
}

fn false_pos_rate_for<T: Container<u64>>(filter: &T) -> f64 {
    let anti_vals = random_numbers(1234)
        .take(TRIALS)
        .map(|x| x + u32::MAX as u64);
    false_pos_rate(filter, anti_vals)
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

pub fn random_numbers(seed: u64) -> impl Iterator<Item = u64> {
    let mut rng = StdRng::seed_from_u64(seed);
    (0..=usize::MAX).map(move |_| rng.gen::<u32>() as u64)
}

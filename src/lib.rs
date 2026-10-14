use rayon::prelude::*;
use std::hash::Hash;

mod random_filter;
pub use random_filter::RandomFilter;
mod container;
pub use container::{Container, XXHashWrapper};

/// Detects smaller false positives (depth)
const TRIALS_PER_BLOOM: usize = 30_000_000; // 1_600_000_000;
/// Controls min and max, smooths curve (less variance per x).
const NUM_THREADS: u64 = 8;
/// Controls how many x values there are
const GRANULARITY: f64 = 256.0;

pub(crate) fn false_pos_rate<X: Hash>(
    filter: &impl Container<X>,
    non_members: impl Iterator<Item = X>,
    stop_after: usize,
) -> f64 {
    let mut total = 0;
    let mut false_positives = 0;
    for x in non_members.into_iter() {
        total += 1;
        false_positives += filter.check(&x) as usize;
        if false_positives >= stop_after {
            break;
        }
    }
    (false_positives as f64) / (total as f64)
}

#[derive(Default)]
struct Ticks {
    cur: usize,
    step: f64,
}

impl Iterator for Ticks {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.cur += 1 << self.step as u32;
        self.step += 1.0 / GRANULARITY;
        Some(self.cur)
    }
}

//// Returns (num_bits, avg fp, min fp, max fp)
pub fn list_fp<T: Container<u64>>(num_bits: usize) -> impl Iterator<Item = (f64, f64, f64, f64)> {
    let data = (0..NUM_THREADS)
        .into_par_iter()
        .map(|_thread| {
            let mut res = Vec::new();
            let mut ticks = Ticks::default();
            let mut filter = T::new(num_bits, ticks.next().unwrap());

            for (prev_num_items, num_items) in std::iter::zip(Ticks::default(), ticks) {
                assert!(prev_num_items < num_items);
                let empty_filter = T::new(num_bits, num_items);
                if empty_filter.num_hashes() != filter.num_hashes() {
                    filter = empty_filter;
                    filter.extend(random_numbers(fastrand::u64(..)).take(num_items));
                } else {
                    let diff = num_items - prev_num_items;
                    filter.extend(random_numbers(fastrand::u64(..)).take(diff));
                }

                const REL_ERROR: f64 = 0.10; // 10% relative error
                const Z_95: f64 = 1.96; // 95% confidence
                let target_fps = (Z_95 / REL_ERROR).powi(2).ceil() as usize; // 385
                                                                             // Approximate FP rate for an optimally configured Bloom filter.
                let expected_fp = 0.6185_f64.powf(num_bits as f64 / num_items as f64);
                // Expected trials to observe enough false positives, capped.
                let max_trials =
                    ((target_fps as f64 / expected_fp).ceil() as usize).clamp(1, TRIALS_PER_BLOOM);

                let trials = random_numbers(fastrand::u64(..)).take(max_trials);
                let fp = false_pos_rate(&filter, trials, target_fps);
                let load = num_items as f64 / num_bits as f64;

                res.push((load, fp));

                if load >= 0.2 {
                    break;
                }
            }

            res
        })
        .collect::<Vec<_>>();

    let rows = min_len(&data);
    (0..rows).map(move |i| {
        let mut total = 0.0f64;
        let mut min = f64::MAX;
        let mut max = f64::MIN;

        for j in 0..NUM_THREADS as usize {
            let err = data[j][i].1;
            total += err;
            if err < min {
                min = err;
            }
            if err > max {
                max = err;
            }
        }

        (data[0][i].0, total / NUM_THREADS as f64, min, max)
    })
}

fn min_len<T>(vecs: &[Vec<T>]) -> usize {
    vecs.iter().min_by_key(|v| v.len()).unwrap().len()
}

pub fn random_numbers(seed: u64) -> impl Iterator<Item = u64> {
    let mut rng = fastrand::Rng::with_seed(seed);
    std::iter::from_fn(move || Some(rng.u64(..)))
}

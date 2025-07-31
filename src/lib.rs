use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;
use std::hash::Hash;

mod random_filter;
pub use random_filter::RandomFilter;
mod container;
pub use container::{Container, XXHashWrapper};

const TRIALS: usize = 3_00_000_000; //2_000_000_000;

fn take<T: Iterator<Item = u64>>(
    iter: &mut T,
    num: usize,
) -> impl Iterator<Item = u64> + use<'_, T> {
    (0..=num).map(|_| iter.next().unwrap())
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

fn false_pos_rate_adaptive<X: Hash>(
    filter: &impl Container<X>,
    non_members: impl IntoIterator<Item = X>,
) -> f64 {
    let mut total = 0;
    let mut false_positives = 0;
    for x in non_members.into_iter() {
        total += 1;
        false_positives += filter.check(&x) as usize;

        if false_positives >= 100 {
            break;
        }

        if false_positives >= 10 && total > 1_000_000 {
            break;
        }

        if false_positives >= 1 && total > 100_000_000 {
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
        self.step += 1.0 / 32.0;
        Some(self.cur)
    }
}

//// Returns (num_bits, avg fp, min fp, max fp)
pub fn list_fp2<T: Container<u64>>(num_bits: usize) -> impl Iterator<Item = (f64, f64, f64, f64)> {
    let num_trials: u64 = 8;
    let data = (0..num_trials)
        .into_par_iter()
        .map(|trial| {
            let mut res = Vec::new();

            let member_offset = trial.wrapping_mul((u64::MAX / 2) / num_trials);
            let non_member_offset = member_offset + (u64::MAX / 2);
            let mut members = member_offset..=u64::MAX / 2;
            let mut non_members = non_member_offset..=u64::MAX;

            let mut ticks = Ticks::default();
            let mut filter = T::new(num_bits, ticks.next().unwrap());

            for (prev_num_items, num_items) in std::iter::zip(Ticks::default(), ticks) {
                assert!(prev_num_items < num_items);
                let empty_filter = T::new(num_bits, num_items);
                if empty_filter.num_hashes() != filter.num_hashes() {
                    filter = empty_filter;
                    filter.extend(take(&mut members, num_items));
                } else {
                    filter.extend(take(&mut members, num_items - prev_num_items));
                }

                let fp = false_pos_rate_adaptive(&filter, take(&mut non_members, TRIALS));
                let load = num_items as f64 / num_bits as f64;

                res.push((load, fp));

                if load >= 0.1 {
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

        for j in 0..num_trials as usize {
            let err = data[j][i].1;
            total += err;
            if err < min {
                min = err;
            }
            if err > max {
                max = err;
            }
        }

        (data[0][i].0, total / num_trials as f64, min, max)
    })
}

fn min_len<T>(vecs: &[Vec<T>]) -> usize {
    vecs.iter().min_by_key(|v| v.len()).unwrap().len()
}

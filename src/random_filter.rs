use rand::Rng;
use std::collections::HashSet;

pub struct RandomFilter {
    data: HashSet<u64, ahash::RandomState>,
    pub num_hashes: usize,
    num_bits: usize,
}

impl RandomFilter {
    pub fn new(num_bits: usize, num_items: usize) -> Self {
        Self {
            data: HashSet::with_capacity_and_hasher(num_bits / 2, ahash::RandomState::new()),
            num_hashes: std::cmp::max(
                1,
                (f64::ln(2.0f64) * num_bits as f64 / num_items as f64).round() as usize,
            ),
            num_bits: num_bits,
        }
    }

    pub fn insert(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..self.num_hashes {
            let index: usize = rng.gen::<usize>() % self.num_bits;
            self.data.insert(index as u64);
        }
    }

    pub fn contains(&self) -> bool {
        let mut rng = rand::thread_rng();
        for _ in 0..self.num_hashes {
            let index: usize = rng.gen::<usize>() % self.num_bits;
            if !self.data.contains(&(index as u64)) {
                return false;
            }
        }
        true
    }
}

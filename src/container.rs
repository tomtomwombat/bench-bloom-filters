use bloom::ASMS;
use bloomfilter::Bloom;
use fastbloom::AtomicBloomFilter;
use fastbloom::BloomFilter;
use fastbloom_rs;
use fastbloom_rs::Hashes;
use fastbloom_rs::Membership;
use probabilistic_collections::bloom::BloomFilter as ProbBloomFilter;
use sbbf_rs_safe;

use std::hash::BuildHasher;
use std::hash::Hash;
use std::hash::Hasher;

/// A common trait for bloom filters to generalize testing functions
pub trait Container<X: Hash> {
    fn check(&self, s: &X) -> bool;
    fn num_hashes(&self) -> usize;
    fn new(num_bits: usize, num_items: usize) -> Self;
    fn extend<I: Iterator<Item = X>>(&mut self, items: I);
    fn name() -> &'static str;
}

macro_rules! impl_container_fastbloom {
    ($($hasher:ty = $name:literal),* $(,)*) => (
        $(
            impl<X: Hash> Container<X> for BloomFilter<$hasher> {
                #[inline]
                fn check(&self, s: &X) -> bool {
                    self.contains(s)
                }
                fn num_hashes(&self) -> usize {
                    self.num_hashes() as usize
                }
                fn new(
                    num_bits: usize,
                    num_items: usize,
                ) -> Self {
                    BloomFilter::with_num_bits(num_bits)
                        .hasher(<$hasher>::default())
                        .expected_items(num_items)
                }
                fn extend<I: Iterator<Item = X>>(&mut self, items: I) {
                    for x in items {
                        self.insert(&x);
                    }
                }
                fn name() -> &'static str {
                    $name
                }
            }
        )*
    )
}
impl_container_fastbloom!(
    fastbloom::DefaultHasher = "fastbloom",
    ahash::RandomState = "fastbloom",
);

impl<X: Hash> Container<X> for AtomicBloomFilter<ahash::RandomState> {
    #[inline]
    fn check(&self, s: &X) -> bool {
        self.contains(s)
    }
    fn num_hashes(&self) -> usize {
        self.num_hashes() as usize
    }
    fn new(num_bits: usize, num_items: usize) -> Self {
        AtomicBloomFilter::with_num_bits(num_bits)
            .hasher(ahash::RandomState::default())
            .expected_items(num_items)
    }
    fn extend<I: Iterator<Item = X>>(&mut self, items: I) {
        for x in items {
            self.insert(&x);
        }
    }
    fn name() -> &'static str {
        "fastbloom (Atomic)"
    }
}

impl<X: Hash> Container<X> for Bloom<X> {
    #[inline]
    fn check(&self, s: &X) -> bool {
        self.check(s)
    }
    fn num_hashes(&self) -> usize {
        self.number_of_hash_functions() as usize
    }
    fn new(num_bits: usize, num_items: usize) -> Self {
        Bloom::<X>::new(num_bits / 8, num_items)
    }
    fn extend<I: Iterator<Item = X>>(&mut self, items: I) {
        for x in items.into_iter() {
            self.set(&x);
        }
    }
    fn name() -> &'static str {
        "bloomfilter"
    }
}

impl Container<u64> for BloomFilter<XXHashWrapper> {
    #[inline]
    fn check(&self, s: &u64) -> bool {
        self.contains(&xxhash_rust::xxh3::xxh3_64(&s.to_be_bytes()))
    }
    fn num_hashes(&self) -> usize {
        self.num_hashes() as usize
    }
    fn new(num_bits: usize, num_items: usize) -> Self {
        let res = BloomFilter::with_num_bits(num_bits)
            .hasher(XXHashWrapper(0))
            .expected_items(num_items);
        res
    }
    fn extend<I: Iterator<Item = u64>>(&mut self, items: I) {
        for x in items {
            self.insert(&xxhash_rust::xxh3::xxh3_64(&x.to_be_bytes()));
        }
    }
    fn name() -> &'static str {
        "fastbloom - xxhash"
    }
}

impl BuildHasher for XXHashWrapper {
    type Hasher = XXHashWrapper;
    #[inline]
    fn build_hasher(&self) -> Self::Hasher {
        XXHashWrapper(0)
    }
}

pub struct XXHashWrapper(u64);
impl Hasher for XXHashWrapper {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
    #[inline]
    fn write(&mut self, _: &[u8]) {
        unreachable!()
    }

    #[inline]
    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
}

impl Container<u64> for sbbf_rs_safe::Filter {
    #[inline]
    fn check(&self, s: &u64) -> bool {
        self.contains_hash(xxhash_rust::xxh3::xxh3_64(&s.to_be_bytes()))
    }
    fn num_hashes(&self) -> usize {
        8
    }
    fn new(num_bits: usize, _num_items: usize) -> Self {
        sbbf_rs_safe::Filter::new(num_bits, 1)
    }
    fn extend<I: Iterator<Item = u64>>(&mut self, items: I) {
        for x in items {
            self.insert_hash(xxhash_rust::xxh3::xxh3_64(&x.to_be_bytes()));
        }
    }
    fn name() -> &'static str {
        "sbbf"
    }
}

impl Container<u64> for fastbloom_rs::BloomFilter {
    #[inline]
    fn check(&self, s: &u64) -> bool {
        self.contains(&s.to_be_bytes())
    }
    fn num_hashes(&self) -> usize {
        self.hashes() as usize
    }
    fn new(num_bits: usize, num_items: usize) -> Self {
        let hashes = bloom::bloom::optimal_num_hashes(num_bits, num_items as u32);
        fastbloom_rs::FilterBuilder::from_size_and_hashes(num_bits as u64, hashes)
            .build_bloom_filter()
    }
    fn extend<I: Iterator<Item = u64>>(&mut self, items: I) {
        for x in items {
            self.add(&x.to_be_bytes());
        }
    }
    fn name() -> &'static str {
        "fastbloom-rs"
    }
}

impl<X: Hash> Container<X> for bloom::BloomFilter {
    #[inline]
    fn check(&self, s: &X) -> bool {
        self.contains(s)
    }
    fn num_hashes(&self) -> usize {
        self.num_hashes() as usize
    }
    fn new(num_bits: usize, num_items: usize) -> Self {
        let hashes = bloom::bloom::optimal_num_hashes(num_bits, num_items as u32);
        bloom::BloomFilter::with_size(num_bits, hashes)
    }
    fn extend<I: Iterator<Item = X>>(&mut self, items: I) {
        for x in items {
            self.insert(&x);
        }
    }
    fn name() -> &'static str {
        "bloom"
    }
}

impl<X: Hash> Container<X> for ProbBloomFilter<X> {
    #[inline]
    fn check(&self, s: &X) -> bool {
        self.contains(s)
    }
    fn num_hashes(&self) -> usize {
        self.hasher_count() as usize
    }
    fn new(num_bits: usize, num_items: usize) -> Self {
        ProbBloomFilter::<X>::from_item_count(num_bits, num_items)
    }
    fn extend<I: Iterator<Item = X>>(&mut self, items: I) {
        for x in items {
            self.insert(&x);
        }
    }
    fn name() -> &'static str {
        "probabilistic-collections"
    }
}

impl Container<u64> for crate::RandomFilter {
    #[inline]
    fn check(&self, s: &u64) -> bool {
        self.contains()
    }
    fn num_hashes(&self) -> usize {
        self.num_hashes
    }
    fn new(num_bits: usize, num_items: usize) -> Self {
        crate::RandomFilter::new(num_bits, num_items)
    }
    fn extend<I: Iterator<Item = u64>>(&mut self, items: I) {
        for x in items {
            self.insert();
        }
    }
    fn name() -> &'static str {
        "Theoretical Best"
    }
}

use rand::Rng;
impl Container<u64> for solana_bloom::bloom::Bloom<solana_program::hash::Hash> {
    #[inline]
    fn check(&self, x: &u64) -> bool {
        let mut b = [0u8; 32];
        b[0..8].copy_from_slice(&x.to_ne_bytes());
        //b[8..16].copy_from_slice(&x.to_ne_bytes());
        //b[16..24].copy_from_slice(&x.to_ne_bytes());
        //b[24..32].copy_from_slice(&x.to_ne_bytes());

        self.contains(&solana_program::hash::Hash::new_from_array(b))
    }
    fn num_hashes(&self) -> usize {
        self.keys.len()
    }
    fn new(num_bits: usize, num_items: usize) -> Self {
        let n = num_items as f64;
        let m = num_bits as f64;
        // infinity as usize is zero in rust 1.43 but 2^64-1 in rust 1.45; ensure it's zero here
        let num_keys = if n == 0.0 {
            0.0
        } else {
            1f64.max(((m / n) * 2f64.ln()).round())
        } as usize;
        let keys: Vec<u64> = (0..num_keys).map(|_| rand::thread_rng().gen()).collect();
        //println!("{} {} {}", num_bits, num_items, num_keys);
        solana_bloom::bloom::Bloom::new(num_bits, keys)
    }
    fn extend<I: Iterator<Item = u64>>(&mut self, items: I) {
        for x in items {
            let mut b = [0u8; 32];
            b[0..8].copy_from_slice(&x.to_ne_bytes());
            //b[8..16].copy_from_slice(&x.to_ne_bytes());
            //b[16..24].copy_from_slice(&x.to_ne_bytes());
            //b[24..32].copy_from_slice(&x.to_ne_bytes());

            self.add(&solana_program::hash::Hash::new_from_array(b));
        }
    }
    fn name() -> &'static str {
        "solana-bloom"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::false_pos_rate_adaptive;

    #[test]
    fn simple() {
        let num_bits = 1 << 12;
        let num_items = num_bits / 10;
        let mut b: solana_bloom::bloom::Bloom<solana_program::hash::Hash> =
            Container::new(num_bits, num_items);

        b.extend(0..num_items as u64);
        let fp = false_pos_rate_adaptive(&mut b, num_items as u64..=u64::MAX);
        println!("{:?}", fp);
    }
}

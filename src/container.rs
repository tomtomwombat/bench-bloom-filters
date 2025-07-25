use bloom::ASMS;
use bloomfilter::Bloom;
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

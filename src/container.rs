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
    fn new<I: IntoIterator<Item = X>>(num_bits: usize, items: I, num_items: usize) -> Self;
    fn name() -> &'static str;
}

macro_rules! impl_container_fastbloom {
    ($($size:literal = $fn_name:ident = $hasher:ty = $name:literal),* $(,)*) => (
        $(
            impl<X: Hash> Container<X> for BloomFilter<$size, $hasher> {
                #[inline]
                fn check(&self, s: &X) -> bool {
                    self.contains(s)
                }
                fn num_hashes(&self) -> usize {
                    self.num_hashes() as usize
                }
                fn new<I: IntoIterator<Item = X>>(
                    num_bits: usize,
                    items: I,
                    num_items: usize,
                ) -> Self {
                    let mut res = BloomFilter::with_num_bits(num_bits)
                        .$fn_name()
                        .hasher(<$hasher>::default())
                        .expected_items(num_items);
                    for x in items.into_iter() {
                        res.insert(&x);
                    }
                    res
                }
                fn name() -> &'static str {
                    $name
                }
            }
        )*
    )
}
impl_container_fastbloom!(
    512 = block_size_512 = fastbloom::DefaultHasher = "fastbloom",
    256 = block_size_256 = fastbloom::DefaultHasher = "fastbloom - 256",
    128 = block_size_128 = fastbloom::DefaultHasher = "fastbloom - 128",
    64 = block_size_64 = fastbloom::DefaultHasher = "fastbloom - 64",
    //512 = block_size_512 = ahash::RandomState = "fastbloom",
    //256 = block_size_256 = ahash::RandomState = "fastbloom - 256",
    //128 = block_size_128 = ahash::RandomState = "fastbloom - 128",
    //64 = block_size_64 = ahash::RandomState = "fastbloom - 64",
);

impl<X: Hash> Container<X> for Bloom<X> {
    #[inline]
    fn check(&self, s: &X) -> bool {
        self.check(s)
    }
    fn num_hashes(&self) -> usize {
        self.number_of_hash_functions() as usize
    }
    fn new<I: IntoIterator<Item = X>>(num_bits: usize, items: I, num_items: usize) -> Self {
        let items = items.into_iter();
        let mut filter = Bloom::<X>::new(num_bits / 8, num_items);
        for x in items {
            filter.set(&x);
        }
        filter
    }
    fn name() -> &'static str {
        "bloomfilter"
    }
}

macro_rules! impl_xxh3_container_fastbloom {
    ($($size:literal = $fn_name:ident = $name:literal),* $(,)*) => (
        $(
            impl Container<u64> for BloomFilter<$size, XXHashWrapper> {
                #[inline]
                fn check(&self, s: &u64) -> bool {
                    self.contains(&xxhash_rust::xxh3::xxh3_64(&s.to_be_bytes()))
                }
                fn num_hashes(&self) -> usize {
                    self.num_hashes() as usize
                }
                fn new<I: IntoIterator<Item = u64>>(
                    num_bits: usize,
                    items: I,
                    num_items: usize,
                ) -> Self {
                    let mut res = BloomFilter::with_num_bits(num_bits)
                        .$fn_name()
                        .hasher(XXHashWrapper(0))
                        .expected_items(num_items);
                    for x in items.into_iter() {
                        res.insert(& xxhash_rust::xxh3::xxh3_64(&x.to_be_bytes()));
                    }
                    res
                }
                fn name() -> &'static str {
                    stringify!($fn_name)
                }
            }
        )*
    )
}

impl_xxh3_container_fastbloom!(
    512 = block_size_512 = "fastbloom - 512 - xxhash",
    256 = block_size_256 = "fastbloom - 256 - xxhash",
    128 = block_size_128 = "fastbloom - 128 - xxhash",
    64 = block_size_64 = "fastbloom - 64 - xxhash",
);

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
        0
    }
    fn new<I: IntoIterator<Item = u64>>(num_bits: usize, items: I, _num_items: usize) -> Self {
        let items = items.into_iter();
        let mut filter = sbbf_rs_safe::Filter::new(num_bits, 1);
        for x in items {
            filter.insert_hash(xxhash_rust::xxh3::xxh3_64(&x.to_be_bytes()));
        }
        filter
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
    fn new<I: IntoIterator<Item = u64>>(num_bits: usize, items: I, num_items: usize) -> Self {
        let items = items.into_iter();
        let hashes = bloom::bloom::optimal_num_hashes(num_bits, num_items as u32);
        let mut filter = fastbloom_rs::FilterBuilder::from_size_and_hashes(num_bits as u64, hashes)
            .build_bloom_filter();
        for x in items {
            filter.add(&x.to_be_bytes());
        }
        filter
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
    fn new<I: IntoIterator<Item = X>>(num_bits: usize, items: I, num_items: usize) -> Self {
        let items = items.into_iter();
        let hashes = bloom::bloom::optimal_num_hashes(num_bits, num_items as u32);
        let mut filter = bloom::BloomFilter::with_size(num_bits, hashes);
        for x in items {
            filter.insert(&x);
        }
        filter
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
    fn new<I: IntoIterator<Item = X>>(num_bits: usize, items: I, num_items: usize) -> Self {
        let items = items.into_iter();
        let mut filter = ProbBloomFilter::<X>::from_item_count(num_bits, num_items);
        for x in items {
            filter.insert(&x);
        }
        filter
    }
    fn name() -> &'static str {
        "probabilistic-collections"
    }
}

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
use wyhash::wyhash;

#[allow(dead_code)]
pub fn false_pos_rate_with_vals<X: Hash + Eq + PartialEq>(
    filter: &impl Container<X>,
    control: &HashSet<X>,
    anti_vals: impl IntoIterator<Item = X>,
) -> f64 {
    let mut total = 0;
    let mut false_positives = 0;
    for x in anti_vals.into_iter() {
        if !control.contains(&x) {
            total += 1;
            false_positives += filter.check(&x) as usize;
        }
    }
    (false_positives as f64) / (total as f64)
}

#[allow(dead_code)]
pub fn list_fp<T: Container<u64>>() {
    let thresh = 0.1;
    let amount = 100_000;
    for bloom_size_bytes in [262144] {
        let mut fp = 0.0;
        for num_items_base in (8..23).map(|x| 1 << x) {
            let all_num_items: Vec<usize> = if fp > 0.0 && fp < thresh {
                let step = num_items_base >> 8;
                ((num_items_base >> 1 + step)..(num_items_base << 1))
                    .step_by(step)
                    .collect()
            } else {
                std::iter::once(num_items_base).collect()
            };
            for num_items in all_num_items {
                if num_items == 0 {
                    continue;
                }
                let sample_vals = random_numbers(num_items, 42);

                let num_bits = bloom_size_bytes * 8;
                let filter = T::new(num_bits, sample_vals.clone().into_iter()); //BloomFilter::builder512(num_bits).items(sample_vals.iter());
                let control: HashSet<u64> = sample_vals.into_iter().collect();
                fp = false_pos_rate_with_vals(&filter, &control, random_numbers(amount, 43));
                print!("{:?}, ", num_items);
                print!("{:?}, ", bloom_size_bytes);
                print!("{:?}, ", filter.num_hashes());
                print!("{:.8}", fp);
                println!("");
                if fp > thresh {
                    break;
                }
            }
            if fp > thresh {
                break;
            }
        }
    }
}

pub fn random_strings(num: usize, min_repeat: u32, max_repeat: u32, seed: u64) -> Vec<String> {
    let mut rng = StdRng::seed_from_u64(seed);
    let gen = rand_regex::Regex::compile(r"[a-zA-Z]+", max_repeat).unwrap();
    (&mut rng)
        .sample_iter(&gen)
        .filter(|s: &String| s.len() >= min_repeat as usize)
        .take(num)
        .collect()
}
pub fn random_numbers(num: usize, seed: u64) -> Vec<u64> {
    let mut rng = StdRng::seed_from_u64(seed);
    repeat(()).take(num).map(|_| rng.gen()).collect()
}

pub trait Container<X: Hash> {
    fn check(&self, s: &X) -> bool;
    fn num_hashes(&self) -> usize;
    fn new<I: IntoIterator<IntoIter = impl ExactSizeIterator<Item = X>>>(
        num_bits: usize,
        items: I,
    ) -> Self;
    fn name() -> &'static str;
}

impl<X: Hash, H: BuildHasher + Default> Container<X> for BloomFilter<512, H> {
    #[inline]
    fn check(&self, s: &X) -> bool {
        self.contains(s)
    }
    fn num_hashes(&self) -> usize {
        self.num_hashes() as usize
    }
    fn new<I: IntoIterator<IntoIter = impl ExactSizeIterator<Item = X>>>(
        num_bits: usize,
        items: I,
    ) -> Self {
        BloomFilter::with_num_bits(num_bits)
            .block_size_512()
            .hasher(H::default())
            .items(items)
    }
    fn name() -> &'static str {
        "fastbloom - 512"
    }
}

impl<X: Hash, H: BuildHasher + Default> Container<X> for BloomFilter<256, H> {
    #[inline]
    fn check(&self, s: &X) -> bool {
        self.contains(s)
    }
    fn num_hashes(&self) -> usize {
        self.num_hashes() as usize
    }
    fn new<I: IntoIterator<IntoIter = impl ExactSizeIterator<Item = X>>>(
        num_bits: usize,
        items: I,
    ) -> Self {
        BloomFilter::with_num_bits(num_bits)
            .block_size_256()
            .hasher(H::default())
            .items(items)
    }
    fn name() -> &'static str {
        "fastbloom - 256"
    }
}

impl<X: Hash, H: BuildHasher + Default> Container<X> for BloomFilter<128, H> {
    #[inline]
    fn check(&self, s: &X) -> bool {
        self.contains(s)
    }
    fn num_hashes(&self) -> usize {
        self.num_hashes() as usize
    }
    fn new<I: IntoIterator<IntoIter = impl ExactSizeIterator<Item = X>>>(
        num_bits: usize,
        items: I,
    ) -> Self {
        BloomFilter::with_num_bits(num_bits)
            .block_size_128()
            .hasher(H::default())
            .items(items)
    }
    fn name() -> &'static str {
        "fastbloom - 128"
    }
}

impl<X: Hash, H: BuildHasher + Default> Container<X> for BloomFilter<64, H> {
    #[inline]
    fn check(&self, s: &X) -> bool {
        self.contains(s)
    }
    fn num_hashes(&self) -> usize {
        self.num_hashes() as usize
    }
    fn new<I: IntoIterator<IntoIter = impl ExactSizeIterator<Item = X>>>(
        num_bits: usize,
        items: I,
    ) -> Self {
        BloomFilter::with_num_bits(num_bits)
            .block_size_64()
            .hasher(H::default())
            .items(items)
    }
    fn name() -> &'static str {
        "fastbloom - 64"
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
    fn new<I: IntoIterator<IntoIter = impl ExactSizeIterator<Item = X>>>(
        num_bits: usize,
        items: I,
    ) -> Self {
        let items = items.into_iter();
        let mut filter = Bloom::<X>::new(num_bits / 8, items.len());
        for x in items {
            filter.set(&x);
        }
        filter
    }
    fn name() -> &'static str {
        "bloomfilter"
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
    fn new<I: IntoIterator<IntoIter = impl ExactSizeIterator<Item = X>>>(
        num_bits: usize,
        items: I,
    ) -> Self {
        let items = items.into_iter();
        let hashes = bloom::bloom::optimal_num_hashes(num_bits, items.len() as u32);
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
    fn new<I: IntoIterator<IntoIter = impl ExactSizeIterator<Item = X>>>(
        num_bits: usize,
        items: I,
    ) -> Self {
        let items = items.into_iter();
        let mut filter = ProbBloomFilter::<X>::from_item_count(num_bits, items.len());
        for x in items {
            filter.insert(&x);
        }
        filter
    }
    fn name() -> &'static str {
        "probabilistic-collections"
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
    fn new<I: IntoIterator<IntoIter = impl ExactSizeIterator<Item = u64>>>(
        num_bits: usize,
        items: I,
    ) -> Self {
        let items = items.into_iter();
        let hashes = bloom::bloom::optimal_num_hashes(num_bits, items.len() as u32);
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
impl Container<String> for fastbloom_rs::BloomFilter {
    #[inline]
    fn check(&self, s: &String) -> bool {
        self.contains(&s.as_bytes())
    }
    fn num_hashes(&self) -> usize {
        self.hashes() as usize
    }
    fn new<I: IntoIterator<IntoIter = impl ExactSizeIterator<Item = String>>>(
        num_bits: usize,
        items: I,
    ) -> Self {
        let items = items.into_iter();
        let hashes = bloom::bloom::optimal_num_hashes(num_bits, items.len() as u32);
        let mut filter = fastbloom_rs::FilterBuilder::from_size_and_hashes(num_bits as u64, hashes)
            .build_bloom_filter();
        for x in items {
            filter.add(&x.as_bytes());
        }
        filter
    }
    fn name() -> &'static str {
        "fastbloom-rs"
    }
}

impl Container<String> for sbbf_rs_safe::Filter {
    #[inline]
    fn check(&self, s: &String) -> bool {
        self.contains_hash(wyhash(s.as_bytes(), 0))
    }
    fn num_hashes(&self) -> usize {
        todo!()
    }
    fn new<I: IntoIterator<IntoIter = impl ExactSizeIterator<Item = String>>>(
        num_bits: usize,
        items: I,
    ) -> Self {
        let items = items.into_iter();
        let hashes = bloom::bloom::optimal_num_hashes(num_bits, items.len() as u32);
        let mut filter = sbbf_rs_safe::Filter::new(hashes as usize, items.len());
        for x in items {
            filter.insert_hash(wyhash(x.as_bytes(), 0));
        }
        filter
    }
    fn name() -> &'static str {
        "sbbf"
    }
}

impl Container<u64> for sbbf_rs_safe::Filter {
    #[inline]
    fn check(&self, s: &u64) -> bool {
        self.contains_hash(wyhash(&s.to_be_bytes(), 0))
    }
    fn num_hashes(&self) -> usize {
        0
    }
    fn new<I: IntoIterator<IntoIter = impl ExactSizeIterator<Item = u64>>>(
        num_bits: usize,
        items: I,
    ) -> Self {
        let items = items.into_iter();
        let hashes = bloom::bloom::optimal_num_hashes(num_bits, items.len() as u32);
        let mut filter = sbbf_rs_safe::Filter::new(hashes as usize, items.len());
        for x in items {
            filter.insert_hash(wyhash(&x.to_be_bytes(), 0));
        }
        filter
    }
    fn name() -> &'static str {
        "sbbf"
    }
}

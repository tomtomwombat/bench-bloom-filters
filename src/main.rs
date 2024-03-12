use bloom_filter_benches::random_numbers;
use std::collections::HashSet;

use bloomfilter::Bloom;

use bloom_filter_benches::false_pos_rate_with_vals;
use bloom_filter_benches::Container;

fn main() {
    for mag in 1..=6 {
        let fp = 1.0f64 / 10u64.pow(mag) as f64;
        for num_items_mag in 1..6 {
            let num_items = 10usize.pow(num_items_mag);
            let sample_vals = random_numbers(num_items, 42);

            let mut filter = Bloom::new_for_fp_rate(num_items, fp);
            for x in sample_vals.iter() {
                filter.set(x);
            }
            // println!("bits: {:?}, hashes: {:?}", filter.number_of_bits(), filter.number_of_hash_functions());
            let control: HashSet<u64> = sample_vals.clone().into_iter().collect();
            let anti_vals = random_numbers(10_000, 3);
            let sample_fp = false_pos_rate_with_vals(&filter, &control, anti_vals);
            println!("{:?}", fp / sample_fp);
            // println!("target: {:?}, sample: {:?}", fp, sample_fp);
        }
    }
}

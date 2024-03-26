use bloom_filter_benches::random_numbers;
use std::collections::HashSet;

use bloom_filter_benches::false_pos_rate_with_vals;
use bloom_filter_benches::list_fp;
use bloom_filter_benches::Container;
use bloomfilter::Bloom;

fn main() {
    list_fp::<fastbloom::BloomFilter<128>>();
}

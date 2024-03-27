use bloom_filter_benches::*;

fn main() {
    list_fp::<fastbloom::BloomFilter<512, XXHashWrapper>>();
    // list_fp::<fastbloom::BloomFilter<64, XXHashWrapper>>();
    // list_fp::<sbbf_rs_safe::Filter>();
}

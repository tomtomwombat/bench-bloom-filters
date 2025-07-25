use bloom_filter_benches::*;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn write_false_pos_data<T: Container<u64>>() -> std::io::Result<()> {
    let now = Instant::now();
    let res = list_fp::<T>();
    let mut file = File::create(format!("BloomFilter-False-Positives/{}.csv", T::name()))?;
    for (num_items, size) in res {
        let row = format!("{},{}\n", num_items, size);
        file.write_all(row.as_bytes())?;
    }
    println!(
        "{} complete in {} seconds",
        T::name(),
        now.elapsed().as_secs()
    );
    Ok(())
}

fn main() -> std::io::Result<()> {
    write_false_pos_data::<bloom::BloomFilter>()?;
    write_false_pos_data::<sbbf_rs_safe::Filter>()?;
    write_false_pos_data::<bloomfilter::Bloom<u64>>()?;
    write_false_pos_data::<probabilistic_collections::bloom::BloomFilter<u64>>()?;

    // write_false_pos_data::<fastbloom_rs::BloomFilter>()?;

    write_false_pos_data::<fastbloom::BloomFilter<ahash::RandomState>>()?;
    Ok(())
}

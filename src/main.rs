use bloom_filter_benches::*;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn write_false_pos_data<T: Container<u64>>() -> std::io::Result<()> {
    let now = Instant::now();
    let res = list_fp::<T>(1 << 12);
    std::fs::create_dir_all("Acc")?;
    let mut file = File::create(format!("Acc/{}.csv", T::name()))?;
    for (num_items, avg, min, max) in res {
        let row = format!("{},{},{},{}\n", num_items, avg, min, max);
        file.write_all(row.as_bytes())?;
    }

    let complete = format!("{}", T::name());
    let time = format!("{:>4} seconds", now.elapsed().as_secs());
    println!("✅ {:28}    {}", complete, time);
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("{}", "Benching Bloom filters...\n");
    let now = Instant::now();

    write_false_pos_data::<fastbloom::BloomFilter<foldhash::fast::RandomState>>()?;
    write_false_pos_data::<bloom::BloomFilter>()?;
    write_false_pos_data::<sbbf_rs_safe::Filter>()?;
    write_false_pos_data::<bloomfilter::Bloom<u64>>()?;
    write_false_pos_data::<probabilistic_collections::bloom::BloomFilter<u64>>()?;
    write_false_pos_data::<solana_bloom::bloom::Bloom<solana_program::hash::Hash>>()?;
    write_false_pos_data::<datasketches::bloom::BloomFilter>()?;
    // write_false_pos_data::<crate::RandomFilter>()?;
    // write_false_pos_data::<fastbloom_rs::BloomFilter>()?;

    println!("\nDone in {} seconds.", now.elapsed().as_secs());
    Ok(())
}

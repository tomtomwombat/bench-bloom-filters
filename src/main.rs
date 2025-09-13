use bloom_filter_benches::*;
use colored::Colorize;
use console::strip_ansi_codes;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use unicode_width::UnicodeWidthStr;

macro_rules! box_println_round {
    ($($arg:tt)*) => {{
        let content = format!($($arg)*);
        let x = strip_ansi_codes(&content);
        let stripped = String::from_utf8_lossy(x.as_bytes());
        let width = UnicodeWidthStr::width(stripped.as_ref());

        let top = format!("╭{}╮", "─".repeat(width + 2));
        let mid = format!("│ {} │", content);
        let bot = format!("╰{}╯", "─".repeat(width + 2));

        println!("{}", top);
        println!("{}", mid);
        println!("{}", bot);
    }};
}

macro_rules! box_println {
    ($($arg:tt)*) => {{
        let content = format!($($arg)*);
        let x = strip_ansi_codes(&content);
        let stripped = String::from_utf8_lossy(x.as_bytes());
        let width = UnicodeWidthStr::width(stripped.as_ref());

        let top = format!("┌{}┐", "─".repeat(width + 2));
        let mid = format!("│ {} │", content);
        let bot = format!("└{}┘", "─".repeat(width + 2));

        println!("{}", top);
        println!("{}", mid);
        println!("{}", bot);
    }};
}

fn write_false_pos_data<T: Container<u64>>() -> std::io::Result<()> {
    let now = Instant::now();
    let res = list_fp2::<T>(1 << 12);
    let mut file = File::create(format!("Acc/{}.csv", T::name()))?;
    for (num_items, avg, min, max) in res {
        let row = format!("{},{},{},{}\n", num_items, avg, min, max);
        file.write_all(row.as_bytes())?;
    }

    let complete = format!("{}", T::name());
    let time = format!("{:>4} seconds", now.elapsed().as_secs());
    println!("✅ {:28}  ⏱️  {}", complete.green(), time.purple());
    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("{}", "Benching Bloom filters...\n".purple().bold());
    let now = Instant::now();
    write_false_pos_data::<bloom::BloomFilter>()?;
    write_false_pos_data::<sbbf_rs_safe::Filter>()?;
    write_false_pos_data::<bloomfilter::Bloom<u64>>()?;
    write_false_pos_data::<probabilistic_collections::bloom::BloomFilter<u64>>()?;
    write_false_pos_data::<fastbloom::BloomFilter<ahash::RandomState>>()?;
    write_false_pos_data::<solana_bloom::bloom::Bloom<solana_program::hash::Hash>>()?;
    // write_false_pos_data::<crate::RandomFilter>()?;
    // write_false_pos_data::<fastbloom_rs::BloomFilter>()?;

    println!("");
    let done = format!("Done in {} seconds.", now.elapsed().as_secs());
    box_println!("{}", done.purple().bold());
    Ok(())
}

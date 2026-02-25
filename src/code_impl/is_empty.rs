// Test Vec is_empty() vs len() == 0, avoid empty optimization by filling some data and pushing one element

pub struct IsEmptyBench;

use std::{hint::black_box, time::Instant};
use crate::code_impl::SnipptBench;

impl SnipptBench for IsEmptyBench {
    fn run(&self) {
        let mut vec: Vec<u64> = Vec::new();
        vec.resize(1000, 0); // Fill with some data to avoid empty optimization
        vec.push(10000);
        let mut acc = 0u64;
        let start = Instant::now();
        for _ in 0..100_000_000 {
            if vec.is_empty() {
                // do nothing
                acc ^= 1;
            }
        }
        black_box(acc);
        let elapsed = start.elapsed();
        println!("Vec is_empty(): {:?}", elapsed);

        let mut acc = 0u64;
        let start = Instant::now();
        for _ in 0..100_000_000 {
            if vec.len() == 0 {
                // do nothing
                acc ^= 1;
            }
        }
        black_box(acc);
        let elapsed = start.elapsed();
        println!("Vec len() == 0: {:?}", elapsed);
        
    }
    fn description(&self) -> &str {
        "Benchmark Vec is_empty() vs len() == 0 performance."
    }
    fn name(&self) -> &str {
        "IsEmptyBench"
    }
}
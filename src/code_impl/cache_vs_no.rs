use std::time::Instant;
use crate::code_impl::SnipptBench;

pub struct CacheVsNoBench;

const N: usize = 128 * 1024 * 1024; // 128M
const STEP: usize = 4096; // 4K step, likely exceeds L1/L2 cache

fn bench_seq_access() {
    let data = vec![1u8; N];
    let mut sum = 0u64;
    let start = Instant::now();
    for i in 0..N {
        sum = sum.wrapping_add(data[i] as u64);
    }
    let elapsed = start.elapsed();
    println!("Sequential access: {:?}, sum={}", elapsed, sum);
}

fn bench_stride_access() {
    let data = vec![1u8; N];
    let mut sum = 0u64;
    let start = Instant::now();
    for offset in 0..STEP {
        let mut i = offset;
        while i < N {
            sum = sum.wrapping_add(data[i] as u64);
            i += STEP;
        }
    }
    let elapsed = start.elapsed();
    println!("Stride access ({}): {:?}, sum={}", STEP, elapsed, sum);
}

impl SnipptBench for CacheVsNoBench {
    fn run(&self) {
        bench_seq_access();
        bench_stride_access();
    }
    fn description(&self) -> &str {
        "Compare CPU cache-friendly (sequential) and cache-unfriendly (stride) memory access."
    }
    fn name(&self) -> &str {
        "CacheVsNoBench"
    }
}

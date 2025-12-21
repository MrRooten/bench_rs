use std::{hint::black_box, time::Instant};
use crate::code_impl::SnipptBench;

const N: usize = 100_000_000;
const ARR_SIZE: usize = 128;

pub struct ListVsPtrBench;

#[inline(never)]
fn sum_slice(arr: &[u8]) -> u64 {
    arr.iter().map(|&x| x as u64).sum::<u64>()
}

#[inline(never)]
fn sum_ptr(ptr: *const u8, len: usize) -> u64 {
    let arr = unsafe { std::slice::from_raw_parts(ptr, len) };
    arr.iter().map(|&x| x as u64).sum::<u64>()
}

impl ListVsPtrBench {
    fn bench_slice() {
        let arr = vec![1u8; ARR_SIZE];
        let start = Instant::now();
        let mut total = 0u64;
        for _ in 0..N {
            total += black_box(sum_slice(&arr));
        }
        let elapsed = start.elapsed();
        println!("Function param &[u8]: {:?}, total: {}", elapsed, total);
    }

    fn bench_ptr() {
        let arr = vec![1u8; ARR_SIZE];
        let start = Instant::now();
        let mut total = 0u64;
        let ptr = arr.as_ptr();
        for _ in 0..N {
            total += black_box(sum_ptr(ptr, ARR_SIZE));
        }
        let elapsed = start.elapsed();
        println!("Function param *const u8 + len: {:?}, total: {}", elapsed, total);
    }
}

impl SnipptBench for ListVsPtrBench {
    fn run(&self) {
        Self::bench_slice();
        Self::bench_ptr();
    }
    fn description(&self) -> &str {
        "Benchmark function call performance: &[u8] vs *const u8 + len as parameters."
    }
    fn name(&self) -> &str {
        "ListVsPtrBench"
    }
}

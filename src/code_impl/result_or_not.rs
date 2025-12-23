use std::{hint::black_box, time::Instant};
use crate::code_impl::SnipptBench;

const N: usize = 100_000_000;

pub struct ResultOrNotBench;

#[inline(never)]
fn return_value(x: u64) -> u64 {
    if x == u64::MAX {
        panic!("Overflow");
    }
    x + 1
}

#[inline(never)]
fn return_result(x: u64) -> Result<u64, &'static str> {
    if x == u64::MAX {
        return Err("Overflow");
    }
    Ok(x + 1)
}

impl ResultOrNotBench {
    fn bench_value() {
        let mut sum = 0u64;
        let mut acc = 1u64;
        let start = Instant::now();
        for i in 0..N {
            // Use acc to prevent loop invariant code motion
            acc = black_box(acc.wrapping_mul(31).wrapping_add(i as u64));
            sum = sum.wrapping_add(black_box(return_value(acc)));
        }
        let elapsed = start.elapsed();
        println!("Return value: {:?}, sum={}", elapsed, sum);
        black_box(sum); // prevent optimization
    }

    fn bench_result() {
        let mut sum = 0u64;
        let mut acc = 1u64;
        let start = Instant::now();
        for i in 0..N {
            acc = black_box(acc.wrapping_mul(31).wrapping_add(i as u64));
            sum = sum.wrapping_add(black_box(return_result(acc).unwrap()));
        }
        let elapsed = start.elapsed();
        println!("Return Result: {:?}, sum={}", elapsed, sum);
        black_box(sum);
    }
}

impl SnipptBench for ResultOrNotBench {
    fn run(&self) {
        Self::bench_value();
        Self::bench_result();
    }
    fn description(&self) -> &str {
        "Benchmark performance difference between returning Result<T, E> and returning T."
    }
    fn name(&self) -> &str {
        "ResultOrNotBench"
    }
}

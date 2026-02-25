use std::num::NonZeroU64;
use std::time::Instant;
use std::ptr;
use crate::code_impl::SnipptBench;

const N: usize = 100_000_000;

pub struct NonZeroBench;

fn sum_u64(x: u64) -> u64 {
    x.wrapping_add(1)
}

fn sum_nonzero(x: NonZeroU64) -> NonZeroU64 {
    // Always safe: x.get() != 0, so x.get() + 1 != 0 for large N
    unsafe { NonZeroU64::new_unchecked(x.get().wrapping_add(1)) }
}

fn ptr_from_nonzero(x: &NonZeroU64) -> *const u64 {
    x as *const NonZeroU64 as *const u64
}

fn ptr_from_u64(x: &u64) -> *const u64 {
    x as *const u64
}

impl NonZeroBench {
    fn bench_u64() {
        let mut sum = 1u64;
        let start = Instant::now();
        for i in 1..=N as u64 {
            sum = sum_u64(sum ^ i);
        }
        let elapsed = start.elapsed();
        println!("u64 value calculation: {:?}, sum={}", elapsed, sum);
    }

    fn bench_nonzero() {
        let mut sum = NonZeroU64::new(1).unwrap();
        let start = Instant::now();
        for i in 1..=N as u64 {
            sum = sum_nonzero(unsafe { NonZeroU64::new_unchecked(sum.get() ^ i) });
        }
        let elapsed = start.elapsed();
        println!("NonZeroU64 value calculation: {:?}, sum={}", elapsed, sum.get());
    }

    fn bench_ptr_u64() {
        let mut sum = 1u64;
        let mut last_ptr = ptr::null();
        let start = Instant::now();
        for i in 1..=N as u64 {
            sum = sum_u64(sum ^ i);
            last_ptr = ptr_from_u64(&sum);
        }
        let elapsed = start.elapsed();
        println!("u64 pointer conversion: {:?}, last_ptr={:?}", elapsed, last_ptr);
    }

    fn bench_ptr_nonzero() {
        let mut sum = NonZeroU64::new(1).unwrap();
        let mut last_ptr = ptr::null();
        let start = Instant::now();
        for i in 1..=N as u64 {
            sum = sum_nonzero(unsafe { NonZeroU64::new_unchecked(sum.get() ^ i) });
            last_ptr = ptr_from_nonzero(&sum);
        }
        let elapsed = start.elapsed();
        println!("NonZeroU64 pointer conversion: {:?}, last_ptr={:?}", elapsed, last_ptr);
    }

    fn bench_nonzero_new() {
        let mut sum = NonZeroU64::new(1).unwrap();
        let start = Instant::now();
        for _ in 1..=N as u64 {
            // Always nonzero, so unwrap is safe
            sum = NonZeroU64::new(sum.get()).unwrap();
        }
        let elapsed = start.elapsed();
        println!("NonZeroU64::new + unwrap: {:?}, sum={}", elapsed, sum.get());
    }

    fn bench_nonzero_unsafe() {
        let mut sum = NonZeroU64::new(1).unwrap();
        let start = Instant::now();
        for _ in 1..=N as u64 {
            // Always nonzero, so unsafe is safe
            sum = unsafe { NonZeroU64::new_unchecked(sum.get()) };
        }
        let elapsed = start.elapsed();
        println!("NonZeroU64::new_unchecked: {:?}, sum={}", elapsed, sum.get());
    }
}

impl SnipptBench for NonZeroBench {
    fn run(&self) {
        Self::bench_u64();
        Self::bench_nonzero();
        Self::bench_ptr_u64();
        Self::bench_ptr_nonzero();
        Self::bench_nonzero_new();
        Self::bench_nonzero_unsafe();
    }
    fn description(&self) -> &str {
        "Benchmark performance of NonZeroU64 vs u64 for value calculation and pointer conversion."
    }
    fn name(&self) -> &str {
        "NonZeroBench"
    }
}

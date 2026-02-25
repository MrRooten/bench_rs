use crate::code_impl::SnipptBench;
use std::hint::black_box;
use std::time::Instant;

pub struct SliceVsVecGetBench;

impl SnipptBench for SliceVsVecGetBench {
    fn run(&self) {
        const N: usize = 1024;
        const REPS: usize = 100_000_00;
        let vec: Vec<u32> = (0..N as u32).collect();
        let slice: &[u32] = &vec;
        let idx = N / 2;

        let start = Instant::now();
        let mut acc = 0u32;
        for _ in 0..REPS {
            acc ^= black_box(slice.get(idx).copied().unwrap());
        }
        println!("slice.get: {:?}", start.elapsed());
        black_box(acc);

        let start = Instant::now();
        let mut acc = 0u32;
        for _ in 0..REPS {
            acc ^= black_box(vec.get(idx).copied().unwrap());
        }
        println!("vec.get: {:?}", start.elapsed());
        black_box(acc);
    }
    fn description(&self) -> &str {
        "Compare performance of slice.get vs vec.get."
    }
    fn name(&self) -> &str {
        "SliceVsVecGetBench"
    }
}

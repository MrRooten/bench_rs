use crate::code_impl::SnipptBench;
use std::hint::black_box;
use std::time::Instant;

pub struct WhileVsLetBench;

impl SnipptBench for WhileVsLetBench {
    fn run(&self) {
        const N: usize = 1024;
        const REPS: usize = 1_000_000;
        let data: Vec<u32> = (0..N as u32).collect();

        // while let Some(x) = iter.next()
        let start = Instant::now();
        let mut acc = 0u32;
        for _ in 0..REPS {
            let mut iter = data.iter();
            while let Some(x) = iter.next() {
                acc ^= black_box(*x);
            }
        }
        println!("while let Some: {:?}", start.elapsed());
        black_box(acc);

        // while idx < data.len()
        let start = Instant::now();
        let mut acc = 0u32;
        for _ in 0..REPS {
            let mut idx = 0;
            while idx < data.len() {
                acc ^= black_box(data[idx]);
                idx += 1;
            }
        }
        println!("while idx < len: {:?}", start.elapsed());
        black_box(acc);

        // for x in &data
        let start = Instant::now();
        let mut acc = 0u32;
        for _ in 0..REPS {
            for x in &data {
                acc ^= black_box(*x);
            }
        }
        println!("for x in &data: {:?}", start.elapsed());
        black_box(acc);
    }
    fn description(&self) -> &str {
        "Compare while let Some, while idx < len, and for loop performance."
    }
    fn name(&self) -> &str {
        "WhileVsLetBench"
    }
}

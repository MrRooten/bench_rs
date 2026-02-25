// bench pass &Vec<usize> or &[usize] to a function, compare performance

pub struct SliceVsRefBench;

use std::{hint::black_box, time::Instant};
use crate::code_impl::SnipptBench;

impl SnipptBench for SliceVsRefBench {
    fn run(&self) {
        let vec: Vec<usize> = (0..1000).collect();
        let start = Instant::now();
        for _ in 0..100_000_0000 {
            black_box(sum_slice(&vec));
        }
        let elapsed = start.elapsed();
        println!("Passing &[usize]: {:?}", elapsed);

        let start = Instant::now();
        for _ in 0..100_000_0000 {
            black_box(sum_ref(&vec));
        }
        let elapsed = start.elapsed();
        println!("Passing &Vec<usize>: {:?}", elapsed);
    }
    fn description(&self) -> &str {
        "Benchmark passing &[usize] vs &Vec<usize> performance."
    }
    fn name(&self) -> &str {
        "SliceVsRefBench"
    }
}

fn sum_slice(slice: &[usize]) {
    // slice.iter().sum()
    black_box(slice);
}

fn sum_ref(vec: &Vec<usize>) {
    // vec.iter().sum()
    black_box(vec);
}
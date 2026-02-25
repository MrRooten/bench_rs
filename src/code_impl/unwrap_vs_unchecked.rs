use crate::code_impl::SnipptBench;
use std::hint::black_box;
use std::time::Instant;

pub struct UnwrapVsUncheckedBench;

impl SnipptBench for UnwrapVsUncheckedBench {
    fn run(&self) {
        const N: usize = 100_000_000;
        let mut acc = 0u64;
        let some = Some(42u64);

        let start = Instant::now();
        for _ in 0..N {
            acc ^= black_box(some).unwrap();
        }
        println!("Option::unwrap: {:?}", start.elapsed());
        black_box(acc);

        let mut acc2 = 0u64;
        let some = Some(42u64);
        let start = Instant::now();
        for _ in 0..N {
            // SAFETY: always Some
            acc2 ^= unsafe { black_box(some).unwrap_unchecked() };
        }
        println!("Option::unwrap_unchecked: {:?}", start.elapsed());
        black_box(acc2);

        // baseline
        let mut acc3 = 0u64;
        let start = Instant::now();
        for _ in 0..N {
            acc3 ^= black_box(42u64);
        }
        println!("Baseline: {:?}", start.elapsed());
        black_box(acc3);
    }

    fn description(&self) -> &str {
        "Benchmark Option::unwrap vs Option::unwrap_unchecked performance."
    }
    fn name(&self) -> &str {
        "UnwrapVsUncheckedBench"
    }
}

use std::time::Instant;
use crate::code_impl::SnipptBench;

const N: usize = 5_000_000;

#[derive(Clone)]
struct BigStruct {
    a: u64,
    b: u64,
    c: [u8; 256], // simulate a large field
    d: u64,
}

#[derive(Clone)]
struct PartialBoxStruct {
    a: u64,
    b: u64,
    c: Box<[u8; 256]>, // only the large field is boxed
    d: u64,
}

// 1. Vec<full struct> 2. Vec<Box<struct>> 3. Vec<struct with large field boxed>
pub struct VecStructBench;
pub struct VecAccessBench;

impl VecStructBench {
    fn bench_vec_struct() {
        let mut v = Vec::with_capacity(N);
        for i in 0..N {
            v.push(BigStruct { a: i as u64, b: i as u64, c: [0; 256], d: i as u64 });
        }
        let start = Instant::now();
        let sum: u64 = v.iter().map(|x| x.a + x.b + x.c[0] as u64 + x.d).sum();
        let elapsed = start.elapsed();
        println!("Vec<BigStruct> iterate+access: {:?}, sum={}", elapsed, sum);
    }

    fn bench_vec_boxed_struct() {
        let mut v = Vec::with_capacity(N);
        for i in 0..N {
            v.push(Box::new(BigStruct { a: i as u64, b: i as u64, c: [0; 256], d: i as u64 }));
        }
        let start = Instant::now();
        let sum: u64 = v.iter().map(|x| x.a + x.b + x.c[0] as u64 + x.d).sum();
        let elapsed = start.elapsed();
        println!("Vec<Box<BigStruct>> iterate+access: {:?}, sum={}", elapsed, sum);
    }

    fn bench_vec_partial_box() {
        let mut v = Vec::with_capacity(N);
        for i in 0..N {
            v.push(PartialBoxStruct {
                a: i as u64,
                b: i as u64,
                c: Box::new([0; 256]),
                d: i as u64,
            });
        }
        let start = Instant::now();
        let sum: u64 = v.iter().map(|x| x.a + x.b + x.c[0] as u64 + x.d).sum();
        let elapsed = start.elapsed();
        println!("Vec<PartialBoxStruct> iterate+access: {:?}, sum={}", elapsed, sum);
    }
}

impl VecAccessBench {
    fn bench_access_struct() {
        let mut v = Vec::with_capacity(N);
        for i in 0..N {
            v.push(BigStruct { a: i as u64, b: i as u64, c: [0; 256], d: i as u64 });
        }
        let start = Instant::now();
        let mut sum = 0u64;
        for i in (0..N).step_by(1000) {
            sum += v[i].c[128] as u64;
        }
        let elapsed = start.elapsed();
        println!("Vec<BigStruct> random access large field: {:?}, sum={}", elapsed, sum);
    }

    fn bench_access_boxed_struct() {
        let mut v = Vec::with_capacity(N);
        for i in 0..N {
            v.push(Box::new(BigStruct { a: i as u64, b: i as u64, c: [0; 256], d: i as u64 }));
        }
        let start = Instant::now();
        let mut sum = 0u64;
        for i in (0..N).step_by(1000) {
            sum += v[i].c[128] as u64;
        }
        let elapsed = start.elapsed();
        println!("Vec<Box<BigStruct>> random access large field: {:?}, sum={}", elapsed, sum);
    }

    fn bench_access_partial_box() {
        let mut v = Vec::with_capacity(N);
        for i in 0..N {
            v.push(PartialBoxStruct {
                a: i as u64,
                b: i as u64,
                c: Box::new([0; 256]),
                d: i as u64,
            });
        }
        let start = Instant::now();
        let mut sum = 0u64;
        for i in (0..N).step_by(1000) {
            sum += v[i].c[128] as u64;
        }
        let elapsed = start.elapsed();
        println!("Vec<PartialBoxStruct> random access large field: {:?}, sum={}", elapsed, sum);
    }
}

impl SnipptBench for VecStructBench {
    fn run(&self) {
        Self::bench_vec_struct();
        Self::bench_vec_boxed_struct();
        Self::bench_vec_partial_box();
    }
    fn description(&self) -> &str {
        "Benchmark Vec with different struct layouts for iteration and field access performance (full iteration + access)"
    }
    fn name(&self) -> &str {
        "VecStructBench"
    }
}

impl SnipptBench for VecAccessBench {
    fn run(&self) {
        Self::bench_access_struct();
        Self::bench_access_boxed_struct();
        Self::bench_access_partial_box();
    }
    fn description(&self) -> &str {
        "Benchmark Vec with different struct layouts for random access of large field performance"
    }
    fn name(&self) -> &str {
        "VecAccessBench"
    }
}

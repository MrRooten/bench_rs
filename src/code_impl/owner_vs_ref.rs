use crate::code_impl::SnipptBench;
use std::hint::black_box;
use std::time::Instant;


pub struct Small([u8; 8]);

pub struct Medium([u8; 64]);

pub struct Large([u8; 256]);

pub struct VecStruct {
    data: Vec<u8>,
}

fn take_ownership<T>(x: T) -> T {
    black_box(x)
}
fn take_ref<T>(x: &T) {
    black_box(x);
    2;
}

pub struct OwnerVsRefBench;

impl SnipptBench for OwnerVsRefBench {
    fn run(&self) {
        const N: usize = 1000_000;
        // Allocate test objects outside timing
        let smalls: Vec<Small> = (0..N).map(|_| Small([1; 8])).collect();
        let mut smalls2: Vec<Small> = Vec::with_capacity(N);
        let mediums: Vec<Medium> = (0..N).map(|_| Medium([2; 64])).collect();
        let mut mediums2: Vec<Medium> = Vec::with_capacity(N);
        let larges: Vec<Large> = (0..N).map(|_| Large([3; 256])).collect();
        let mut larges2: Vec<Large> = Vec::with_capacity(N);

        // Ownership test: move each object
        let start = Instant::now();
        for s in smalls.into_iter().enumerate() {
            // acc ^= take_ownership(s);
            // smalls2[s.0] = take_ownership(s.1);
            smalls2.push(s.1);
        }
        println!("Small take_ownership: {:?}", start.elapsed());

        // Reference test: borrow each object
        let start = Instant::now();
        for s in &smalls2 {
            take_ref(s);
        }
        println!("Small take_ref: {:?}", start.elapsed());

        let start = Instant::now();
        for m in mediums.into_iter().enumerate() {
            // mediums2[m.0] = take_ownership(m.1);
            mediums2.push(m.1);
        }
        println!("Medium take_ownership: {:?}", start.elapsed());

        let start = Instant::now();
        for m in &mediums2 {
            take_ref(m);
        }
        println!("Medium take_ref: {:?}", start.elapsed());

        let start = Instant::now();
        for l in larges.into_iter().enumerate() {
            // larges2[l.0] = take_ownership(l.1);
            larges2.push(l.1);
        }
        println!("Large take_ownership: {:?}", start.elapsed());

        let start = Instant::now();
        for l in &larges2 {
            take_ref(l);
        }
        println!("Large take_ref: {:?}", start.elapsed());
    }
    fn description(&self) -> &str {
        "Compare performance of take ownership vs reference for different struct sizes."
    }
    fn name(&self) -> &str {
        "OwnerVsRefBench"
    }
}

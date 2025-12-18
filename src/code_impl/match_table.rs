use crate::code_impl::SnipptBench;

pub struct MatchTableBench;


use std::time::Instant;
use std::hint::black_box;

const ITER: usize = 30_000_000;
const OPCODE_COUNT: usize = 72;
#[inline(never)]
fn op0(x: usize) -> usize { x.wrapping_add(1) }
#[inline(never)]
fn op1(x: usize) -> usize { x.wrapping_add(2) }
#[inline(never)]
fn op2(x: usize) -> usize { x.wrapping_add(3) }
#[inline(never)]
fn op3(x: usize) -> usize { x.wrapping_add(4) }
#[inline(never)]
fn op4(x: usize) -> usize { x.wrapping_add(5) }
#[inline(never)]
fn op5(x: usize) -> usize { x.wrapping_add(6) }
#[inline(never)]
fn op6(x: usize) -> usize { x.wrapping_add(7) }
#[inline(never)]
fn op7(x: usize) -> usize { x.wrapping_add(8) }

fn match_dispatch(op: usize, acc: usize) -> usize {
    match op {
        0 => op0(acc),
        1 => op1(acc),
        2 => op2(acc),
        3 => op3(acc),
        4 => op4(acc),
        5 => op5(acc),
        6 => op6(acc),
        7 => op7(acc),
        8 => op0(acc),
        9 => op1(acc),
        10 => op2(acc),
        11 => op3(acc),
        12 => op4(acc),
        13 => op5(acc),
        14 => op6(acc),
        15 => op7(acc),
        16 => op0(acc),
        17 => op1(acc),
        18 => op2(acc),
        19 => op3(acc),
        20 => op4(acc),
        21 => op5(acc),
        22 => op6(acc),
        23 => op7(acc),
        24 => op0(acc),
        25 => op1(acc),
        26 => op2(acc),
        27 => op3(acc),
        28 => op4(acc),
        29 => op5(acc),
        30 => op6(acc),
        31 => op7(acc),
        32 => op0(acc),
        33 => op1(acc),
        34 => op2(acc),
        35 => op3(acc),
        36 => op4(acc),
        37 => op5(acc),
        38 => op6(acc),
        39 => op7(acc),
        40 => op0(acc),
        41 => op1(acc),
        42 => op2(acc),
        43 => op3(acc),
        44 => op4(acc),
        45 => op5(acc),
        46 => op6(acc),
        47 => op7(acc),
        // until to 72
        48 => op0(acc),
        49 => op1(acc),
        50 => op2(acc),
        51 => op3(acc),
        52 => op4(acc),
        53 => op5(acc),
        54 => op6(acc),
        55 => op7(acc),
        56 => op0(acc),
        57 => op1(acc),
        58 => op2(acc),
        59 => op3(acc),
        60 => op4(acc),
        61 => op5(acc),
        62 => op6(acc),
        63 => op7(acc),
        64 => op0(acc),
        65 => op1(acc),
        66 => op2(acc),
        67 => op3(acc),
        68 => op4(acc),
        69 => op5(acc),
        70 => op6(acc),
        71 => op7(acc),
        _ => unreachable!(),
    }
}

type OpFn = fn(usize) -> usize;

fn run() {
    let ops: [OpFn; OPCODE_COUNT] = [
        op0, op1, op2, op3, op4, op5, op6, op7,
        op0, op1, op2, op3, op4, op5, op6, op7,
        op0, op1, op2, op3, op4, op5, op6, op7,
        op0, op1, op2, op3, op4, op5, op6, op7,
        op0, op1, op2, op3, op4, op5, op6, op7,
        op0, op1, op2, op3, op4, op5, op6, op7,
        op0, op1, op2, op3, op4, op5, op6, op7,
        op0, op1, op2, op3, op4, op5, op6, op7,
        op0, op1, op2, op3, op4, op5, op6, op7
    ];

    // ---------- match ----------
    let mut acc = 0usize;
    let start = Instant::now();
    for i in 0..ITER {
        let op = i & (OPCODE_COUNT - 1);
        acc = match_dispatch(op, black_box(acc));
    }
    let match_time = start.elapsed();
    println!("match dispatch: {:?}, acc={}", match_time, acc);

    // ---------- table ----------
    let mut acc = 0usize;
    let start = Instant::now();
    for i in 0..ITER {
        let op = i & (OPCODE_COUNT - 1);
        let f = ops[op];
        acc = f(black_box(acc));
    }
    let table_time = start.elapsed();
    println!("table dispatch: {:?}, acc={}", table_time, acc);
}

impl SnipptBench for MatchTableBench {
    fn run(&self) {
        run();
    }
    
    fn description(&self) -> &str {
        "This is a benchmark for match table dispatch."
    }
    
    fn name(&self) -> &str {
        "MatchTableBench"
    }
}
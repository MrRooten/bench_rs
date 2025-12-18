// Test some call way overhead

pub struct CallOverHead;

use std::time::Instant;
use std::hint::black_box;

use crate::code_impl::SnipptBench;

const N: usize = 50_000_000;


#[inline(never)]
fn add_fn(a: i32) -> i32 {
    a + 1
}

#[inline(always)]
fn add_fn_inline(a: i32) -> i32 {
    a + 1
}

#[inline(never)]
fn match_dispatch(op: u8, a: i32) -> i32 {
    match op {
        0 => add_fn(a),
        _ => add_fn(a),
    }
}

type FnPtr = fn(i32) -> i32;

static FN_TABLE: [FnPtr; 1] = [
    add_fn,
];

#[inline(never)]
fn table_dispatch(a: i32) -> i32 {
    let f = FN_TABLE[0];
    f(a)
}

// ---------------- trait ----------------

trait Op {
    fn call(&self, a: i32) -> i32;
}

struct Add;

impl Op for Add {
    #[inline(never)]
    fn call(&self, a: i32) -> i32 {
        a + 1
    }
}

#[inline(never)]
fn dyn_trait_call(op: &dyn Op, a: i32) -> i32 {
    op.call(a)
}

#[inline(never)]
fn static_trait_call<T: Op>(op: &T, a: i32) -> i32 {
    op.call(a)
}

#[inline(never)]
fn closure_call(f: &dyn Fn(i32) -> i32, a: i32) -> i32 {
    f(a)
}

fn bench(name: &str, mut f: impl FnMut()) {
    let start = Instant::now();
    f();
    let elapsed = start.elapsed();
    println!("{:<25} {:?}", name, elapsed);
}


fn run() {
    let mut x = 0;
    let add = Add;
    let closure = |a| a + 1;

    bench("direct fn", || {
        for _ in 0..N {
            x = black_box(add_fn(x));
        }
    });

    bench("inline fn", || {
        for _ in 0..N {
            x = black_box(add_fn_inline(x));
        }
    });

    bench("match dispatch", || {
        for _ in 0..N {
            x = black_box(match_dispatch(0, x));
        }
    });

    bench("fn table", || {
        for _ in 0..N {
            x = black_box(table_dispatch(x));
        }
    });

    bench("dyn trait", || {
        for _ in 0..N {
            x = black_box(dyn_trait_call(&add, x));
        }
    });

    bench("static trait", || {
        for _ in 0..N {
            x = black_box(static_trait_call(&add, x));
        }
    });

    bench("closure", || {
        for _ in 0..N {
            x = black_box(closure_call(&closure, x));
        }
    });

    // 防止被优化掉
    println!("final x = {}", x);
}


impl SnipptBench for CallOverHead {
    fn run(&self) {
        run();
    }
    
    fn description(&self) -> &str {
        "This is a benchmark for function call overhead."
    }
    
    fn name(&self) -> &str {
        "CallOverHead"
    }
}
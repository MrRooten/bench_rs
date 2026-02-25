use crate::code_impl::SnipptBench;
use std::hint::black_box;
use std::time::Instant;

type ObjId = usize;

pub struct SelfStack<const N: usize> {
    stack: Box<[ObjId;N]>,
    top: usize,
}

impl<const N: usize> SelfStack<N> {
    pub fn new() -> SelfStack<N> {
        SelfStack {
            stack: Box::new([0;N]),
            top: 0,
        }
    }
    
    pub fn push(&mut self, id: ObjId) {
        self.stack[self.top] = id;
        self.top += 1;
    }

    pub fn pop(&mut self) -> Option<ObjId> {
        if self.top == 0 {
            None
        } else {
            self.top -= 1;
            Some(self.stack[self.top])
        }
    }

    pub fn pop_unchecked(&mut self) -> ObjId {
        self.top -= 1;
        self.stack[self.top]
    }
}


pub struct StackBench;

impl SnipptBench for StackBench {
	fn run(&self) {
		const N: usize = 1024;
		const REPS: usize = 200_000;

		let start = Instant::now();
		let mut acc_vec = 0usize;
		for _ in 0..REPS {
			let mut stack: Vec<usize> = Vec::with_capacity(N);
			for i in 0..N {
				stack.push(black_box(i));
			}
			while let Some(v) = stack.pop() {
				acc_vec = acc_vec.wrapping_add(black_box(v));
			}
			black_box(&mut stack);
		}
		black_box(acc_vec);
		println!("Vec<usize> stack: {:?}", start.elapsed());

		let start = Instant::now();
		let mut acc_stack = 0usize;
		for _ in 0..REPS {
			let mut stack: [usize; N] = [0; N];
			let mut top = 0usize;
			for i in 0..N {
				stack[top] = black_box(i);
				top += 1;
			}
			while top > 0 {
				top -= 1;
				acc_stack = acc_stack.wrapping_add(black_box(stack[top]));
			}
			black_box(&mut stack);
			black_box(top);
		}
		black_box(acc_stack);
		println!("[usize; N] stack: {:?}", start.elapsed());

		let start = Instant::now();
		let mut acc_heap = 0usize;
		for _ in 0..REPS {
			let mut stack = Box::new([0usize; N]);
			let mut top = 0usize;
			for i in 0..N {
				stack[top] = black_box(i);
				top += 1;
			}
			while top > 0 {
				top -= 1;
				acc_heap = acc_heap.wrapping_add(black_box(stack[top]));
			}
			black_box(&mut stack);
			black_box(top);
		}
		black_box(acc_heap);
		println!("Box<[usize; N]> stack: {:?}", start.elapsed());


        let start = Instant::now();
        let mut acc_self_stack = 0usize;
        for _ in 0..REPS {
            let mut stack = SelfStack::<N>::new();
            for i in 0..N {
                stack.push(black_box(i));
            }
            while stack.top > 0 {
                let v = stack.pop_unchecked();
                acc_self_stack = acc_self_stack.wrapping_add(black_box(v));
            }
            black_box(&mut stack);
        }
        black_box(acc_self_stack);
        println!("SelfStack stack: {:?}", start.elapsed());
	}

	fn description(&self) -> &str {
		"Benchmark Vec stack vs stack array vs heap array for push/pop."
	}

	fn name(&self) -> &str {
		"StackBench"
	}
}

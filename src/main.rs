use bench_rs::code_impl::{SnipptBench, cache_vs_no::CacheVsNoBench, fn_call::CallOverHead, is_empty::IsEmptyBench, list_vs_ptr::ListVsPtrBench, match_table::MatchTableBench, nonzero::NonZeroBench, owner_vs_ref, result_or_not::ResultOrNotBench, slice_vs_ref::SliceVsRefBench, slice_vs_vec, stack::StackBench, unwrap_vs_unchecked, vec_size::{VecAccessBench, VecStructBench}, while_vs_let::WhileVsLetBench};



pub fn main() {
    let mut benches: Vec<Box<dyn SnipptBench>> = vec![];
    let match_table: Box<dyn SnipptBench> = Box::new(MatchTableBench {});
    benches.push(match_table);
    benches.push(Box::new(CallOverHead {}));
    benches.push(Box::new(CacheVsNoBench {}));
    benches.push(Box::new(VecStructBench {}));
    benches.push(Box::new(VecAccessBench {}));
    benches.push(Box::new(ListVsPtrBench {}));
    benches.push(Box::new(ResultOrNotBench {}));
    benches.push(Box::new(NonZeroBench {}));
    benches.push(Box::new(IsEmptyBench {}));
    benches.push(Box::new(StackBench {}));
    benches.push(Box::new(SliceVsRefBench {}));
    benches.push(Box::new(unwrap_vs_unchecked::UnwrapVsUncheckedBench {}));
    benches.push(Box::new(owner_vs_ref::OwnerVsRefBench {}));
    benches.push(Box::new(slice_vs_vec::SliceVsVecGetBench {}));
    benches.push(Box::new(WhileVsLetBench {}));
    for bench in benches.iter() {
        println!("Running benchmark: {}", bench.description());
        bench.run();
        println!("-----------------------------")
    }
}
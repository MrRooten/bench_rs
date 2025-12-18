use bench_rs::code_impl::{SnipptBench, cache_vs_no::CacheVsNoBench, fn_call::CallOverHead, match_table::MatchTableBench};



pub fn main() {
    let mut benches: Vec<Box<dyn SnipptBench>> = vec![];
    let match_table: Box<dyn SnipptBench> = Box::new(MatchTableBench {});
    benches.push(match_table);
    benches.push(Box::new(CallOverHead {}));
    benches.push(Box::new(CacheVsNoBench {}));

    for bench in benches.iter() {
        println!("Running benchmark: {}", bench.description());
        bench.run();
        println!("-----------------------------")
    }
}
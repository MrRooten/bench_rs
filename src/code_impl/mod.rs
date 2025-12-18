pub mod match_table;
pub mod fn_call;
pub mod cache_vs_no;
pub trait SnipptBench {
    fn run(&self);

    fn description(&self) -> &str;

    fn name(&self) -> &str;
}
pub mod match_table;
pub mod fn_call;
pub trait SnipptBench {
    fn run(&self);

    fn description(&self) -> &str;

    fn name(&self) -> &str;
}
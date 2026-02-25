pub mod unwrap_vs_unchecked;
pub mod match_table;
pub mod fn_call;
pub mod cache_vs_no;
pub mod vec_size;
pub mod list_vs_ptr;
pub mod result_or_not;
pub mod nonzero;
pub mod is_empty;
pub mod stack;
pub mod slice_vs_ref;
pub mod owner_vs_ref;
pub mod slice_vs_vec;
pub mod while_vs_let;
pub trait SnipptBench {
    fn run(&self);

    fn description(&self) -> &str;

    fn name(&self) -> &str;
}
use super::*;

#[allow(unused)]
use crate::{
    frame_system::RawOrigin,
    mock::{new_test_ext, Test},
    Pallet,
};
use frame::deps::frame_benchmarking::v2::*;

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn some_benchmark() {}

    impl_benchmark_test_suite!(Pallet, new_test_ext(), Test);
}

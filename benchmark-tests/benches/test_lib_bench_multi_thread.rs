use std::hint::black_box;

use benchmark_tests::bubble_sort;
use iai_callgrind::{library_benchmark, library_benchmark_group, main, LibraryBenchmarkConfig};

fn setup_worst_case_array(start: i32) -> Vec<i32> {
    if start.is_negative() {
        (start..0).rev().collect()
    } else {
        (0..start).rev().collect()
    }
}

#[library_benchmark]
#[bench::worst_case_20(setup_worst_case_array(20))]
fn bench_with_thread(array: Vec<i32>) -> Vec<i32> {
    std::thread::spawn(|| black_box(bubble_sort(array)))
        .join()
        .unwrap()
}

#[library_benchmark]
#[bench::worst_case_20(setup_worst_case_array(20))]
fn bench_without_thread(array: Vec<i32>) -> Vec<i32> {
    black_box(bubble_sort(array))
}

library_benchmark_group!(
    name = bench_group;
    benchmarks = bench_with_thread, bench_without_thread
);

main!(
    config = LibraryBenchmarkConfig::default();
    library_benchmark_groups = bench_group
);

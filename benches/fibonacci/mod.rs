use criterion::{black_box, criterion_group, Criterion};
use criterion::{BenchmarkId};
use criterion_cycles_per_byte::CyclesPerByte;

use::libhermit_rs_bench::fibonacci;

/**
 * Simple benchmark
 */
pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci::as_recursive(black_box(20))));
}
criterion_group!(simple_bench, criterion_benchmark);

/**
 * Compare two functions and measure cycles, not time!
 */
pub fn bench(c: &mut Criterion<CyclesPerByte>) {
    let mut group = c.benchmark_group("fibonacci");

    for i in 0..20 {
        group.bench_function(BenchmarkId::new("recursive", i), |b| b.iter(|| fibonacci::as_recursive(black_box(i))));
        group.bench_function(BenchmarkId::new("loop", i), |b| b.iter(|| fibonacci::as_loop(black_box(i))));
    }

    group.finish()
}
criterion_group!(
    name = cycle_bench;
    config = Criterion::default().with_measurement(CyclesPerByte);
    targets = bench
);

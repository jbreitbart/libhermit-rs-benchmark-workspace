# libhermit-rs-benchmark-workspace

This a prototype workspace on how [Criterion.rs](https://github.com/bheisler/criterion.rs) can be used to benchmark function from [libhermit-rs](https://github.com/hermitcore/libhermit-rs). To run benchmarks and generate a report just clone the repository and call `cargo bench`. The generate report will be available in `target/criterion/report`.

Currently there are 3 benchmark groups available

* _fib 20_: It just benchmarks the function call `fibonacci::as_recursive(20)`.
* _fibonacci_: This compares the two functions `fibonacci::as_recursive` and `fibonacci::as_loop` for various input sizes and measures the performance in cycles not seconds.
* _memcpy throughput_ compares the libhermit-rs `memcpy` implementation with the one of `libc` and `std::ptr::copy_nonoverlapping`. It measure the time as well as the throughput in bytes.

The primary goal of [Criterion.rs](https://github.com/bheisler/criterion.rs) is to provide:

* a powerful and statistically rigorous tool for measuring the performance of code.
* preventing performance regressions by comparing the current performance with previous runs.
* accurately measuring optimizations.

use criterion::{criterion_main};

mod fibonacci;
use crate::fibonacci::{simple_bench, cycle_bench};

mod mem;
use crate::mem::memcpy_libc_cmp;

// Add the benchmark groups that should be run
criterion_main!(simple_bench, cycle_bench, memcpy_libc_cmp);

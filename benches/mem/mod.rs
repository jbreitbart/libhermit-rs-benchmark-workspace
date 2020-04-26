use criterion::BenchmarkId;
use criterion::{black_box, criterion_group, AxisScale, Criterion, PlotConfiguration, Throughput};

use std::time::Duration;

use libhermit_rs_bench::mem;

pub fn bench_memcpy(c: &mut Criterion) {
    let size = vec![1, 64, 512, 4096, 4097, 2097152, 2097152 * 16];

    let src = vec![8; *size.last().unwrap()];
    let mut dst = vec![0; *size.last().unwrap()];

    let mut group = c.benchmark_group("memcpy throughput");

    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);
    group.plot_config(plot_config);

    for s in size {
        group.throughput(Throughput::Bytes(s as u64));
        if s >= 33554432 {
            group.measurement_time(Duration::new(25, 0));
        }

        group.bench_with_input(BenchmarkId::new("rustyhermit", s), &s, |b, s| {
            b.iter(|| unsafe {
                let _ = mem::cpy(
                    black_box(dst.as_mut_ptr()),
                    black_box(src.as_ptr()),
                    black_box(*s),
                );
            });
        });

        group.bench_with_input(
            BenchmarkId::new("rust copy_nonoverlapping", s),
            &s,
            |b, s| {
                b.iter(|| unsafe {
                    let _ = std::ptr::copy_nonoverlapping(
                        black_box(src.as_ptr()),
                        black_box(dst.as_mut_ptr()),
                        black_box(*s),
                    );
                });
            },
        );

        group.bench_with_input(BenchmarkId::new("libc", s), &s, |b, s| {
            b.iter(|| unsafe {
                let dst = dst.as_mut_ptr() as *mut libc::c_void;
                let src = src.as_ptr() as *const libc::c_void;
                let _ = libc::memcpy(black_box(dst), black_box(src), black_box(*s));
            });
        });
    }

    group.finish();
}

criterion_group!(memcpy_libc_cmp, bench_memcpy);

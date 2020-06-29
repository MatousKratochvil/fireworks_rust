use criterion::{black_box, Criterion};
use fireworks::*;


pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci::fibonacci(black_box(20))));
}
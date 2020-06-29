use criterion::{criterion_group, criterion_main};

mod fibonacci_benchmark;

criterion_group!(benches, fibonacci_benchmark::criterion_benchmark);
criterion_main!(benches);
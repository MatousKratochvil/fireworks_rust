use criterion::{criterion_group, criterion_main};

mod fibonacci_benchmark;

criterion_group!(benches, fibonacci_benchmark::fibonnaci_bench);
criterion_main!(benches);
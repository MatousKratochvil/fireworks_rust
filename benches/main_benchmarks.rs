use criterion::{black_box, Criterion};
use criterion::{criterion_group, criterion_main};

use fireworks::collection::*;

pub fn collection_benchmark(c: &mut Criterion) {
	let mut collection = Collection::<u64>::new();

	let mut col_empty = c.benchmark_group("Collection_Empty");
	col_empty.bench_function("Accept", |b| {
		b.iter(|| {
			collection.accept(|&x| {
				black_box(x);
				()
			})
		})
	});
	col_empty.bench_function("Accept_Mutable", |b| {
		b.iter(|| collection.accept_mut(|&mut x| black_box(x)))
	});
	col_empty.finish();

	for n in 0..100 {
		collection.attach(n);
	}

	let mut col_hundread_items = c.benchmark_group("Collection_100_Items");
	col_hundread_items.bench_function("Accept", |b| {
		b.iter(|| {
			collection.accept(|&x| {
				black_box(x);
				()
			})
		})
	});
	col_hundread_items.bench_function("Accept_Mutable", |b| {
		b.iter(|| collection.accept_mut(|&mut x| black_box(x)))
	});
	col_hundread_items.finish();
}

criterion_group!(benches, collection_benchmark);
criterion_main!(benches);

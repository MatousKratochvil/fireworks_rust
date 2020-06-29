use criterion::{black_box, Criterion};
use fireworks::*;

macro_rules! benchmark {
	($func_name:ident, $name:literal, $fn_impl:expr) => {
		pub fn $func_name(c: &mut Criterion) {
			c.bench_function($name, |b| b.iter($fn_impl));
		}
	};
}


benchmark!(fibonnaci_bench, "fib 20", || fibonacci::fibonacci(black_box(80)));
use std::env;

use corrector::Corrector;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

#[cfg(not(target_env = "msvc"))]
use tikv_jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn criterion_benchmark(c: &mut Criterion) {
  let mut crtr = Corrector::new();
  let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
  let big = manifest_dir + "/big.txt";
  crtr.load(big).unwrap();

  let long_word = "korrectud".to_string();
  c.bench_function("correct", |b| {
    b.iter(|| crtr.correct(black_box(&long_word)))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

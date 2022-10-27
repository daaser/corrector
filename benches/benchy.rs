use corrector::Corrector;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  let crtr = Corrector::new();
  let long_word = "korrectud".to_string();
  c.bench_function("correct", |b| {
    b.iter(|| crtr.correct(black_box(&long_word)))
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

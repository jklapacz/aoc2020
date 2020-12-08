use criterion::{criterion_group, criterion_main, Criterion};
use aoc::day03;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("day 03", |b| b.iter(|| day03::call("./inputs/03.yaml")));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

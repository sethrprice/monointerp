use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_interpolation::monointerp;

pub fn criterion_benchmark(c: &mut Criterion) {
    let x: Vec<f32> = (0..200000).map(|val| val as f32).collect();
    let y: Vec<f32> = x.iter().map(|val| val.powf(2.0) + 3. * val + 1.).collect();
    let query: Vec<f32> = x.iter().map(|val| val + 0.5).take(x.len() - 1).collect();
    c.bench_function("monointerp 200,000", |b| {
        b.iter(|| monointerp(black_box(&query), black_box(&x), black_box(&y)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

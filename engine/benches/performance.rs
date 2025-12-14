use criterion::{criterion_group, criterion_main, Criterion};
use quantum_engine::compute;

fn bench_sum_operations(c: &mut Criterion) {
    let data: Vec<f64> = (0..10000).map(|x| x as f64).collect();
    
    c.bench_function("scalar_sum_10000", |b| {
        b.iter(|| {
            let sum: f64 = data.iter().sum();
            criterion::black_box(sum);
        })
    });
    
    c.bench_function("optimized_sum_10000", |b| {
        b.iter(|| {
            let sum = compute::optimized_sum(&data);
            criterion::black_box(sum);
        })
    });
}

criterion_group!(benches, bench_sum_operations);
criterion_main!(benches);
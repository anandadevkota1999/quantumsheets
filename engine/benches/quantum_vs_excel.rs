use criterion::{criterion_group, criterion_main, Criterion};
use quantum_engine::{compute, QuantumColumn};

fn bench_sum_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sum Operations");
    
    let sizes = [100, 1000, 10000, 100000];
    
    for &size in &sizes {
        let data: Vec<f64> = (0..size).map(|x| x as f64).collect();
        
        group.bench_function(&format!("scalar_sum_{}", size), |b| {
            b.iter(|| {
                let sum: f64 = data.iter().sum();
                criterion::black_box(sum);
            })
        });
        
        group.bench_function(&format!("optimized_sum_{}", size), |b| {
            b.iter(|| {
                let sum = compute::optimized_sum(&data);
                criterion::black_box(sum);
            })
        });
    }
    
    group.finish();
}

criterion_group!(benches, bench_sum_operations);
criterion_main!(benches);
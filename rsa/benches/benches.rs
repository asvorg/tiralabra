use criterion::{criterion_group, criterion_main, Criterion};
use rsa::prime_func::Prime_func; 

fn generate_prime_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_prime");
    let bit_sizes = [128,256,512, 1024, 2048, 4096];

    for &bits in bit_sizes.iter() {
        group.bench_function(format!("{} bits", bits), |b| {
            b.iter(|| {
                Prime_func::generate_prime(bits);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, generate_prime_benchmark);
criterion_main!(benches);

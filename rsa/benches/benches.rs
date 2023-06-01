use criterion::{criterion_group, criterion_main, Criterion};
use num_bigint::{BigUint, ToBigUint};
use rsa::prime_func::Prime_func;
use rsa::keygen::Keygen;

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

//benchmark the calculate_d function
fn calculate_d_benchmark(c: &mut Criterion) {
    let mut group: criterion::BenchmarkGroup<criterion::measurement::WallTime> = c.benchmark_group("calculate_d");
    let bit_sizes: [i32; 6] = [128,256,512, 1024, 2048, 4096];

    for &bits in bit_sizes.iter() {
        group.bench_function(format!("{} bits", bits), |b: &mut criterion::Bencher| {
            b.iter(|| {
                let e: BigUint = 65537.to_biguint().unwrap();
                let phi: BigUint = 3723232.to_biguint().unwrap();
                Keygen::calculate_d(e.clone(), phi.clone());
            });
        });
    }

    group.finish();
}

criterion_group!(benches, generate_prime_benchmark);
criterion_main!(benches);

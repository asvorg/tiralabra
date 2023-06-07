use criterion::{criterion_group, criterion_main, Criterion};
use num_bigint::{BigUint, ToBigUint};
use rsa::prime_func::PrimeFunc;
use rsa::keygen::Keygen;

//benchmark the generate prime function
fn generate_prime_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("generate_prime");
    let bit_sizes = [128,256,512, 1024, 2048, 4096];

    for &bits in bit_sizes.iter() {
        group.bench_function(format!("{} bits", bits), |b| {
            b.iter(|| {
                PrimeFunc::generate_prime(bits);
            });
        });
    }

    group.finish();
}

//benchmark the extended euclidean algorithm
fn extended_euclidean_algorithm_benchmark(c: &mut Criterion) {
    let mut group: criterion::BenchmarkGroup<criterion::measurement::WallTime> = c.benchmark_group("calculate_d");
    let bit_sizes: [i32; 6] = [128,256,512, 1024, 2048, 4096];

    for &bits in bit_sizes.iter() {
        group.bench_function(format!("{} bits", bits), |b: &mut criterion::Bencher| {
            b.iter(|| {
                let e: BigUint = 65537.to_biguint().unwrap();
                let phi: BigUint = 3723232.to_biguint().unwrap();
                Keygen::extended_euclidean_algorithm(e.clone(), phi.clone());
            });
        });
    }

    group.finish();
}

//benchmark the gcd function
fn gcd_benchmark(c: &mut Criterion) {
    let mut group: criterion::BenchmarkGroup<criterion::measurement::WallTime> = c.benchmark_group("gcd");
    let bit_sizes: [i32; 7] = [128,256,512, 1024, 2048, 4096,8192];

    for &bits in bit_sizes.iter() {
        group.bench_function(format!("{} bits", bits), |b: &mut criterion::Bencher| {
            b.iter(|| {
                let e: BigUint = 65537.to_biguint().unwrap();
                let phi: BigUint = 3723232.to_biguint().unwrap();
                Keygen::gcd(e.clone(), phi.clone());
            });
        });
    }

    group.finish();
}

criterion_group!(benches, gcd_benchmark);
criterion_main!(benches);

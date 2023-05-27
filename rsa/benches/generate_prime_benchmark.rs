use prime_func::generate_prime;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn generate_prime_benchmark(c: &mut Criterion) {
    c.bench_function("generate_prime", |b| b.iter(|| generate_prime(100)));
}

criterion_group!(
    name = benches;
    config = Criterion::default().with_measurement_time(std::time::Duration::from_secs(5));
    targets = generate_prime_benchmark
);

criterion_main!(benches);

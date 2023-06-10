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
    let mut group: criterion::BenchmarkGroup<criterion::measurement::WallTime> = c.benchmark_group("extended_euclidean_algorithm");
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

//benchmark the encryption and decryption functions
fn encrypt_and_decrypt_benchmark(c: &mut Criterion) {
    let mut group: criterion::BenchmarkGroup<criterion::measurement::WallTime> = c.benchmark_group("encrypt_and_decrypt");
    let bit_sizes: [i32; 6] = [128,256,512, 1024, 2048, 4096];

    for &bits in bit_sizes.iter() {
        group.bench_function(format!("{} bits", bits), |b: &mut criterion::Bencher| {
            b.iter(|| {
                let ((n, d), (n_2, e),(_p,_q)) = Keygen::keygen(bits as u64);
                let message: String = String::from("This is a test message");
                let message_uint: BigUint = rsa::encryption::Encrypt::convert_text_to_int(&message);
                let message_uint_encrypted: BigUint = rsa::encryption::Encrypt::encrypt(message_uint.clone(),n,e);
                let message_uint_decrypted: BigUint = rsa::decryption::Decrpypt::decrypt(message_uint_encrypted, n_2, d);
                let _message_decrypted: String = rsa::decryption::Decrpypt::convert_int_to_text(&message_uint_decrypted);
            });
        });
    }

    group.finish();
}

criterion_group!(benches, encrypt_and_decrypt_benchmark);
criterion_main!(benches);

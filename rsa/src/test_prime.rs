use rand::Rng;
use num_bigint::{BigUint, RandBigInt, ToBigUint};

fn generate_large_prime(bit_size: u32) -> BigUint {
    let mut rng = rand::thread_rng();

    loop {
        let candidate = rng.gen_biguint(bit_size);
        if is_prime(&candidate, 10) {
            return candidate;
        }
    }
}

fn is_prime(n: &BigUint, num_rounds: u32) -> bool {
    if n == &2u32.to_biguint().unwrap() {
        return true;
    }

    if n.is_even() || n < &2u32.to_biguint().unwrap() {
        return false;
    }

    let (mut d, mut r) = decompose(n - 1u32.to_biguint().unwrap());

    for _ in 0..num_rounds {
        let a = rng.gen_biguint_range(&2u32.to_biguint().unwrap(), n);

        let mut x = a.modpow(&d, n);

        if x == 1u32.to_biguint().unwrap() || x == n - 1u32.to_biguint().unwrap() {
            continue;
        }

        for _ in 0..(r - 1) {
            x = x.modpow(&2u32.to_biguint().unwrap(), n);
            if x == n - 1u32.to_biguint().unwrap() {
                break;
            }
        }

        if x != n - 1u32.to_biguint().unwrap() {
            return false;
        }
    }

    true
}

fn decompose(n: &BigUint) -> (BigUint, u32) {
    let mut d = n - 1u32.to_biguint().unwrap();
    let mut r = 0u32;

    while d.is_even() {
        d >>= 1;
        r += 1;
    }

    (d, r)
}

fn main() {
    let prime = generate_large_prime(1024);
    println!("Generated prime: {}", prime);
}

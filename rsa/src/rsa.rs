use crate::prime_func;
use num_bigint::BigUint;
use num_traits::{One,Zero};

pub fn keygen() -> ((BigUint, BigUint), (BigUint, BigUint)){
    let p: &BigUint = prime_func::generate_prime(50);
    let q: &BigUint = prime_func::generate_prime(50);
    let n: BigUint = p * q;
    let z: BigUint = (p - 1u32) * (q - 1u32);
    let e: BigUint = BigUint::from(65537u64);

    let minus = BigUint::one() - BigUint::one();
    let d = e.modpow(&minus, &z);
    return ((n.clone(),d),(n.clone(),e))
}

use num_bigint::BigUint;
use num_traits::{One, Zero,ToPrimitive,FromPrimitive};

use crate::prime_func::Prime_func;

pub struct Keygen;
impl Keygen{

pub fn keygen() -> ((BigUint, BigUint), (BigUint, BigUint)){ //alculate the keys for the encryption and decryption functions
    let p: &BigUint = Prime_func::generate_prime(1024);
    let q: &BigUint = Prime_func::generate_prime(1024);
    let n: BigUint = p * q;
    let z: BigUint = (p - 1u32) * (q - 1u32);
    let e: BigUint = BigUint::from(65537u64);

    let p_minus_one: BigUint = p - BigUint::one();
    let q_minus_one: BigUint = q - BigUint::one();
    let phi: BigUint = &p_minus_one * &q_minus_one;

    let d: BigUint = e.modpow(&(e.clone() - BigUint::one()), &phi);

    return ((n.clone(),d),(n.clone(),e))
    }
}

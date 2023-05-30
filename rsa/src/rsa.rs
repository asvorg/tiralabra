use crate::prime_func;
use std::u128;
use std::option::Option;
use num_bigint::BigUint;
use num_traits::{One, Zero,ToPrimitive,FromPrimitive};
use num_integer::Integer;


pub fn keygen() -> ((BigUint, BigUint), (BigUint, BigUint)){
    let p: &BigUint = prime_func::generate_prime(1024);
    let q: &BigUint = prime_func::generate_prime(1024);
    let n: BigUint = p * q;
    let z: BigUint = (p - 1u32) * (q - 1u32);
    let e: BigUint = BigUint::from(65537u64);

    let p_minus_one: BigUint = p - BigUint::one();
    let q_minus_one: BigUint = q - BigUint::one();
    let phi: BigUint = &p_minus_one * &q_minus_one;

    let d_option: Option<BigUint> = mod_inverse(&e, &phi);
    
    let d: BigUint = d_option.unwrap_or_else(|| BigUint::zero());

    return ((n.clone(),d),(n.clone(),e))
}

fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
    let (gcd, x, _) = a.extended_gcd(m);

    if gcd.is_one() {
        Some((x % m + m) % m)
    } else {
        None
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keygen() {
        let ((n, d), (_, e)) = keygen();

        // Assert that n, d, and e are not zero
        assert_ne!(n, BigUint::zero());
        assert_ne!(d, BigUint::zero());
        assert_ne!(e, BigUint::zero());

        // Assert that n, d, and e are not equal
        assert_ne!(n, d);
        assert_ne!(n, e);
        assert_ne!(d, e);

        // Assert that d is the modular inverse of e
        let p: &BigUint = prime_func::generate_prime(1024);
        let q: &BigUint = prime_func::generate_prime(1024);
        let p_minus_one: BigUint = p - BigUint::one();
        let q_minus_one: BigUint = q - BigUint::one();
        let phi: BigUint = &p_minus_one * &q_minus_one;
        let d_expected: BigUint = e.clone().modpow(&BigUint::one(), &phi);
        assert_eq!(d, d_expected);
    }
}
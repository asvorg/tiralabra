use crate::prime_func;
use num_bigint::BigUint;
use num_traits::{One,Zero};

pub fn keygen() -> ((BigUint, BigUint), (BigUint, BigUint)){
    let p: &BigUint = prime_func::generate_prime(1024);
    let q: &BigUint = prime_func::generate_prime(1024);
    let n: BigUint = p * q;
    let z: BigUint = (p - 1u32) * (q - 1u32);
    let e: BigUint = BigUint::from(65537u64);

    let d: BigUint = calculate_d(&e, p, q);
    return ((n.clone(),d),(n.clone(),e))
}


fn calculate_d(e: &BigUint, p: &BigUint, q: &BigUint) -> BigUint {
    let p_minus_one: BigUint = p - 1u32;
    let q_minus_one: BigUint = q - 1u32;
    let phi: BigUint = &p_minus_one * &q_minus_one;
    
    e.clone().modpow(&BigUint::one(), &phi)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keygen() {
        let ((public_n, public_d), (public_n2, public_e)) = keygen();

        // Check if n is a product of two prime numbers
        assert!(prime_func::is_prime(&(public_n.clone() / &public_d)));
        assert!(prime_func::is_prime(&(public_n2.clone() / &public_e)));

        // Check if e and d are multiplicative inverses modulo z
        let p: &BigUint = &(public_n / &public_d);
        let q: &BigUint = &(public_n2 / &public_e);
        let z: BigUint = (p - 1u32) * (q - 1u32);
        let minus = BigUint::one() - BigUint::one();
        let d = public_e.modpow(&minus, &z);
        assert_eq!(d, public_d);
    }
}
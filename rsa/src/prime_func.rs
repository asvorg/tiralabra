use num_traits::{One, Zero};
use num_bigint::{BigUint, ToBigUint, RandBigInt};

pub fn get_candidate() -> BigUint {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let mut candidate: BigUint = rng.gen_biguint(1024);
    if &candidate % &BigUint::from(2u32) == BigUint::zero() {
        candidate += BigUint::one();
    }
    candidate
}

pub fn miller_rabin(candidate: BigUint) -> bool {
    let mut a: BigUint = 2u32.to_biguint().unwrap();
    let mut b: BigUint = a.modpow(&(candidate.clone() - BigUint::one()), &candidate.clone());

    for i in 1..=20 {
        let exponent = 2u32.pow(i - 1);
        a = b.clone().modpow(&(exponent.to_biguint().unwrap()), &candidate.clone());
        b = a.modpow(&BigUint::from(2u32), &candidate.clone());

        if a != BigUint::one() && a != candidate.clone() - BigUint::one() && b == BigUint::one() {
            return true;
        }
    }

    false
}

    pub fn low_level_primality(candidate: BigUint) -> bool{
        let small_primes: [u32; 13] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41];
        for prime in small_primes {
            if candidate.clone() % prime.to_biguint().unwrap() == BigUint::zero() {
                return false;
            }
        }
        true
    }

    pub fn is_prime(n: &BigUint) -> bool {
        if n.is_zero() || n == &BigUint::one() {
            return false;
        }
    
        let sqrt_n = n.sqrt();
        let two = BigUint::from(2u32);
    
        // Check divisibility by odd numbers up to sqrt(n)
        let mut i = BigUint::from(3u32);
        while &i <= &sqrt_n {
            if n % &i == BigUint::zero() {
                return false;
            }
            i += &two;
        }
    
        true
    }
    
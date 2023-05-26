use num_traits::{One, Zero};
use num_bigint::{BigUint, ToBigUint, RandBigInt};

pub fn get_candidate() -> BigUint {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let mut candidate: BigUint = rng.gen_biguint(100);
    if &candidate % &BigUint::from(2u32) == BigUint::zero() {
        candidate += BigUint::one();
    }
    candidate
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
    
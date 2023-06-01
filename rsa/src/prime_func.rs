use num_traits::{One, Zero};
use num_bigint::{BigUint, ToBigUint, RandBigInt};

pub struct PrimeFunc;
impl PrimeFunc {
    
pub fn get_candidate(n: u64) -> BigUint { //Randomly get a candidate number to be a possible prime
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let mut candidate: BigUint = rng.gen_biguint(n);
    if &candidate % &BigUint::from(2u32) == BigUint::zero() {
        candidate += BigUint::one();
    }
    candidate
}


pub fn is_prime(n: &BigUint) -> bool {// Test if the number is prime WITH CERTAINITY, only used for tests
    if n.is_zero() || n == &BigUint::one() {
        return false;
    }

    let sqrt_n: BigUint = n.sqrt();
    let two: BigUint = BigUint::from(2u32);

    let mut i: BigUint = BigUint::from(3u32);
    while &i <= &sqrt_n {
        if n % &i == BigUint::zero() {
            return false;
        }
        i += &two;
    }

    true
}

pub fn low_level_primality(candidate: &BigUint) -> bool{// Test low level primality, using this it is possible to discard the random numbers faster
    let small_primes: [u32; 11] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    for prime in small_primes {
        if candidate.clone() % prime.to_biguint().unwrap() == BigUint::zero() {
            return false;
        }
    }
    true
}


pub fn miller_rabin(n: &BigUint) -> bool {//Miller Rabin primality test
    let k: u32 = 50;
    if n <= &BigUint::one() {
        return false;
    }
    let mut d: BigUint = n - BigUint::one();
    let mut r: i32 = 0;
    while &d % &BigUint::from(2u32) == BigUint::zero() {
        d >>= 1;
        r += 1;
    }
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    for _ in 0..k {
        let a: BigUint = rng.gen_biguint_range(&BigUint::from(2u32), &n);
        let mut x: BigUint = a.modpow(&d, &n);
        if x == BigUint::one() || x == n - BigUint::one() {
            continue;
        }
        let mut i: i32 = 0;
        while i < r - 1 {
            x = x.modpow(&BigUint::from(2u32), &n);
            if x == BigUint::one() {
                return false;
            }
            if x == n - BigUint::one() {
                break;
            }
            i += 1;
        }
        if i == r - 1 {
            return false;
        }
    }
    true
}

pub fn generate_prime(n: u64) -> &'static BigUint { //Generate prime number using the other functions, note that the number is not a prime with certainity, only with a high probability
    loop {
        let candidate: BigUint = Self::get_candidate(n);
            if Self::low_level_primality(&candidate){
                if Self::miller_rabin(&candidate) {
                    return Box::leak(Box::new(candidate));
                }
            }
    }
}

}
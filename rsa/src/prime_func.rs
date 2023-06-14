//functions to generate prime numbers, test if a number is prime, and other functions related to prime numbers
use num_integer::Integer;
use num_traits::{One, Zero};
use num_bigint::{BigUint, ToBigUint, RandBigInt};

pub struct PrimeFunc;
impl PrimeFunc {
    
//Randomly get a candidate number to be a possible prime
pub fn get_candidate(n: u64) -> BigUint {
    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
    let mut candidate: BigUint = rng.gen_biguint(n);
    if &candidate % &BigUint::from(2u32) == BigUint::zero() {
        candidate += BigUint::one();
    }
    candidate
}

// Test if the number is prime WITH CERTAINITY, only used for tests
#[allow(dead_code)]
pub fn is_prime(n: &BigUint) -> bool {
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
// low level primality test
pub fn low_level_primality(candidate: &BigUint) -> bool{// Test low level primality, using this it is possible to discard the random numbers faster
    let small_primes: [u32; 11] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    for prime in small_primes {
        if candidate.clone() % prime.to_biguint().unwrap() == BigUint::zero() {
            return false;
        }
    }
    true
}

// Miller Rabin primality test, probability of prime very high
pub fn miller_rabin(n: &BigUint) -> bool {
    //find d and r, where n-1 = 2^r * d
    let mut d = n - &BigUint::from(1u32);
    let mut r = BigUint::zero();
    while d.is_even() {
        d = d >> 1;
        r = &r + &BigUint::one();
        }

    let one_biguint: BigUint = BigUint::one();
    let two_biguint: BigUint = &one_biguint + &one_biguint;
    let mut rng = rand::thread_rng();
    let random_num = one_biguint.clone();
    if n != &BigUint::from(5u64) {
        let _random_num = rng.gen_biguint_range(&one_biguint, &(n - BigUint::from(4u64)));
        }

    let a = BigUint::from(2u64) + random_num;
    let mut x = BigUint::modpow(&a, &d, &n);
    if x == one_biguint || x == n - &one_biguint {
        return true;
        }

    while d != n - &one_biguint {
        x = (&x * &x) % n;
        d *= &two_biguint;
        if x == one_biguint {
            return false;
            }

        if x == n - &one_biguint {
            return true;
            }
    }
    false
}


// Generate prime number using the other functions, note that the number is not a prime with certainity, only with a high probability
pub fn generate_prime(n: u64) -> &'static BigUint {
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
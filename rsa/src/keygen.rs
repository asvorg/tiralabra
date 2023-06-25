//keygen module for generating the keys for the encryption and decryption functions.
use crate::prime_func::PrimeFunc;
use num_bigint::BigInt;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::str::FromStr;
use std::time::Instant;
pub struct Keygen;
impl Keygen {
    //generate the keys for the encryption and decryption functions
    pub fn keygen(num: u64) -> ((BigUint, BigUint), (BigUint, BigUint), (BigUint, BigUint)) {
        let start = Instant::now();
        let p: &BigUint = PrimeFunc::generate_prime(num);
        let q: &BigUint = PrimeFunc::generate_prime(num);
        let n: BigUint = p * q;
        let _z: BigUint = (p - 1u32) * (q - 1u32);
        let e: BigUint = BigUint::from(65537u64);

        let p_minus_one: BigUint = p - BigUint::one();
        let q_minus_one: BigUint = q - BigUint::one();
        let phi: BigUint = &p_minus_one * &q_minus_one;

        //check if e and phi are coprime
        if Keygen::gcd(e.clone(), phi.clone()) != BigUint::one() {
            panic!("e and phi are not coprime");
        }

        let d: BigUint = Keygen::extended_euclidean_algorithm(e.clone(), phi.clone());

        println!("Key generation time was {:?}", start.elapsed());

        ((n.clone(), d), (n, e.clone()), (p.clone(), q.clone()))
    }

    //implement gcd function
    pub fn gcd(a: BigUint, b: BigUint) -> BigUint {
        let mut a: BigUint = a.clone();
        let mut b: BigUint = b.clone();
        while a != BigUint::zero() {
            let t: BigUint = b.clone();
            b = a.clone();
            a = t % a;
        }
        b
    }

    //extended euclidean algorithm for calculating d
    pub fn extended_euclidean_algorithm(e: BigUint, phi: BigUint) -> BigUint {
        let mut e_bigint = Self::convert_biguint_to_bigint(e);
        let mut phi_bigint = Self::convert_biguint_to_bigint(phi);
        let phi_copy = phi_bigint.clone();

        let mut x = BigInt::zero();
        let mut y = BigInt::one();
        let mut u = BigInt::one();
        let mut v = BigInt::zero();

        while e_bigint != BigInt::zero() {
            let q = &phi_bigint / &e_bigint;
            let r = &phi_bigint % &e_bigint;
            let m = &x - &u * &q;
            let n = &y - &v * &q;
            phi_bigint = e_bigint;
            e_bigint = r;
            x = u;
            y = v;
            u = m;
            v = n;
        }

        let x_copy = x.clone();
        //if x is negative, add phi to it
        if x < BigInt::zero() {
            x = x + phi_copy;
            return Self::convert_bigint_to_biguint(x);
        }
        //return x as biguint
        return Self::convert_bigint_to_biguint(x_copy);
    }
    //convert biguint to bigint
    pub fn convert_biguint_to_bigint(number: BigUint) -> num_bigint::BigInt {
        let number_string: String = number.to_str_radix(10);
        let number_bigint: num_bigint::BigInt =
            num_bigint::BigInt::from_str(&number_string).unwrap();
        number_bigint
    }

    //convert bigint to biguint without str_radix
    pub fn convert_bigint_to_biguint(number: num_bigint::BigInt) -> BigUint {
        let number_string: String = number.to_string();
        let number_biguint: BigUint = BigUint::parse_bytes(number_string.as_bytes(), 10).unwrap();
        number_biguint
    }
}

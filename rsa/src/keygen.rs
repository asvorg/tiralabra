use std::str::FromStr;

use num_bigfloat::BigFloat;
use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero,ToPrimitive};
use crate::prime_func::PrimeFunc;
use bigdecimal::BigDecimal;
pub struct Keygen;
impl Keygen{

//generate the keys for the encryption and decryption functions
pub fn keygen(num:u64) -> ((BigUint, BigUint), (BigUint, BigUint)){
    let p: &BigUint = PrimeFunc::generate_prime(num);
    let q: &BigUint = PrimeFunc::generate_prime(num);
    let n: BigUint = p * q;
    let _z: BigUint = (p - 1u32) * (q - 1u32);
    let e: BigUint = BigUint::from(65537u64);

    let p_minus_one: BigUint = p - BigUint::one();
    let q_minus_one: BigUint = q - BigUint::one();
    let phi: BigUint = &p_minus_one * &q_minus_one;

    //check if e and phi are coprime
    if Keygen::gcd(e.clone(), phi.clone()) != BigUint::one(){
        panic!("e and phi are not coprime");
    }

    let d: BigUint = Keygen::calculate_d(e.clone(), phi.clone());

    ((n.clone(), d), (n, e.clone()))
    }

   //implement gcd function
    pub fn gcd(a: BigUint, b: BigUint) -> BigUint{
        let mut a: BigUint = a.clone();
        let mut b: BigUint = b.clone();
        while a != BigUint::zero() {
            let t: BigUint = b.clone();
            b = a.clone();
            a = t % a;
        }
        b
    }
    
    //testing function for keygen with fixed values
    pub fn dummy_keygen() -> ((BigUint, BigUint), (BigUint, BigUint)){
        let p: &BigUint = &61u32.to_biguint().unwrap();
        let q: &BigUint = &53u32.to_biguint().unwrap();
        let n: BigUint = p * q;
        let _z: BigUint = (p - 1u32) * (q - 1u32);

        let e: BigUint = BigUint::from(65537u64);
        let p_minus_one: BigUint = p - BigUint::one();
        let q_minus_one: BigUint = q - BigUint::one();
        let phi: BigUint = &p_minus_one * &q_minus_one;

        if Keygen::gcd(e.clone(), phi.clone()) != BigUint::one(){
            panic!("e and phi are not coprime");
        }

        let d: BigUint = Keygen::calculate_d(e.clone(), phi.clone());

        ((n.clone(), d.clone()), (n, e.clone()))
    }

    //calculate rsa d value
    pub fn calculate_d(e: BigUint, phi: BigUint) -> BigUint{
        let e_float: BigDecimal = Self::biguint_to_bigdecimal(&e);
        let mut phi_float: BigDecimal = Self::biguint_to_bigdecimal(&phi);
        let phi_float_orig: BigDecimal = phi_float;    
        
        let mut d_float: BigDecimal = (BigDecimal::one() + phi_float) / e_float;

        while (BigDecimal::one() + phi_float) % e_float != BigDecimal::zero() {
            let updated_phi_float: BigDecimal = &phi_float_orig + &phi_float;
            d_float = (BigDecimal::one() + updated_phi_float.clone()) / e_float;
            println!("d_float: {}", d_float);
            phi_float = updated_phi_float;
        }

        let d: BigUint = d_float.to_u64().unwrap().into();
        d
    }

    
    fn biguint_to_bigdecimal(value: &BigUint) -> BigDecimal {
        let value_str = value.to_str_radix(10);
        BigDecimal::from_str(&value_str).unwrap()
    }
}

    


    

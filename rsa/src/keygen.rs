use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero,ToPrimitive};
use astro_float::BigFloat as AstroBigFloat;
use crate::prime_func::PrimeFunc;
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
        let e_float: AstroBigFloat = e.to_f64().unwrap().into();
        let mut phi_float: AstroBigFloat = phi.to_f64().unwrap().into();
        let phi_float_orig: AstroBigFloat = phi_float;   
        let astro_one: AstroBigFloat =1.0.to_f64().unwrap().into();  
        let astro_zero: AstroBigFloat =0.0.to_f64().unwrap().into();
        
        let mut added:AstroBigFloat = astro_float::BigFloat::add(&astro_one, &phi_float, 2048, astro_float::RoundingMode::ToEven);
        let mut d_float: AstroBigFloat = astro_float::BigFloat::div(&added,&e_float, 2048, astro_float::RoundingMode::ToEven);

        loop {
            if astro_float::BigFloat::is_int(&d_float)
            {
                break;
            }
            let added:AstroBigFloat = astro_float::BigFloat::add(&astro_one, &phi_float, 2048, astro_float::RoundingMode::ToEven);
            let d_float: AstroBigFloat = astro_float::BigFloat::div(&added,&e_float, 2048, astro_float::RoundingMode::ToEven);
        }

        let d_float64: f64 = astro_float::FromExt::from_ext(&d_float,2048, astro_float::RoundingMode::ToEven);
        d
  
    }

}
    

use num_bigfloat::BigFloat;
use num_bigint::{BigUint, BigInt, ToBigUint};
use num_traits::{One, Zero,ToPrimitive,FromPrimitive};
use crate::prime_func::Prime_func;
pub struct Keygen;
impl Keygen{

pub fn keygen(num:u64) -> ((BigUint, BigUint), (BigUint, BigUint)){ //Calculate the keys for the encryption and decryption functions
    let p: &BigUint = Prime_func::generate_prime(num);
    let q: &BigUint = Prime_func::generate_prime(num);
    let n: BigUint = p * q;
    let z: BigUint = (p - 1u32) * (q - 1u32);
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
 
    pub fn dummy_keygen() -> ((BigUint, BigUint), (BigUint, BigUint)){
        let p: &BigUint = &61u32.to_biguint().unwrap();
        let q: &BigUint = &53u32.to_biguint().unwrap();
        let n: BigUint = p * q;
        let z: BigUint = (p - 1u32) * (q - 1u32);

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

    fn calculate_d(e: BigUint, phi: BigUint) -> BigUint{
        let e_float: BigFloat = e.to_f64().unwrap().into();
        let mut phi_float: BigFloat = phi.to_f64().unwrap().into();
        let phi_float_orig: BigFloat = phi_float;    
        
        let mut d_float: BigFloat = (BigFloat::one() + phi_float) / e_float;

        while (BigFloat::one() + phi_float) % e_float != BigFloat::zero(){
            println!("d_float: {}", d_float);  
            phi_float = phi_float_orig + phi_float;
            d_float = (BigFloat::one() + phi_float) / e_float;
        }

        let d: BigUint = d_float.to_u64().unwrap().into();
        d
  
    }
}
    
use num_bigint::BigUint;
use crate::prime_func;

pub fn generate_prime_variable_length(){
    for n in 10..1000 {
        
        let mut number: &BigUint = &prime_func::generate_prime(n);
        println!("{} {}", number,n)
    }
}
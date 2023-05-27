use num_bigint::BigUint;
use num_traits::{One,Zero};
use super::prime_func; 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_candidate() {
        let n: u64 = 1024;
        let candidate = prime_func::get_candidate(n);
        assert_eq!(candidate.bits(), n as usize); 
        assert_eq!((&candidate % &BigUint::from(2u32)), BigUint::one()); 
    }
}
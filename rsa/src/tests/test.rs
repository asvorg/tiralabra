use super::*;
#[cfg(test)]
mod tests {
    use super::get_candidate;
    use num_bigint::BigUint;
    use num_traits::{One, Zero};

    #[test]
    fn test_get_candidate() {
        // Test multiple cases to ensure correctness

        // Test with n = 8
        let candidate = get_candidate(8);
        assert!(candidate >= BigUint::from(1u32 << 8));  // Ensure candidate is at least 2^8
        assert!(candidate % BigUint::from(2u32) == BigUint::one());  // Ensure candidate is odd

        // Test with n = 16
        let candidate = get_candidate(16);
        assert!(candidate >= BigUint::from(1u32 << 16));  // Ensure candidate is at least 2^16
        assert!(candidate % BigUint::from(2u32) == BigUint::one());  // Ensure candidate is odd

        // Test with n = 32
        let candidate = get_candidate(32);
        assert!(candidate >= BigUint::from(1u32 << 32));  // Ensure candidate is at least 2^32
        assert!(candidate % BigUint::from(2u32) == BigUint::one());  // Ensure candidate is odd
    }
}

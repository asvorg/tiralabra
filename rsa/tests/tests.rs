#[cfg(test)]
mod tests {
    use num_bigint::{ToBigUint, BigUint};
    use num_traits::{Zero, One};
    use rsa::prime_func::Prime_func;   
    use rsa::keygen::Keygen;

#[test]
fn test_is_prime() { 
    // Test with prime numbers
    let primes: [u64; 5] = [2, 3, 5, 7, 11];
    for &prime in primes.iter() {
        let n: BigUint = BigUint::from(prime);
        assert!(Prime_func::is_prime(&n));
    }

    // Test with zero and one
    let zero: BigUint = BigUint::zero();
    let one: BigUint = BigUint::one();
    assert!(!Prime_func::is_prime(&zero));
    assert!(!Prime_func::is_prime(&one));

    // Test with large prime number
    let large_prime: BigUint = BigUint::from(1_000_000_007u64);
    assert!(Prime_func::is_prime(&large_prime));

}

#[test]
fn test_generate_prime() { //very slow if larger prime numbers are used

    // Test generating prime numbers
    for _ in 0..10 {
        let prime: &BigUint = Prime_func::generate_prime(40);
        assert!(Prime_func::is_prime(prime));
    }
}

#[test]
fn test_miller_rabin() {
    //Test the Miller Rabin primality test
    let primes: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let non_primes: Vec<u64> = vec![4, 6, 8, 9, 10, 12, 14, 15, 16, 18, 20];

    // Test known prime numbers
    for &prime in &primes {
        let n: BigUint = prime.to_biguint().unwrap();
        assert!(Prime_func::miller_rabin(&n), "{} should be prime", prime);
    }

    // Test known composite numbers
    for &non_prime in &non_primes {
        let n: BigUint = non_prime.to_biguint().unwrap();
        assert!(!Prime_func::miller_rabin(&n), "{} should not be prime", non_prime);
    }
    
}

#[test]
fn test_low_level_primality() {
    // Test with a prime number
    let prime_candidate = 17.to_biguint().unwrap();
    assert!(Prime_func::low_level_primality(&prime_candidate));

    // Test with a non-prime number
    let non_prime_candidate = 20.to_biguint().unwrap();
    assert!(!Prime_func::low_level_primality(&non_prime_candidate));
    }

#[test]
fn test_keygen() {
    let ((n, d), (m, e)) = Keygen::keygen(1024);

    // Assert that n, d, m, and e are not empty
    assert!(!n.is_zero());
    assert!(!d.is_zero());
    assert!(!m.is_zero());
    assert!(!e.is_zero());

    // Print the generated values
    println!("n: {}", n);
    println!("d: {}", d);
    println!("m: {}", m);
    println!("e: {}", e);
    }

#[test]
fn test_calculate_d() {
    // Test with a known prime number
    let p: BigUint = 61.to_biguint().unwrap();
    let q: BigUint = 53.to_biguint().unwrap();
    let e: BigUint = 65537.to_biguint().unwrap();
    let p_minus_one: BigUint = p - BigUint::one();
    let q_minus_one: BigUint = q - BigUint::one();
    let phi: BigUint = &p_minus_one * &q_minus_one;
    let d: BigUint = Keygen::calculate_d(e.clone(), phi.clone());
    assert_eq!(d, 3723233.to_biguint().unwrap());
    }

#[test]
fn test_gcd() {
    // Test with a known prime number
    let p: BigUint = 61.to_biguint().unwrap();
    let q: BigUint = 53.to_biguint().unwrap();
    let e: BigUint = 65537.to_biguint().unwrap();
    let p_minus_one: BigUint = p - BigUint::one();
    let q_minus_one: BigUint = q - BigUint::one();
    let phi: BigUint = &p_minus_one * &q_minus_one;
    let gcd: BigUint = Keygen::gcd(e.clone(), phi.clone());
    assert_eq!(gcd, BigUint::one());
    }
}
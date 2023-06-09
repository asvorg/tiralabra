#[cfg(test)]
mod tests {
    use num_bigint::{ToBigUint, BigUint, BigInt, ToBigInt};
    use num_traits::{Zero, One};
    use rsa::{encryption, decryption};
    use rsa::prime_func::PrimeFunc;   
    use rsa::keygen::Keygen;

#[test]
fn test_is_prime() { 
    // Test with prime numbers
    let primes: [u64; 5] = [2, 3, 5, 7, 11];
    for &prime in primes.iter() {
        let n: BigUint = BigUint::from(prime);
        assert!(PrimeFunc::is_prime(&n));
    }

    // Test with zero and one
    let zero: BigUint = BigUint::zero();
    let one: BigUint = BigUint::one();
    assert!(!PrimeFunc::is_prime(&zero));
    assert!(!PrimeFunc::is_prime(&one));

    // Test with large prime number
    let large_prime: BigUint = BigUint::from(1_000_000_007u64);
    assert!(PrimeFunc::is_prime(&large_prime));

}

#[test]
fn test_generate_prime() { //very slow if larger prime numbers are used

    // Test generating prime numbers
    for _ in 0..10 {
        let prime: &BigUint = PrimeFunc::generate_prime(40);
        assert!(PrimeFunc::is_prime(prime));
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
        assert!(PrimeFunc::miller_rabin(&n), "{} should be prime", prime);
    }

    // Test known composite numbers
    for &non_prime in &non_primes {
        let n: BigUint = non_prime.to_biguint().unwrap();
        assert!(!PrimeFunc::miller_rabin(&n), "{} should not be prime", non_prime);
    }
    
}

#[test]
fn test_low_level_primality() {
    // Test with a prime number
    let prime_candidate = 17.to_biguint().unwrap();
    assert!(PrimeFunc::low_level_primality(&prime_candidate));

    // Test with a non-prime number
    let non_prime_candidate = 20.to_biguint().unwrap();
    assert!(!PrimeFunc::low_level_primality(&non_prime_candidate));
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

#[test]
//test extended Euclidean Algorithm
fn test_extended_euclidean_algorithm() {
    let e: BigUint = 65537.to_biguint().unwrap();
    let phi: BigUint = 3723232.to_biguint().unwrap();
    let d: BigUint = Keygen::extended_euclidean_algorithm(e.clone(), phi.clone());
    assert_eq!(d, 2293.to_biguint().unwrap());
    }

#[test]
fn test_convert_biguint_to_bigint() {
    let biguint: BigUint = 100.to_biguint().unwrap();
    let bigint: BigInt = Keygen::convert_biguint_to_bigint(biguint);
    assert_eq!(bigint, 100.to_bigint().unwrap());
    }

#[test]
fn test_convert_bigint_to_biguint() {
    let bigint: BigInt = 100.to_bigint().unwrap();
    let biguint: BigUint = Keygen::convert_bigint_to_biguint(bigint);
    assert_eq!(biguint, 100.to_biguint().unwrap());
    }

//test encrypt and decrypt
#[test]
fn test_encrypt_and_decrypt() {
    let ((n, d), (n_2, e)) = Keygen::keygen(256);
    let message: String = String::from("This is a test message");
    let message_uint: BigUint = encryption::Encrypt::convert_text_to_int(&message);
    let message_uint_encrypted: BigUint = encryption::Encrypt::encrypt(message_uint.clone(),n.clone(),e.clone());
    let message_uint_decrypted: BigUint = decryption::Decrpypt::decrypt(message_uint_encrypted, n_2.clone(), d.clone());
    let message_decrypted: String = decryption::Decrpypt::convert_int_to_text(&message_uint_decrypted);
    assert_eq!(message_decrypted, message);
    }


#[test]
//test check_length function, check that the function does not panic when the message is not too long
fn test_check_length_true() {
    let message: String = String::from("S");
    let n: BigUint = 1000000.to_biguint().unwrap();
    let message_uint: BigUint = encryption::Encrypt::convert_text_to_int(&message);
    assert!(encryption::Encrypt::check_length(message_uint, n));
    }

#[test]
//test check_length function, check that the function panics when the message is too long
#[should_panic]
fn test_check_length_panic() {
    let message: String = String::from("This message is too longThis message is too longThis message is too longThis message is too longThis message is too longThis message is too longThis message is too long");
    let n: BigUint = 100.to_biguint().unwrap();
    let message_uint: BigUint = encryption::Encrypt::convert_text_to_int(&message);
    encryption::Encrypt::check_length(message_uint, n);
    }


}
use num::pow;
use rand::Rng;


fn get_candidate() -> u128 {
    let mut rng = rand::thread_rng();
    let n: u128 = 61;

    let prime_min = pow(2, (n - 1).try_into().unwrap()) + 1;
    let prime_max = pow(2, n.try_into().unwrap()) - 1;

    let mut candidate: u128 = rng.gen_range(prime_min..prime_max);
    if candidate % 2 == 0{
        candidate = candidate - 1;
    }
    return candidate
}

fn low_level_primality() -> u128 {
    let v = sieve_of_eratosthenes();
    let mut prime_candidate: u128 = get_candidate();
    loop {
        prime_candidate = get_candidate();
        for num in v.iter() {
            if prime_candidate % num == 0 && pow(*num, 2) <= prime_candidate {
                break;
            }else{
                return prime_candidate
            }
        }
    }
}

fn sieve_of_eratosthenes() -> Vec<u128> {
    let n: u128 = 1000000;
    let mut test_list = vec![true; (n + 1) as usize];
    test_list[0] = false;
    test_list[1] = false;
    
    let mut prime_numbers = Vec::new();

    for i in 2..=n {
        if test_list[i as usize] {
            prime_numbers.push(i);
            let mut j = i * i;
            while j <= n {
                test_list[j as usize] = false;
                j += i;
            }
        }
    }

   return prime_numbers
}

fn fermat_primality(prime_candidate: u128) -> bool{
    let k: usize = 100;
    let mut rng = rand::thread_rng();

    for _ in 0..k {
        let a: u128 = rng.gen_range(2..prime_candidate);
        let power = mod_exp(a, prime_candidate - 1, prime_candidate);

        if power != 1 {
            return false;
        }
    }

    return true 
}

fn mod_exp(base: u128, exponent: u128, modulus: u128) -> u128 {
    if modulus == 1 {
        return 0; 
    }

    let mut result = 1;
    let mut base = base % modulus;

    let mut exp = exponent;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }

    return result
}

pub fn get_prime() -> u128{
    let mut prime_candidate: u128;
    loop{
       prime_candidate = low_level_primality();
       //println!("{}",prime_candidate);
       if fermat_primality(prime_candidate) == true{
        return prime_candidate
       }
    }
}
    

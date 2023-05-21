use num::pow;
use rand::Rng;


pub fn get_candidate() -> u128 {
    let mut rng = rand::thread_rng();
    let n: u128 = 120;

    let prime_min = pow(2, (n - 1).try_into().unwrap()) + 1;
    let prime_max = pow(2, n.try_into().unwrap()) - 1;

    let candidate: u128 = rng.gen_range(prime_min..prime_max);
    return candidate
}

pub fn low_level_primality() -> u128 {
    let v = sieve_of_eratosthenes();
    loop {
        let mut prime_candidate: u128 = get_candidate();
        for num in v.iter() {
            if prime_candidate % num == 0 && pow(*num, 2) <= prime_candidate {
                break;
            }else{
                return prime_candidate
            }
        }
    }
}

pub fn sieve_of_eratosthenes() -> Vec<u128> {
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
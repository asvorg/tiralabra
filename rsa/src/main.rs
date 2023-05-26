use num_bigint::BigUint;

mod prime_func;

fn main() {
    loop 
     {  
        let mut number: BigUint = prime_func::get_candidate();
        if prime_func::is_prime(&number) {
            println!("{} is a prime number.", number)
            }

        }
}
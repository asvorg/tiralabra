mod prime_func;
use crate::prime_func::get_candidate;
use crate::prime_func::sieve_of_eratosthenes;
use crate::prime_func::low_level_primality;

fn main() {
    //let n = get_candidate();
    //println!("{}",n);
    //let v2 = sieve_of_eratosthenes();
    //println!("{:?}", v2);
    let llp = low_level_primality();
    println!("{}",llp);
}   

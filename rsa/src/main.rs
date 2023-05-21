mod prime_func;
use crate::prime_func::get_prime;

fn main() {
    for i in 0..100{
    let mut n = get_prime();
    println!("{}",n);
    }   
}
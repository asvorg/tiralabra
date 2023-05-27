use num_bigint::BigUint;
mod prime_func;
mod testers;
mod rsa;

fn main() {

    println!("{:?}",rsa::keygen())

}

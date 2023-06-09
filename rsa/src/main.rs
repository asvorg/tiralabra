use std::env;
mod keygen;
mod encryption;
mod decryption;
mod prime_func;
mod padding;
mod ui;
use num_bigint::ToBigUint;

use crate::keygen::Keygen;


fn main() {
    env::set_var("RUST_BACKTRACE", "128");
    println!("{}", prime_func::PrimeFunc::miller_rabin(&7919.to_biguint().unwrap()));
}

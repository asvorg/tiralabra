use std::env;
mod prime_func;
mod keygen;

use crate::keygen::Keygen;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let ((n, d), (_n_2, e)) = Keygen::keygen();
    println!("Private key: {},{}", n,d);
    println!();
    println!("Public key: {},{}", n,e);

}

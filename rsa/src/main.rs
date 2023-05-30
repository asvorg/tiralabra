use std::env;
mod prime_func;
mod testers;
mod rsa;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    println!("{:?}",rsa::keygen())

}

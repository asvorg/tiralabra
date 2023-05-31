use std::env;
mod prime_func;
mod keygen;
mod encryption;
use crate::{keygen::Keygen, encryption::Encrypt};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let ((n, d), (_n_2, e)) = Keygen::keygen(256);
    println!("Private key: {},{}", n,d);
    println!();
    println!("Public key: {},{}", n,e);
    let message = "Test";
    println!("{}",encryption::Encrypt::convert_text_to_int(message));
    let message_uint = encryption::Encrypt::convert_text_to_int(message);
    println!("{}",encryption::Encrypt::encrypt(message_uint,n,e));

}

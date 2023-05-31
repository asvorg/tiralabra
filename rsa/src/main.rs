use std::env;
mod prime_func;
mod keygen;
mod encryption;
mod decryption;
use crate::{keygen::Keygen};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let ((n, d), (n_2, e)) = Keygen::keygen(512);
    println!("Private key: {},{}", n,d);
    println!();
    println!("Public key: {},{}", n,e);
    println!();
    let message = "Test";
    let message_uint = encryption::Encrypt::convert_text_to_int(message);
    let message_uint_encrypted = encryption::Encrypt::encrypt(message_uint.clone(),n,e);
    println!("{}",message_uint);
    println!();
    println!("{}",message_uint_encrypted);
    println!();
    let message_uint_decrypted = decryption::Decrpypt::decrypt(message_uint_encrypted, n_2, d);
    println!("{}",message_uint_decrypted);

}

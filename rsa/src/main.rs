use std::env;
mod prime_func;
mod keygen;
mod encryption;
mod decryption;
use crate::{keygen::Keygen};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let ((n, d), (n_2, e)) = Keygen::keygen(64);
    println!("n: {}", n);
    println!();
    println!("d: {}", d);
    println!();
    println!("e: {}", e);
    println!();
    let message = "w";
    let message_uint = encryption::Encrypt::convert_text_to_int(message);
    let message_uint_encrypted = encryption::Encrypt::encrypt(message_uint.clone(),n,e);
    println!("message_uint: {}",message_uint);
    println!();
    println!("message_uint_encrypted: {}",message_uint_encrypted);
    println!();
    let message_uint_decrypted = decryption::Decrpypt::decrypt(message_uint_encrypted, n_2, d);
    let message_decrypted = decryption::Decrpypt::convert_int_to_text(&message_uint_decrypted);
    println!("message_uint_decrypted: {}",message_uint_decrypted);
    println!();
    println!("message_decrypted: {}",message_decrypted);
}

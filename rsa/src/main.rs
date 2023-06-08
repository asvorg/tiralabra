use std::env;
mod keygen;
mod encryption;
mod decryption;
mod prime_func;
mod padding;
mod ui;
use crate::keygen::Keygen;


fn main() {
    env::set_var("RUST_BACKTRACE", "128");
    let ((n, d), (n_2, e)) = Keygen::keygen(1024);
    println!("n: {}", n);
    println!();
    println!("d: {}", d);
    println!();
    println!("e: {}", e);
    println!();
    //let message: String = padding::Padding::pad(32, "iiiii");
    let message: String = String::from("This is a test message");
    let message_uint: num_bigint::BigUint = encryption::Encrypt::convert_text_to_int(&message);
    let message_uint_encrypted: num_bigint::BigUint = encryption::Encrypt::encrypt(message_uint.clone(),n,e);
    println!("message_uint: {}",message_uint);
    println!();
    println!("message_uint_encrypted: {}",message_uint_encrypted);
    println!();
    let message_uint_decrypted: num_bigint::BigUint = decryption::Decrpypt::decrypt(message_uint_encrypted, n_2, d);
    let message_decrypted: String = decryption::Decrpypt::convert_int_to_text(&message_uint_decrypted);
    //message_decrypted = padding::Padding::unpad(&message_decrypted);
    println!("message_uint_decrypted: {}",message_uint_decrypted);
    println!();
    println!("message_decrypted: {}",message_decrypted);
}

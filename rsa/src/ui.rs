use std::io;
use crate::keygen::Keygen;
use crate::encryption::Encrypt;
use crate::decryption::Decrpypt;

pub struct Ui;
impl Ui {
    //very ugly demo function
    #[cfg(not(tarpaulin_include))]
    pub fn demo() -> (){
        let mut prime_size:u64 = 256;
        let ((n, d), (n_2, e), (p, q)) = Keygen::keygen(prime_size);
        let n_copy = n.clone();
        let n_copy2 = n.clone();
        let n_copy3 = n_2.clone();
        let d_copy = d.clone();
        let e_copy = e.clone();
        println!("Prime size: {}", prime_size);
        println!();
        println!("p: {}", p);
        println!();
        println!("q: {}", q);
        println!();
        println!("n: {}", n_copy);
        println!();
        println!("d: {}", d_copy);
        println!();
        println!("e: {}", e_copy);
        println!();
        let message: String = String::from("This is a test message");
        println!("Message: {}",message);
        println!();
        let message_uint: num_bigint::BigUint = Encrypt::convert_text_to_int(&message);
        Encrypt::check_length(message_uint.clone(), n_copy.clone());
        let message_uint_encrypted: num_bigint::BigUint = Encrypt::encrypt(message_uint.clone(),n,e);
        println!("Message converted to integer: {}",message_uint);
        println!();
        println!("Encrypted message: {}",message_uint_encrypted);
        println!();
        let message_uint_decrypted: num_bigint::BigUint = Decrpypt::decrypt(message_uint_encrypted, n_copy, d);
        let message_decrypted: String = Decrpypt::convert_int_to_text(&message_uint_decrypted);
        println!("Message decrypted but still an integer: {}",message_uint_decrypted);
        println!();
        println!("Message decrypted and converted: {}",message_decrypted);
        println!();
        println!();
        println!();
        println!();
        println!();
        println!();
        prime_size = 1024;
        println!("Prime size: {}", prime_size);
        println!();
        let message_2 = String::from("This is a second test message");
        println!("Message: {}",message_2);
        println!();
        let message_uint_2: num_bigint::BigUint = Encrypt::convert_text_to_int(&message_2);
        println!("Message converted to integer: {}",message_uint_2);
        println!();
        let message_uint_encrypted_2: num_bigint::BigUint = Encrypt::encrypt(message_uint_2.clone(),n_copy2,e_copy);
        println!("Encrypted message: {}",message_uint_encrypted_2);
        println!();
        let message_uint_decrypted_2: num_bigint::BigUint = Decrpypt::decrypt(message_uint_encrypted_2, n_copy3, d_copy);
        println!("Message decrypted but still an integer: {}",message_uint_decrypted_2);
        println!();
        let message_decrypted_2: String = Decrpypt::convert_int_to_text(&message_uint_decrypted_2);
        println!("Message decrypted and converted: {}",message_decrypted_2);
    }

    #[cfg(not(tarpaulin_include))]
    pub fn encrypt_and_print() -> () {
        //read the message from the user as an input
        let mut message: String = String::new();
        //read the prime size from the user as an input
        println!("Enter prime size: ");
        io::stdin().read_line(&mut message).expect("Failed to read line");
        let prime_size: u64 = message.trim().parse().expect("Please type a number!");
        //generate the public and private keys
        let ((n, d), (n_2, e),(_p,_q)) = Keygen::keygen(prime_size);
        
        loop {
        println!("Enter a message to encrypt: ");
        io::stdin().read_line(&mut message).expect("Failed to read line");
        //convert the message to an integer
        let mut message_uint: num_bigint::BigUint = Encrypt::convert_text_to_int(&message);
        //encrypt the message
        let mut message_uint_encrypted: num_bigint::BigUint = Encrypt::encrypt(message_uint.clone(),n.clone(),e.clone());
        //print the encrypted message
        println!("Encrypted message: {}",message_uint_encrypted);
            }
        }
}
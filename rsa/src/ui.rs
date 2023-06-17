//ui
use crate::decryption::Decrypt;
use crate::encryption::Encrypt;
use crate::keygen::Keygen;
use num_bigint::BigUint;
use std::io;

pub struct Ui;
impl Ui {
    //very ugly demo function
    #[cfg(not(tarpaulin_include))] //#[cfg(not(tarpaulin_include))] is for excluding from tarpaulin coverage
    pub fn demo() -> () {
        let mut prime_size: u64 = 256;
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
        println!("Message: {}", message);
        println!();
        let message_uint: num_bigint::BigUint = Encrypt::convert_text_to_int(&message);
        Encrypt::check_length(message_uint.clone(), n_copy.clone());
        let message_uint_encrypted: num_bigint::BigUint =
            Encrypt::encrypt(message_uint.clone(), n, e);
        println!("Message converted to integer: {}", message_uint);
        println!();
        println!("Encrypted message: {}", message_uint_encrypted);
        println!();
        let message_uint_decrypted: num_bigint::BigUint =
            Decrypt::decrypt(message_uint_encrypted, n_copy, d);
        let message_decrypted: String = Decrypt::convert_int_to_text(&message_uint_decrypted);
        println!(
            "Message decrypted but still an integer: {}",
            message_uint_decrypted
        );
        println!();
        println!("Message decrypted and converted: {}", message_decrypted);
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
        println!("Message: {}", message_2);
        println!();
        let message_uint_2: num_bigint::BigUint = Encrypt::convert_text_to_int(&message_2);
        println!("Message converted to integer: {}", message_uint_2);
        println!();
        let message_uint_encrypted_2: num_bigint::BigUint =
            Encrypt::encrypt(message_uint_2.clone(), n_copy2, e_copy);
        println!("Encrypted message: {}", message_uint_encrypted_2);
        println!();
        let message_uint_decrypted_2: num_bigint::BigUint =
            Decrypt::decrypt(message_uint_encrypted_2, n_copy3, d_copy);
        println!(
            "Message decrypted but still an integer: {}",
            message_uint_decrypted_2
        );
        println!();
        let message_decrypted_2: String = Decrypt::convert_int_to_text(&message_uint_decrypted_2);
        println!("Message decrypted and converted: {}", message_decrypted_2);
    }

    #[cfg(not(tarpaulin_include))]
    fn encrypt_and_print() -> () {
        let mut message: String = String::new();
        println!("Enter prime size: ");
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");
        let prime_size: u64 = message.trim().parse().expect("Please type a number!");
        let ((n, _d), (_n_2, e), (_p, _q)) = Keygen::keygen(prime_size);
        loop {
            println!("Enter a message to encrypt: ");
            io::stdin()
                .read_line(&mut message)
                .expect("Failed to read line");
            let message_uint: num_bigint::BigUint = Encrypt::convert_text_to_int(&message);
            let message_uint_encrypted: num_bigint::BigUint =
                Encrypt::encrypt(message_uint.clone(), n.clone(), e.clone());
            println!("Encrypted message: {}", message_uint_encrypted);
        }
    }
    #[cfg(not(tarpaulin_include))]
    pub fn menu() -> () {
        println!();
        println!("Welcome to the RSA encryption and decryption program!");
        println!("Please select the mode you would like to use:");
        println!("1. Encrypt a message");
        println!("2. Decrypt a message");
        println!("3. Generate keys");
        println!(
            "4. Demo mode, generate relevant prime numbers and keys, encrypt and decrypt a message"
        );
        println!("5. Demo mode with user supplied keys");
        println!("6. Exit");
        println!("7. This message");
        println!("Please enter the number of the mode you would like to use: ");
        println!();
    }

    #[cfg(not(tarpaulin_include))]
    fn decrypt_and_print() {
        let n_string = String::from("Enter n");
        let d_string = String::from("Enter d");
        let n: BigUint = BigUint::parse_bytes(&n_string.into_bytes(), 10).unwrap();
        let d: BigUint = BigUint::parse_bytes(&d_string.into_bytes(), 10).unwrap();
        let message_string = String::from("Enter message");
        let message: BigUint = Encrypt::convert_text_to_int(&message_string);
        println!("{}", Decrypt::decrypt(message, d, n));
    }

    #[cfg(not(tarpaulin_include))]
    fn generate_keys() -> () {
        let mut message: String = String::new();
        println!("Enter prime size: ");
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");
        let prime_size: u64 = message.trim().parse().expect("Please type a number!");
        let ((n, d), (_n_2, e), (_p, _q)) = Keygen::keygen(prime_size);
        println!();
        println!("n: {}", n);
        println!();
        println!("d: {}", d);
        println!();
        println!("e: {}", e);
        println!();
    }

    #[cfg(not(tarpaulin_include))]
    fn demo_with_user_keys() -> () {
        let mut message: String = String::new();
        println!("Enter prime size: ");
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");
        let prime_size: u64 = message.trim().parse().expect("Please type a number!");
        let ((n, d), (n_2, e), (_p, _q)) = Keygen::keygen(prime_size);
        println!("Enter message to encrypt: ");
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");
        let message_uint: num_bigint::BigUint = Encrypt::convert_text_to_int(&message);
        let message_uint_encrypted: num_bigint::BigUint =
            Encrypt::encrypt(message_uint.clone(), n.clone(), e.clone());
        println!("Encrypted message: {}", message_uint_encrypted);
        let message_uint_decrypted: num_bigint::BigUint =
            Decrypt::decrypt(message_uint_encrypted, n_2, d);
        let message_decrypted: String = Decrypt::convert_int_to_text(&message_uint_decrypted);
        println!("Message decrypted and converted: {}", message_decrypted);
    }

    #[cfg(not(tarpaulin_include))]
    pub fn selection() -> () {
        let mut message: String = String::new();
        io::stdin()
            .read_line(&mut message)
            .expect("Failed to read line");
        let selection: u64 = message.trim().parse().expect("Please type a number!");
        match selection {
            1 => Ui::encrypt_and_print(),
            2 => Ui::decrypt_and_print(),
            3 => Ui::generate_keys(),
            4 => Ui::demo(),
            5 => Ui::demo_with_user_keys(),
            6 => Ui::quit(),
            7 => Ui::menu(),
            _ => println!("Please enter a valid number"),
        }
    }
    fn quit() -> () {
        //force quit
        println!("Goodbye!");
        std::process::exit(0);
    }
}

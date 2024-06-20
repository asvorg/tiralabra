//encryption module and the relevant conversions from text to int and vice versa.
use num_bigint::BigUint;
use num_traits::Zero;

pub struct Encrypt;
impl Encrypt {
    //convert utf8 text to int
    pub fn convert_text_to_int(text: &str) -> BigUint {
        let mut int: BigUint = BigUint::zero();
        for c in text.chars() {
            int *= 256u32;
            int += c as u8;
        }
        int
    }

    //implement encrypt function
    pub fn encrypt(message: BigUint, n: BigUint, e: BigUint) -> BigUint {
        if message >= n {
            println!("Message is too is too long for the key size");
            return BigUint::zero();
        }
        let ciphertext = message.modpow(&e, &n);
        ciphertext
    }
}

//encryption module and the relevant conversions from text to int and vice versa.
use num_bigint::{BigUint};
use num_traits::{Zero};

pub struct Encrypt;
impl Encrypt{

//convert utf8 text to int
pub fn convert_text_to_int(text: &str) -> BigUint{
    let mut int: BigUint = BigUint::zero();
    for c in text.chars(){
        int *= 256u32;
        int += c as u8;
    }
    int
}


//implement encrypt function
pub fn encrypt(message: BigUint, n:BigUint, e:BigUint) -> BigUint{
    let ciphertext = message.modpow(&e, &n);
    ciphertext
    }
//message length must be less than n
pub fn check_length(message: BigUint, n:BigUint) -> bool{
    if message < n{
        true
        }
    else{
        panic!("Message length must be less than n");
        }
    }
}
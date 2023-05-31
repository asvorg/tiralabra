use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero,ToPrimitive,FromPrimitive};

pub struct Encrypt;
impl Encrypt{

pub fn convert_text_to_int(text: &str) -> BigUint{
    let m = text.chars().fold(BigUint::zero(), |acc, c| {
        let ascii_val = c as u32;
        acc * 256u32.to_biguint().unwrap() + ascii_val.to_biguint().unwrap()
        });
    m
    }

pub fn encrypt(plaintext: BigUint,n:BigUint,e:BigUint) -> BigUint{
    let cipher: BigUint = plaintext.modpow(&e,&n);
    cipher
}

}
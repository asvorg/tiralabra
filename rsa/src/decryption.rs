use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero,ToPrimitive,FromPrimitive};

pub struct Decrpypt;
impl Decrpypt {

   pub fn convert_int_to_text(m: &BigUint) -> String {
        let mut result = String::new();
        let mut num = m.clone();
    
        while num > BigUint::zero() {
            let remainder = (num.clone() % 256u32.to_biguint().unwrap()).to_u32().unwrap() as u8;
            result.push(remainder as char);
            num /= 256u32.to_biguint().unwrap();
        }
    
        result.chars().rev().collect::<String>()
    }

    pub fn decrypt(ciphertext: BigUint,n:BigUint,d:BigUint) -> BigUint{
        let decrypted: BigUint = ciphertext.modpow(&d,&n);
        decrypted
    }

}
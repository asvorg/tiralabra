use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero,ToPrimitive,FromPrimitive};

pub struct Decrpypt;
impl Decrpypt {

    //convert int to text
    pub fn convert_int_to_text(int: &BigUint) -> String{
        let mut int: BigUint = int.clone();
        let mut text: String = String::new();
        while int > BigUint::zero(){
            let c: char = (int.clone() % 256u32.to_biguint().unwrap()).to_u32().unwrap() as u8 as char;
            text.insert(0,c);
            int /= 256u32;
        }
        text
    }

    //implement decrypt function
    pub fn decrypt(ciphertext: BigUint,n:BigUint,d:BigUint) -> BigUint{
        let message = ciphertext.modpow(&d, &n);
        message
    }

}
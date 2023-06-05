
pub struct Padding;
impl Padding{
    

// a padding function which takes the key size and the message and pads it with 0s
// doesnt always work
pub fn pad(key_size: u64, message: &str) -> String{
    let mut padded_message: String = String::new();
    let message: String = message.to_string();
    let mut byte_len: u64 = message.len() as u64;
    while byte_len < key_size / 8{
        padded_message.push('0');
        byte_len += 1;
    }
    padded_message.push_str(&message);
    println!("padded_message: {}", padded_message);
    padded_message
    }

//unpad the message
pub fn unpad(message: &str) -> String{
    let mut message: String = message.to_string();
    let mut i: usize = 0;
    while message.chars().nth(i).unwrap() == '0'{
        i += 1;
    }
    message = message.chars().skip(i).collect();
    message
    }



}
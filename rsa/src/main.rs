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
    ui::Ui::encrypt_and_print();
}

use std::env;
mod keygen;
mod encryption;
mod decryption;
mod prime_func;
mod padding;
mod ui;


fn main() {
    env::set_var("RUST_BACKTRACE", "128");
    ui::Ui::demo();
}

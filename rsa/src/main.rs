use std::env;
mod keygen;
mod encryption;
mod decryption;
mod prime_func;
mod ui;

#[cfg(not(tarpaulin_include))]
fn main() {
    env::set_var("RUST_BACKTRACE", "128");
    ui::Ui::startup();
    ui::Ui::demo();
}

use std::env;
mod decryption;
mod encryption;
mod keygen;
mod prime_func;
mod ui;

#[cfg(not(tarpaulin_include))]
fn main() {
    env::set_var("RUST_BACKTRACE", "128");
    ui::Ui::menu();
    loop {
        ui::Ui::selection();
        println!();
        ui::Ui::menu();
    }
}

mod prime_func;
mod test_prime;

fn main() {
    for _i in 0..100{
    println!("{}",prime_func::low_level_primality());
    }
}
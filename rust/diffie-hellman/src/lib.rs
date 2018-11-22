extern crate rand;

use rand::thread_rng;
use rand::Rng;

///
/// Calculate a base raised to an exponent under a modulus
/// O(log exp) run time
/// O(log exp) memory for recursive stack frames
fn exp_mod(base: u64, exp: u64, modulus: u64) -> u64 {
    (match exp {
        0 => 1,
        1 => base,
        _ if exp % 2 == 0 => exp_mod(base, exp / 2, modulus).pow(2),
        _ => {
            let floor_half = exp / 2;
            exp_mod(base, floor_half, modulus) * exp_mod(base, exp - floor_half, modulus)
        }
    }) % modulus
}

pub fn private_key(p: u64) -> u64 {
    thread_rng().gen_range(1, p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // pub = g**a mod p
    exp_mod(g, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // s = b_pub**a mod p
    exp_mod(b_pub, a, p)
}

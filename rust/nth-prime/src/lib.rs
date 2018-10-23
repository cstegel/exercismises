extern crate integer_sqrt;

use integer_sqrt::IntegerSquareRoot;

fn primes() -> impl Iterator<Item = u32> {
    (2..).filter(|x| is_prime(*x))
}

fn factors(x: u32) -> impl Iterator<Item = u32> {
    (2..=x.integer_sqrt()).filter(move |fact| x % fact == 0)
}

fn is_prime(x: u32) -> bool {
    match factors(x).next() {
        // Need to check first factor when x == 2
        Some(fact) => fact == x,
        None => true,
    }
}

pub fn nth(n: u32) -> u32 {
    primes().nth(n as usize).unwrap()
}

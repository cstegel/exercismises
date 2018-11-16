pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut factor = 2;
    let mut x = n;
    while factor <= x {
        while x % factor == 0 {
            factors.push(factor);
            x /= factor;
        }
        factor += 1;
    }
    factors
}

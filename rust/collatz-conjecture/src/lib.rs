pub fn collatz(mut n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut steps = 0;
    loop {
        if n == 1 {
            return Some(steps);
        }
        match n % 2 {
            0 => n /= 2,
            _ => n = 3 * n + 1,
        };
        steps += 1;
    }
}

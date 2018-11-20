pub fn collatz(n: u64) -> Option<u64> {
    // recursive solution
    match n {
        0 => None,
        1 => Some(0),
        _ => match n {
            _ if n % 2 == 0 => collatz(n / 2),
            _ => collatz(3 * n + 1),
        }.map(|x| x + 1),
    }

    // Below is the iterative solution
    // if n == 0 {
    //     return None;
    // }
    // let mut steps = 0;
    // loop {
    //     if n == 1 {
    //         return Some(steps);
    //     }
    //     match n % 2 {
    //         0 => n /= 2,
    //         _ => n = 3 * n + 1,
    //     };
    //     steps += 1;
    // }
}

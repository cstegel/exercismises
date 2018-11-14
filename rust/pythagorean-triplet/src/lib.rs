pub fn find() -> Option<u32> {
    // uses formulas
    // c = 1000 - a - b
    // b = (1_000_000 - 2000a) / (2000 - 2a)
    for a in 1..999u32 {
        let num = 1_000_000u32.checked_sub(2000 * a);
        let b = if let Some(x) = num {
            let denom = 2000 - 2 * a;
            // only proceed if integer division is even
            if x % denom != 0 {
                continue;
            }
            x / denom
        } else {
            continue;
        };

        if b <= a {
            continue;
        }

        match 1000u32.checked_sub(a + b) {
            Some(c) if c > b => {
                return Some(a * b * c);
            }
            _ => continue,
        }
    }
    None
}

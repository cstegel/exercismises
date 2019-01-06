/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let rev_no_white = || code.chars().rev().filter(|c| !c.is_whitespace());

    // don't allow invalid digits or sequences of <= 1 digit
    if rev_no_white().any(|c| !c.is_digit(10) || rev_no_white().count() <= 1) {
        return false;
    }

    rev_no_white()
        .map(|c| c.to_digit(10).unwrap())
        .enumerate()
        .map(|(i, digit)| {
            if i % 2 == 0 {
                digit
            } else {
                let prod = 2 * digit;
                if prod > 9 {
                    prod - 9
                } else {
                    prod
                }
            }
        })
        .sum::<u32>()
        % 10
        == 0
}

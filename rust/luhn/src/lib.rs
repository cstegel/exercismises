/// Check a Luhn checksum.
/// Inspired by JaneL's brilliant solution here:
/// https://exercism.io/tracks/rust/exercises/luhn/solutions/b310e5203500486398d482b47561cac1
pub fn is_valid(code: &str) -> bool {
    code.chars()
        .rev()
        .filter(|c| !c.is_whitespace())
        .try_fold((0, 0), |(acc, n_digits), digit_candidate| {
            digit_candidate
                .to_digit(10)
                .map(|num| {
                    if n_digits % 2 == 0 {
                        num
                    } else {
                        match 2 * num {
                            prod if prod > 9 => prod - 9,
                            prod => prod,
                        }
                    }
                })
                .map(|num| (acc + num, n_digits + 1))
        })
        .map_or(false, |(sum, n_digits)| sum % 10 == 0 && n_digits > 1)
}

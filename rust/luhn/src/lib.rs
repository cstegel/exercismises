/// Check a Luhn checksum.
/// Inspired by JaneL's brilliant solution here:
/// https://exercism.io/tracks/rust/exercises/luhn/solutions/b310e5203500486398d482b47561cac1
pub fn is_valid(code: &str) -> bool {
    let mut n_digits = 0;
    let mut sum = 0;

    for c in code.chars().rev() {
        if c.is_whitespace() {
            continue;
        }
        let maybe_digit = c.to_digit(10);
        if maybe_digit.is_none() {
            return false;
        }
        let digit = maybe_digit.unwrap();

        let contribution = if n_digits % 2 == 0 {
            digit
        } else {
            match 2 * digit {
                prod if prod > 9 => prod - 9,
                prod => prod,
            }
        };

        sum += contribution;
        n_digits += 1;
    }

    sum % 10 == 0 && n_digits > 1

    // code.chars()
    //     .rev()
    //     .filter(|c| !c.is_whitespace())
    //     .try_fold((0, 0), |(sum, n_digits), digit_candidate| {
    //         digit_candidate
    //             .to_digit(10)
    //             .map(|digit| {
    //                 if n_digits % 2 == 0 {
    //                     digit
    //                 } else {
    //                     match 2 * digit {
    //                         prod if prod > 9 => prod - 9,
    //                         prod => prod,
    //                     }
    //                 }
    //             })
    //             .map(|digit| (sum + digit, n_digits + 1))
    //     })
    //     .map_or(false, |(sum, n_digits)| sum % 10 == 0 && n_digits > 1)
}

use std::iter;

/// Determines whether the supplied string is a valid ISBN number
/// 10 digits with optional "-" between some of them, each must be 0-9
/// except last which can be 10
/// valid if:
///  (x1 * 10 + x2 * 9 + x3 * 8 + x4 * 7 + x5 * 6 + x6 * 5
/// + x7 * 4 + x8 * 3 + x9 * 2 + x10 * 1) mod 11 == 0
pub fn is_valid_isbn(isbn: &str) -> bool {
    // treat as unicode to prevent a panic when assuming ascii but given unicode
    let undashed_isbn = || isbn.chars().filter(|ch| *ch != '-');

    let n = undashed_isbn().count();
    // check number of digits
    if n != 10 {
        return false;
    }

    let all_but_last = || undashed_isbn().take(n - 1);

    // check all but last digits are valid chars
    if undashed_isbn().take(n - 1).any(|ch| !ch.is_digit(10)) {
        return false;
    }

    // check last digit is valid
    let last_ch = undashed_isbn().last().unwrap();
    if !(last_ch.is_digit(10) || last_ch == 'X') {
        return false;
    }

    // evaluate the digits
    let digit_eval: u32 = all_but_last()
        .map(|ch| ch.to_digit(10).unwrap())
        .chain(iter::once(match last_ch {
            'X' => 10,
            _ => last_ch.to_digit(10).unwrap(),
        })).enumerate()
        .map(|(i, x)| x * (10 - i as u32))
        .sum();

    // check modulus is correct
    digit_eval % 11 == 0
}

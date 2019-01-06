#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        // tests say 1 is expected return value of 0-length span
        return Ok(1);
    }

    // Inspired by bwasty's solution:
    // https://exercism.io/tracks/rust/exercises/largest-series-product/solutions/aa49c251125a49f89fbb698e5e41ca04
    string_digits
        .chars()
        .map(|c| c.to_digit(10).ok_or(Error::InvalidDigit(c)))
        .collect::<Result<Vec<_>, _>>()?
        .windows(span)
        .map(|window| window.iter().product::<u32>() as u64)
        .max()
        .ok_or(Error::SpanTooLong)
}

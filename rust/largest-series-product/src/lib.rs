#[derive(Debug, PartialEq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span > string_digits.len() {
        return Err(Error::SpanTooLong);
    }
    if span == 0 {
        // tests say 1 is expected return value of 0-length span
        return Ok(1);
    }

    let digits = match string_digits
        .chars()
        .map(|c| c.to_digit(10).ok_or(c))
        .collect::<Result<Vec<_>, char>>()
    {
        Err(invalid_char) => return Err(Error::InvalidDigit(invalid_char)),
        Ok(digits) => digits,
    };

    match digits
        .windows(span)
        .map(|window| window.iter().product::<u32>())
        .max()
    {
        Some(max_prod) => Ok(max_prod as u64),
        None => panic!("Shouldn't get here"),
    }
}

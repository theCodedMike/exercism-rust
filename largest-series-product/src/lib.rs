#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    string_digits
        .chars()
        .map(|c| {
            c.to_digit(10)
                .map(|num| num as u64)
                .ok_or(Error::InvalidDigit(c))
        })
        .collect::<Result<Vec<_>, Error>>()?
        .windows(span)
        .map(|series| series.into_iter().product())
        .max()
        .ok_or(Error::SpanTooLong)
}

/// Iterate over the digits of a number starting from the least significant
#[derive(Debug)]
struct Digits {
    num: u32,
    done: bool,
}

impl Digits {
    fn new(num: u32) -> Digits {
        Digits { num, done: false }
    }
}

impl Iterator for Digits {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        match self.done {
            false => {
                let digit = self.num % 10;
                self.num /= 10;
                self.done = self.num == 0;
                Some(digit)
            }
            true => None,
        }
    }
}

pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<_> = Digits::new(num).collect();
    digits
        .iter()
        .map(|digit| digit.pow(digits.len() as u32))
        .sum::<u32>()
        == num
}

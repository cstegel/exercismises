const UNDER_20: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 9] = [
    "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
];

const ORDERS: [&str; 6] = [
    "thousand",
    "million",
    "billion",
    "trillion",
    "quadrillion",
    "quintillion",
];

fn triplet_words(n: u64, order: usize) -> String {
    let digit_words = match n {
        0...19 => UNDER_20[n as usize].to_string(),
        20...99 => {
            TENS[((n / 10) - 1) as usize].to_string() + match n % 10 {
                0 => "".to_string(),
                digit => "-".to_string() + UNDER_20[digit as usize],
            }.as_str()
        }
        _ => {
            UNDER_20[(n / 100) as usize].to_string() + " hundred" + match n % 100 {
                0 => "".to_string(),
                under_100 => " ".to_string() + encode(under_100).as_str(),
            }.as_str()
        }
    };
    digit_words + match order {
        0 => "".to_string(),
        _ => " ".to_string() + ORDERS[order - 1],
    }.as_str()
}

fn num_to_words(n: u64, order: usize) -> String {
    // recursively convert n * 10^order into english
    match n {
        0...999 => triplet_words(n, order),
        _ => thousand_over_words(n, order),
    }
}

fn thousand_over_words(n: u64, order: usize) -> String {
    num_to_words(n / 1000, order + 1) + match n % 1000 {
        0 => "".to_string(),
        digits => format!(" {}", num_to_words(digits, order)),
    }.as_str()
}

/// convert a u64 to its representation as a sequence of english words
/// run time: O(log(n)) -> each 3 digits triggers at most 1 recursive call and 1 base case call,
/// memory:   O(log(n)) -> necessary to store the resulting string
pub fn encode(n: u64) -> String {
    num_to_words(n, 0)
}

fn digit_to_word(digit: u8) -> &'static str {
    match digit {
        0 => "zero",
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        _ => panic!("invalid digit ".to_string() + digit.to_string().as_str()),
    }
}

fn teen_to_word(num: u8) -> &'static str {
    match num {
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        d => panic!(format!(
            "invalid teen number {}. Must be between [10, 19]",
            d
        )),
    }
}

fn under_hundred_to_words(num: u8) -> String {
    match num / 10 {
        0 => digit_to_word(num).to_string(),
        1 => teen_to_word(num).to_string(),
        tens @ 2...10 => {
            match tens {
                2 => "twenty",
                3 => "thirty",
                4 => "forty",
                5 => "fifty",
                6 => "sixty",
                7 => "seventy",
                8 => "eighty",
                9 => "ninety",
                d => panic!("{} is an invalid multiple of 10 under 100", d),
            }.to_string()
                + match num % 10 {
                    0 => "".to_string(),
                    ones => "-".to_string() + digit_to_word(ones),
                }.as_str()
        }
        d => panic!("{} is not under 100", d),
    }
}

fn under_thousand_to_words(num: u16) -> String {
    match num / 100 {
        0 => under_hundred_to_words(num as u8),
        hundreds @ 1...10 => {
            digit_to_word(hundreds as u8).to_string() + " hundred" + match num % 100 {
                0 => "".to_string(),
                under_hundred => {
                    " ".to_string() + under_hundred_to_words(under_hundred as u8).as_str()
                }
            }.as_str()
        }
        d => panic!("invalid hundreds number ".to_string() + d.to_string().as_str()),
    }
}

/// recursively convert a sequence of digit triplets into their corresponding
/// english words
/// The sequence of digits MUST represent a number > 0 since 0 is a special case
/// in this recursive function
fn thousand_groups_to_str(groups: &[u16]) -> String {
    let group = groups.last().unwrap();
    let triplet_index = groups.len();
    let triplet_quantifier_word = match group {
        0 => "".to_string(),
        _ => under_thousand_to_words(*group),
    };

    // get words for this triplet
    let triplet_words = if triplet_quantifier_word.is_empty() {
        "".to_string()
    } else {
        triplet_quantifier_word + match triplet_index {
            1 => "".to_string(),
            _ => {
                " ".to_string() + match triplet_index {
                    2 => "thousand",
                    3 => "million",
                    4 => "billion",
                    5 => "trillion",
                    6 => "quadrillion",
                    7 => "quintillion",
                    _ => panic!("cannot handle numbers outside u64"),
                }
            }
        }.as_str()
    };

    // recursively get the words for the lower triplets
    let remaining_words = match triplet_index {
        0 => panic!("should not be checking for remaining words when converting last triplet"),
        1 => "".to_string(),
        l => thousand_groups_to_str(&groups[..l - 1]),
    };

    // only include a space if the current triplet has words or there are more words
    triplet_words
        + if *group == 0 || remaining_words.is_empty() {
            ""
        } else {
            " "
        }
        + remaining_words.as_str()
}

/// convert a u64 to its representation as a sequence of english words
pub fn encode(n: u64) -> String {
    if n == 0 {
        return "zero".to_string();
    }

    let mut n_copy = n;
    let mut thousand_groups: Vec<u16> = Vec::new();
    while n_copy > 0 {
        thousand_groups.push((n_copy % 1000) as u16);
        n_copy /= 1000;
    }

    thousand_groups_to_str(&thousand_groups)
}

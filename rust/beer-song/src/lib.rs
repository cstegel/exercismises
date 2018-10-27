// "3 bottles of beer on the wall, 3 bottles of beer."
// "Take one down and pass it around, 2 bottles of beer on the wall."
//
// "2 bottles of beer on the wall, 2 bottles of beer."
// "Take one down and pass it around, 1 bottle of beer on the wall."
//
// "1 bottle of beer on the wall, 1 bottle of beer."
// "Take it down and pass it around, no more bottles of beer on the wall."
//
// "No more bottles of beer on the wall, no more bottles of beer."
// "Go to the store and buy some more, 99 bottles of beer on the wall."

fn capitalize(s: &str) -> String {
    let mut s_chars = s.chars();
    match s_chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().chain(s_chars).collect(),
    }
}

fn bottles_str(n: i32) -> String {
    match n {
        n if n > 1 => format!("{} bottles", n),
        1 => "1 bottle".to_string(),
        0 => "no more bottles".to_string(),
        _ => panic!("Cannot have {} bottles of beer", n),
    }
}

pub fn verse(n: i32) -> String {
    let b_str = bottles_str(n);
    let line1 = format!(
        "{} of beer on the wall, {} of beer.\n",
        capitalize(&b_str),
        b_str
    );

    let line2 = match n {
        n if n > 0 => {
            let take_str = match n {
                1 => "it",
                _ => "one",
            };
            format!(
                "Take {} down and pass it around, {} of beer on the wall.\n",
                take_str,
                bottles_str(n - 1)
            )
        }
        0 => "Go to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
        _ => panic!("invalid verse number {}", n),
    };

    line1 + line2.as_str()
}

pub fn sing(start: i32, end: i32) -> String {
    assert!(start > end, "start: {}, end: {}", start, end);
    assert!(end >= 0, "start: {}, end: {}", start, end);

    (end..=start)
        .rev()
        .map(|n| verse(n))
        .collect::<Vec<String>>()
        .join("\n")
}

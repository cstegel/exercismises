use integer_sqrt::IntegerSquareRoot;
use itertools::Itertools;
use std::iter;

pub fn encrypt(input: &str) -> String {
    // constraints:
    // only alphanumeric chars allowed
    // cols >= rows
    // cols - rows <= 1

    let lower_input = input.to_lowercase();
    let filtered = || lower_input.chars().filter(|ch| ch.is_alphanumeric());

    let n_chars = filtered().count();
    let rows = n_chars.integer_sqrt();

    // if n_chars is perf square then cols = rows, else it's rows + 1
    let cols = rows + (n_chars - rows.pow(2) > 0) as usize;
    let n_blank_chars = rows * cols - n_chars;

    // Soln 1: traverse padded string by column (jump by "col" stride along the string)
    let padded_filtered = filtered()
        // pad with blanks to fill the rect
        // so that each output column has correct blanks in them
        .chain(iter::repeat(' ').take(n_blank_chars))
        .collect::<String>();

    (0..cols)
        .map(|col| {
            padded_filtered
                .chars()
                .skip(col)
                .step_by(cols)
                .collect::<String>()
        })
        .join(" ")

    // Soln 2: traverse padded string by row while collecting to columns
    // filtered()
    //     // pad with blanks to fill the rect
    //     // so that each output column has correct blanks in them
    //     .chain(iter::repeat(' ').take(n_blank_chars))
    //     .enumerate()
    //     .map(|(i, ch)| (i % cols, ch))
    //     // .inspect(|(i, ch)| println!("{} {}", i, ch))
    //     // collect into columns
    //     .fold(vec![vec![]; cols], |mut col_vals, (col, ch)| {
    //         col_vals[col].push(ch);
    //         col_vals
    //     })
    //     .iter()
    //     .intersperse(&vec![' '])
    //     // single stream of chars that are in column-order
    //     .flatten()
    //     .collect::<String>()
}

use std::vec::Vec;

/// Find points where value is >= all in row and <= all in col
/// input: slice of rows
/// run time: O(rows * cols)
/// memory: O(rows + cols)
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.is_empty() || input[0].is_empty() {
        return Vec::new();
    }

    let n_rows = input.len();
    let n_cols = input[0].len();

    let row_maxes: Vec<u64> = input
        .iter()
        .map(|row| *row.iter().max().expect("All rows must be same size"))
        .collect();
    let col_mins: Vec<u64> = (0..n_cols)
        .map(|c| input.iter().map(|row| row[c]).min().unwrap())
        .collect();

    (0..n_rows)
        .flat_map(|r| (0..n_cols).map(move |c| (r, c)))
        .filter(|(r, c)| {
            let v = input[*r][*c];
            v == row_maxes[*r] && v == col_mins[*c]
        }).collect()
}

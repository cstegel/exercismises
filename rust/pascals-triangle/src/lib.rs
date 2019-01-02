use std::iter;

pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

fn create_next_row(prev: &Vec<u32>) -> Vec<u32> {
    let mut row: Vec<u32> = iter::repeat(1).take(prev.len() + 1).collect();

    let splice_range = 1..row.len() - 1;
    row.splice(
        splice_range.clone(),
        splice_range.map(|i| prev[i - 1] + prev[i]),
    );
    row
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = iter::repeat(Vec::new()).take(row_count as usize).collect();
        if row_count > 0 {
            rows[0] = vec![1];
        }
        for r in 1..row_count as usize {
            rows[r] = create_next_row(&rows[r - 1]);
        }

        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.iter().cloned().collect()
    }
}

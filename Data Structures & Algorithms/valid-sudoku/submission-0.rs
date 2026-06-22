use std::collections::{HashSet, HashMap};

static SUDOKU_LEN: usize = 9;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut rows = vec![HashSet::new(); SUDOKU_LEN];
        let mut cols = vec![HashSet::new(); SUDOKU_LEN];
        let mut sqs: HashMap<(usize, usize), HashSet<char>> = HashMap::new();

        for (row_idx, row) in board.into_iter().enumerate() {
            for (col_idx, entry) in row.into_iter().enumerate() {
                if entry == '.' {
                    continue;
                }
                if rows[row_idx].insert(entry) == false {
                    println!("rows problem row: {row_idx}, col: {col_idx}, entry: {entry}");
                    return false;
                }
                if cols[col_idx].insert(entry) == false {
                    println!("cols problem");
                    return false;
                }

                // Determine which square 0..9 the entry is is
                let sq_row: usize = (row_idx / 3) * 3;
                let sq_col: usize = (col_idx / 3) * 3;

                if sqs.entry((sq_row, sq_col)).or_default().insert(entry) == false {
                    println!("SQ problem");
                    return false;
                }
            }
        }

        true
    }
}

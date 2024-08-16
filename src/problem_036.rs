struct Solution;
use std::collections::HashSet;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut set: HashSet<String> = HashSet::new();

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                let val = board[i][j];
                if val == '.' {
                    continue;
                }
                let row = format!("row:{i} value{val}");
                let col = format!("col:{j} value{val}");
                let box_x = (i / 3) as i32;
                let box_y = (j / 3) as i32;
                let box_x_y = format!("box:{box_x}{box_y} value{val}");

                if set.contains(&row) || set.contains(&col) || set.contains(&box_x_y) {
                    return false;
                }

                set.insert(row);
                set.insert(col);
                set.insert(box_x_y);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        let input = vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let output = true;
        assert_eq!(Solution::is_valid_sudoku(input), output);
    }

    #[test]
    fn test_case_2() {
        let input = vec![
            vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
        ];
        let output = false;
        assert_eq!(Solution::is_valid_sudoku(input), output);
    }
}

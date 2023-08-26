impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        use std::collections::HashSet;

        let mut row = HashSet::new();
        let mut col = HashSet::new();

        let m = matrix.len();
        let n = matrix[0].len();

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    row.insert(i);
                    col.insert(j);
                }
            }
        }

        for i in row.iter() {
            for j in 0..n {
                matrix[*i][j] = 0;
            }
        }

        for i in col.iter() {
            for j in 0..m {
                matrix[j][*i] = 0;
            }
        }

    }
}
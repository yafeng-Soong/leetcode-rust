use crate::solution::Solution;

use std::cmp::Ordering;
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let (m, n) = (matrix.len(), matrix[0].len());
        let (mut x, mut y) = (0, n as i32 - 1);
        while x < m && y >= 0 {
            match matrix[x][y as usize].cmp(&target) {
                Ordering::Less => x += 1,
                Ordering::Equal => return true,
                Ordering::Greater => y -= 1,
            }
        }

        false
    }
}

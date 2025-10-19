#![allow(dead_code)]

pub struct Solution {}
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len() as i32;
        let n = matrix[0].len() as i32;
        let mut x = 0_i32;
        let mut y = n as i32 - 1;
        while x < m && y >= 0 {
            if matrix[x as usize][y as usize] == target {
                return true;
            }

            if matrix[x as usize][y as usize] < target {
                x += 1;
            } else {
                y -= 1;
            }
        }

        false
    }
}

#[test]
fn test() {
    let matrix = vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]];
    assert_eq!(Solution::search_matrix(matrix, 3), true);
}

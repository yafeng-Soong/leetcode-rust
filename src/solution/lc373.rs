use crate::solution::Solution;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl Solution {
    pub fn k_smallest_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> Vec<Vec<i32>> {
        let mut heap = BinaryHeap::new();
        for num1 in nums1 {
            let tmp = Pair {
                a: num1,
                b: nums2[0],
            };
            if heap.len() >= k as usize && tmp.cmp(heap.peek().unwrap()) == Ordering::Greater {
                break;
            }

            for &num2 in &nums2 {
                let pair = Pair { a: num1, b: num2 };

                if heap.len() < k as usize {
                    heap.push(pair);
                    continue;
                }

                match pair.cmp(heap.peek().unwrap()) {
                    Ordering::Less => {
                        heap.pop();
                        heap.push(pair);
                    }
                    _ => break,
                }
            }
        }

        heap.into_vec().iter().map(|x| vec![x.a, x.b]).collect()
    }
}

struct Pair {
    a: i32,
    b: i32,
}

impl Eq for Pair {}
impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        let sum = self.a + self.b;
        sum.cmp(&(other.a + other.b))
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test_k_smallest_pairs() {
    struct Test {
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        k: i32,
        expected: Vec<Vec<i32>>,
    }

    let tests = vec![
        Test {
            nums1: vec![1, 7, 11],
            nums2: vec![2, 4, 6],
            k: 3,
            expected: vec![vec![1, 2], vec![1, 4], vec![1, 6]],
        },
        Test {
            nums1: vec![1, 1, 2],
            nums2: vec![1, 2, 3],
            k: 2,
            expected: vec![vec![1, 1], vec![1, 1]],
        },
    ];

    for t in tests {
        let mut res = Solution::k_smallest_pairs(t.nums1, t.nums2, t.k);
        res.sort();
        assert_eq!(res, t.expected);
    }
}
